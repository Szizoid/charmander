use std::process::Command;

use crate::config::OutputMethod;

pub fn insert(symbol: &str, method: &OutputMethod) -> Result<(), Box<dyn std::error::Error>> {
    match method {
        OutputMethod::Wtype => {
            Command::new("wtype").arg(symbol).status()?;
        }
        OutputMethod::WlCopy => {
            Command::new("wl-copy").arg(symbol).status()?;
        }
        OutputMethod::Ydotool => {
            Command::new("ydotool").arg("type").arg(symbol).status()?;
        }
    }
    Ok(())
}
