mod serial_tools;
mod modem_tools;
mod display_tools;
mod tests;

use std::io;
use display_tools::tui::app::run_app;

fn main() -> Result<(), io::Error> {
    run_app()?;

    Ok(())
}
