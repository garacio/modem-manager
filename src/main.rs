mod serial_tools;
mod modem_tools;
mod display_tools;
mod tests;

use std::io;
use display_tools::tui::start_tui;

fn main() -> Result<(), io::Error> {
    let baud_rate: u32 = 115_200;

    start_tui(baud_rate)?;

    Ok(())
}
