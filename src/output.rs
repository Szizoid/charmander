use std::process::Command;

// Called after a short delay (see main.rs) to let the Wayland compositor
// release the exclusive keyboard grab held by the layer shell window.
// wtype injects into whichever window currently has focus — that must be
// the previously active window, not ours.
pub fn insert(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("wtype").arg(symbol).status()?;
    Ok(())
}
