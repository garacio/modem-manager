mod serial_tools;
mod modem_tools;
mod display_tools;
mod tests;

use std::process::exit;
use crate::serial_tools::serial_reader::{get_port};
use crate::modem_tools::modem::{get_modem_info, get_modem_info_string};
use std::thread;
use std::time::Duration;

fn main() {
    let baud_rate :u32 = 115_200;
    match get_port() {
        Ok(port_name) => {
            if port_name.len() == 0 {
                eprintln!("There is no available ports");
                exit(0)
            }
            loop {

                let modem_info_string = get_modem_info_string(&port_name, baud_rate).unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    exit(1);
                });

                let modem_info = get_modem_info(modem_info_string).unwrap();


                print!("\x1B[2J\x1B[1;1H");
                modem_info.display_modem_info();
                modem_info.display_signal_info();
                modem_info.display_carrier_info();

                thread::sleep(Duration::from_secs(3));
            }
        },
        Err(e) => panic!("Cannot list ports {}", e)
    }
}
