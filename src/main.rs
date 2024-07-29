mod serial_tools;
mod modem_tools;
mod display_tools;

use crate::serial_tools::serial_reader::{get_port};
use crate::modem_tools::modem::{get_modem_info, get_sim_info, get_signal_info};
use crate::display_tools::bars::get_bar;
use std::thread;
use std::time::Duration;

// === Status ===
// Operator:         mt:s (LTE)
// Distance:         0,234km
// Signal:             35%   [███░░░░░]
// RSSI:              -61dBm [█████░░░]
// SINR:                4dB  [███░░░░░]
// RSRP:              -92dBm [███░░░░░]
// RSRQ:              -11dB  [█████░░░]
// Band:             B3@20MHz B3@10MHz
// EARFCN:           1500

fn main() {
    let baud_rate :u32 = 115_200;
    match get_port() {
        Ok(port_name) => {
            loop {
                let modem_info = get_modem_info(&port_name, baud_rate).unwrap();

                let sim_info =  get_sim_info(&port_name, baud_rate).unwrap();
                // sim_info.print();

                let signal_info = get_signal_info(&port_name, baud_rate).unwrap();

                let status_string: String = format!("=== Status ===\n\
                Operator:             {} ({})\n\
                Distance:             {}\n\
                Signal:               {:2}%    [{}]\n\
                RSSI:                 {:2}dBm  [{}]\n\
                SINR:                 {:2}dB  [{}]\n\
                RSRP:                 {:2}    [{}]\n\
                RSRQ:                 {:2}    [{}]\n\
                Band:                 {}\n\
                EARFCN:               {}\n\
                ",
                                                    sim_info.operator, sim_info.mode,
                                                    "",
                                                    signal_info.csq_perc, get_bar(signal_info.csq_perc, 0, 100),
                                                    signal_info.rssi, get_bar(signal_info.rssi, -120, -25),
                                                    signal_info.sinr, get_bar(signal_info.sinr, -10, 30),
                                                    signal_info.rsrp, get_bar(signal_info.rsrp, -120, -50),
                                                    signal_info.rsrq, get_bar(signal_info.rsrq, -25, -1),
                                                    signal_info.band,
                                                    ""
                );
                print!("\x1B[2J\x1B[1;1H");
                modem_info.print();
                println!("{}", status_string);

                match get_signal_info(&port_name, baud_rate) {
                    Ok(signal_info) => signal_info.display_carrier_info(),
                    Err(e) => eprintln!("Error: {}", e)
                }
                thread::sleep(Duration::from_secs(5));
            }
        },
        Err(e) => panic!("Cannot list ports {}", e)
    }
}
