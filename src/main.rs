mod config;
mod history;
mod output;
mod search;
mod ui;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::gio::prelude::{ApplicationExt, ApplicationExtManual};

fn main() {
    let home = std::env::var("HOME").expect("Переменная Home не задана");

    let config_path = PathBuf::from(format!("{}/.config/charmander/config.toml", home));
    let history_path = PathBuf::from(format!("{}/.config/charmander/history.toml", home));

    std::fs::create_dir_all(history_path.parent().unwrap()).unwrap();

    config::create_default_if_missing(&config_path)
        .expect("Не удалось создать конфиг по умолчанию");
    let cfg = config::load(&config_path).expect("Не удалось загрузить конфиг");
    let history = Rc::new(RefCell::new(history::History::load(&history_path)));

    let app = gtk4::Application::builder()
        .application_id("io.github.szizoid.charmander")
        .build();

    let characters = cfg.characters;
    let max_results = cfg.settings.max_results;
    let sel_indicator = cfg.settings.selection_indicator;
    let no_sel_indicator = cfg.settings.no_selection_indicator;
    let pending_symbol: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    app.connect_activate({
        let pending_symbol = pending_symbol.clone();
        move |app| {
            ui::window::build(
                app,
                characters.clone(),
                max_results,
                sel_indicator.clone(),
                no_sel_indicator.clone(),
                history.clone(),
                history_path.clone(),
                pending_symbol.clone(),
            );
        }
    });

    app.run();

    if let Some(symbol) = pending_symbol.borrow_mut().take() {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = output::insert(&symbol);
    }
}
