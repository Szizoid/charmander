use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Label, ListBox, Orientation, ScrolledWindow,
    SearchEntry,
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::config::{CharacterEntry, OutputMethod};
use crate::history::History;
use crate::{output, search};

pub fn build(
    app: &Application,
    characters: Vec<CharacterEntry>,
    output_method: OutputMethod,
    history: Rc<RefCell<History>>,
    history_path: PathBuf,
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
    let initial = search::search("", &characters, &history_ref);
    drop(history_ref);
    populate_list(&list_box, &initial);

    let chars_for_search = characters.clone();
    let list_box_for_search = list_box.clone();
    let history_for_search = history.clone();

    search_entry.connect_search_changed(move |entry| {
        let query = entry.text();
        let h = history_for_search.borrow();
        let resuls = search::search(query.as_str(), &chars_for_search, &h);
        drop(h);
        populate_list(&list_box_for_search, &resuls);
    });

    let window_for_activation = window.clone();
    let history_for_activation = history.clone();

    list_box.connect_row_activated(move |_, row| {
        if let Some(child) = row.child() {
            let symbol = child.widget_name();
            {
                let mut h = history_for_activation.borrow_mut();
                h.increment(symbol.as_str());
                let _ = h.save(&history_path);
            }
            let _ = output::insert(symbol.as_str(), &output_method);
        }
        window_for_activation.close();
    });

    let key_controller = gtk4::EventControllerKey::new();
    let window_for_escape = window.clone();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            window_for_escape.close();
            return gtk4::glib::Propagation::Stop;
        }
        gtk4::glib::Propagation::Proceed
    });
    window.add_controller(key_controller);

    let provider = gtk4::CssProvider::new();
    provider.load_from_string("window { background-color: @window_bg_color; }");
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // Enter в поиске - активирует первый результат
    let list_box_for_enter = list_box.clone();
    search_entry.connect_activate(move |_| {
        if let Some(row) = list_box_for_enter.row_at_index(0) {
            row.activate();
        }
    });

    // ↓ в поиске - переход в список
    let search_key = gtk4::EventControllerKey::new();
    let list_box_for_down = list_box.clone();
    search_key.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Down {
            if let Some(row) = list_box_for_down.row_at_index(0) {
                list_box_for_down.select_row(Some(&row));
                list_box_for_down.grab_focus();
            }
            return gtk4::glib::Propagation::Stop;
        }
        gtk4::glib::Propagation::Proceed
    });
    search_entry.add_controller(search_key);

    // ↑ в списке на первой строке — возврат в поиск
    let list_key = gtk4::EventControllerKey::new();
    let list_box_for_nav = list_box.clone();
    let search_entry_for_up = search_entry.clone();

    list_key.connect_key_pressed(move |_, key, _, _| match key {
        gtk4::gdk::Key::Up => {
            if let Some(row) = list_box_for_nav.selected_row() {
                if row.index() == 0 {
                    search_entry_for_up.grab_focus();
                } else if let Some(prev) = list_box_for_nav.row_at_index(row.index() - 1) {
                    list_box_for_nav.select_row(Some(&prev));
                }
            }
            gtk4::glib::Propagation::Stop
        }
        gtk4::gdk::Key::Down => {
            if let Some(row) = list_box_for_nav.selected_row() {
                if let Some(next) = list_box_for_nav.row_at_index(row.index() + 1) {
                    list_box_for_nav.select_row(Some(&next));
                }
            }
            gtk4::glib::Propagation::Stop
        }
        gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
            if let Some(row) = list_box_for_nav.selected_row() {
                row.activate();
            }
            gtk4::glib::Propagation::Stop
        }
        _ => gtk4::glib::Propagation::Proceed,
    });
    list_box.add_controller(list_key);

    window.present();

    search_entry.grab_focus();
}

fn populate_list(list_box: &ListBox, entries: &[CharacterEntry]) {
    while let Some(row) = list_box.first_child() {
        list_box.remove(&row);
    }

    for entry in entries {
        let label = Label::builder()
            .label(format!("{}  {}", entry.symbol, entry.name))
            .xalign(0.0)
            .build();
        label.set_widget_name(&entry.symbol);
        list_box.append(&label);
    }
}
