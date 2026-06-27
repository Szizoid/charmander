mod config;
mod history;
mod output;
mod search;
mod ui;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::gio::prelude::{ApplicationExt, ApplicationExtManual};

fn print_help() {
    println!(
        "charmander {} — Wayland popup character picker

USAGE:
    charmander                      Open the popup window (bind this to a hotkey)

CONFIG:
    charmander config path          Print the config file path
    charmander config delete        Delete the config file
    charmander config default       Backup current config and restore defaults

HISTORY:
    charmander history path         Print the history file path
    charmander history delete       Delete the history file

INFO:
    charmander -v, --version        Print version
    charmander -h, --help, help     Print this help message

FILES:
    Config:   ~/.config/charmander/config.toml
    History:  ~/.local/share/charmander/history.toml",
        env!("CARGO_PKG_VERSION")
    );
}

fn main() {
    let home = std::env::var("HOME").expect("HOME variable is not set");
    let config_path = PathBuf::from(format!("{}/.config/charmander/config.toml", home));
    let history_path = PathBuf::from(format!("{}/.local/share/charmander/history.toml", home));

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("-v") | Some("--version") => {
            println!("charmander {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        Some("-h") | Some("--help") | Some("help") => {
            print_help();
            return;
        }
        Some("config") => {
            match args.get(2).map(|s| s.as_str()) {
                Some("path") => {
                    println!("{}", config_path.display());
                }
                Some("delete") => {
                    if config_path.exists() {
                        std::fs::remove_file(&config_path).expect("Failed to delete config");
                        println!("Deleted {}", config_path.display());
                    } else {
                        println!("Config file does not exist: {}", config_path.display());
                    }
                }
                Some("default") => {
                    config::restore_default(&config_path).expect("Failed to restore default config");
                }
                _ => {
                    eprintln!("Usage: charmander config <path|delete|default>");
                    std::process::exit(1);
                }
            }
            return;
        }
        Some("history") => {
            match args.get(2).map(|s| s.as_str()) {
                Some("path") => {
                    println!("{}", history_path.display());
                }
                Some("delete") => {
                    if history_path.exists() {
                        std::fs::remove_file(&history_path).expect("Failed to delete history");
                        println!("Deleted {}", history_path.display());
                    } else {
                        println!("History file does not exist: {}", history_path.display());
                    }
                }
                _ => {
                    eprintln!("Usage: charmander history <path|delete>");
                    std::process::exit(1);
                }
            }
            return;
        }
        Some(unknown) => {
            eprintln!("Unknown command: '{}'", unknown);
            eprintln!("Run 'charmander --help' for usage.");
            std::process::exit(1);
        }
        None => {}
    }

    // No subcommand — open the GTK popup window.

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
