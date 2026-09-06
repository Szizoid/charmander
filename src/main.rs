use charmander::config;
use charmander::ui::window::{Charmander, namespace, update, view};
use iced_layershell::Settings;
use iced_layershell::build_pattern::application;
use iced_layershell::settings::{LayerShellSettings, StartMode};

fn main() -> Result<(), iced_layershell::Error> {
    let _ = application(Charmander::default, namespace, update, view)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: config::WINDOW_SIZE,
                exclusive_zone: config::EXCLUSIVE_ZONE,
                start_mode: StartMode::Active,
                ..Default::default()
            },
            ..Default::default()
        })
        .run();
    Ok(())
}
