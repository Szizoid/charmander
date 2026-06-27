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
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "-v" || a == "--version") {
        println!("charmander {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let home = std::env::var("HOME").expect("HOME variable is not set");

    let config_path = PathBuf::from(format!("{}/.config/charmander/config.toml", home));
    let history_path = PathBuf::from(format!("{}/.local/share/charmander/history.toml", home));

    std::fs::create_dir_all(history_path.parent().unwrap()).unwrap();

    config::create_default_if_missing(&config_path)
        .expect("Failed to create default config");
    let cfg = config::load(&config_path).expect("Failed to load config");
    let history = Rc::new(RefCell::new(history::History::load(&history_path)));

    let app = gtk4::Application::builder()
        .application_id("io.github.szizoid.charmander")
        .build();

    let characters = cfg.characters;
    let max_results = cfg.settings.max_results;
    let sel_indicator = cfg.settings.selection_indicator;
    let no_sel_indicator = cfg.settings.no_selection_indicator;

    // Symbol is stored here by the GTK callback instead of being typed immediately.
    // We must call wtype only after app.run() returns, because the layer shell window
    // holds an exclusive keyboard grab until it's fully destroyed. wtype would otherwise
    // inject keystrokes into our own window rather than the previously focused one.
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

    // Brief pause so the compositor can process the window close and reassign focus
    // before wtype tries to inject keystrokes.
    if let Some(symbol) = pending_symbol.borrow_mut().take() {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = output::insert(&symbol);
    }
}
