use std::process::Command;

pub fn insert(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("wtype").arg(symbol).status()?;
    Ok(())
}
