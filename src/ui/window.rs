use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Label, ListBox, Orientation, ScrolledWindow,
    SearchEntry,
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::config::CharacterEntry;
use crate::history::History;
use crate::search;

pub fn build(
    app: &Application,
    characters: Vec<CharacterEntry>,
    max_results: usize,
    sel_indicator: String,
    no_sel_indicator: String,
    history: Rc<RefCell<History>>,
    history_path: PathBuf,
    pending_symbol: Rc<RefCell<Option<String>>>,
) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(400)
        .title("Charmander")
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    let vbox = GtkBox::new(Orientation::Vertical, 8);
    let search_entry = SearchEntry::new();
    let scrolled = ScrolledWindow::builder().vexpand(true).build();
    let list_box = ListBox::new();

    scrolled.set_child(Some(&list_box));
    vbox.append(&search_entry);
    vbox.append(&scrolled);
    window.set_child(Some(&vbox));

    let history_ref = history.borrow();
    let mut initial = search::search("", &characters, &history_ref);
    drop(history_ref);
    if max_results > 0 {
        initial.truncate(max_results);
    }
    populate_list(&list_box, &initial, &no_sel_indicator);

    let in_list = Rc::new(Cell::new(false));

    let chars_for_search = characters.clone();
    let list_box_for_search = list_box.clone();
    let history_for_search = history.clone();
    let in_list_for_search = in_list.clone();
    let no_sel_for_search = no_sel_indicator.clone();

    search_entry.connect_search_changed(move |entry| {
        in_list_for_search.set(false);
        list_box_for_search.unselect_all();
        let query = entry.text();
        let h = history_for_search.borrow();
        let mut results = search::search(query.as_str(), &chars_for_search, &h);
        drop(h);
        if max_results > 0 {
            results.truncate(max_results);
        }
        populate_list(&list_box_for_search, &results, &no_sel_for_search);
    });

    let window_for_activation = window.clone();
    let history_for_activation = history.clone();

    list_box.connect_row_activated(move |_, row| {
        if let Some(child) = row.child() {
            let symbol = child.widget_name().to_string();
            {
                let mut h = history_for_activation.borrow_mut();
                h.increment(&symbol);
                let _ = h.save(&history_path);
            }
            *pending_symbol.borrow_mut() = Some(symbol);
        }
        window_for_activation.close();
    });

    let provider = gtk4::CssProvider::new();
    provider.load_from_string("window { background-color: @window_bg_color; }");
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let list_box_for_enter = list_box.clone();
    search_entry.connect_activate(move |_| {
        if let Some(row) = list_box_for_enter.row_at_index(0) {
            row.activate();
        }
    });

    let key_controller = gtk4::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let window_for_key = window.clone();
    let list_box_for_key = list_box.clone();
    let search_entry_for_key = search_entry.clone();
    let in_list_for_key = in_list.clone();
    let sel_for_key = sel_indicator.clone();
    let no_sel_for_key = no_sel_indicator.clone();

    key_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gtk4::gdk::Key::Escape => {
                window_for_key.close();
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Down => {
                if !in_list_for_key.get() {
                    if let Some(row) = list_box_for_key.row_at_index(0) {
                        list_box_for_key.select_row(Some(&row));
                        set_row_indicator(&row, true, &sel_for_key, &no_sel_for_key);
                        in_list_for_key.set(true);
                    }
                } else if let Some(cur) = list_box_for_key.selected_row() {
                    if let Some(next) = list_box_for_key.row_at_index(cur.index() + 1) {
                        set_row_indicator(&cur, false, &sel_for_key, &no_sel_for_key);
                        list_box_for_key.select_row(Some(&next));
                        set_row_indicator(&next, true, &sel_for_key, &no_sel_for_key);
                    }
                }
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Up => {
                if in_list_for_key.get() {
                    if let Some(cur) = list_box_for_key.selected_row() {
                        set_row_indicator(&cur, false, &sel_for_key, &no_sel_for_key);
                        if cur.index() == 0 {
                            list_box_for_key.unselect_all();
                            in_list_for_key.set(false);
                            search_entry_for_key.grab_focus();
                        } else if let Some(prev) = list_box_for_key.row_at_index(cur.index() - 1) {
                            list_box_for_key.select_row(Some(&prev));
                            set_row_indicator(&prev, true, &sel_for_key, &no_sel_for_key);
                        }
                    }
                }
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                if in_list_for_key.get() {
                    if let Some(row) = list_box_for_key.selected_row() {
                        row.activate();
                    }
                    gtk4::glib::Propagation::Stop
                } else {
                    gtk4::glib::Propagation::Proceed
                }
            }
            _ => gtk4::glib::Propagation::Proceed,
        }
    });
    window.add_controller(key_controller);

    window.present();

    search_entry.grab_focus();
}

fn populate_list(list_box: &ListBox, entries: &[CharacterEntry], no_sel: &str) {
    while let Some(row) = list_box.first_child() {
        list_box.remove(&row);
    }

    for entry in entries {
        let label = Label::builder()
            .label(format!("{}{}  {}", no_sel, entry.symbol, entry.name))
            .xalign(0.0)
            .build();
        label.set_widget_name(&entry.symbol);
        list_box.append(&label);
    }
}

fn set_row_indicator(row: &gtk4::ListBoxRow, active: bool, sel: &str, no_sel: &str) {
    if let Some(label) = row.child().and_then(|w| w.downcast::<Label>().ok()) {
        let text = label.text();
        let (skip, new_prefix) = if active {
            (no_sel.chars().count(), sel)
        } else {
            (sel.chars().count(), no_sel)
        };
        let rest: String = text.chars().skip(skip).collect();
        label.set_text(&format!("{}{}", new_prefix, rest));
    }
}
