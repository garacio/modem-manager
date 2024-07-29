use std::time::Duration;
use std::io::{self, Read, Write};
use std::thread;
use regex::Regex;
use serialport::{available_ports, SerialPortInfo, SerialPortType, SerialPort};

fn list_ports() -> Result<Vec<SerialPortInfo>, serialport::Error> {
    match available_ports() {
        Ok(ports) => {
            println!("Available serial ports:");
            for (index, port) in ports.iter().enumerate() {
                println!("{}. Port: {}", index + 1, port.port_name);
                match &port.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("  Type: USB");
                        println!("  VID: {:04x}, PID: {:04x}", info.vid, info.pid);
                        println!("  Manufacturer: {:?}", info.manufacturer);
                        println!("  Product: {:?}", info.product);
                        println!("  Serial Number: {:?}", info.serial_number);
                    }
                    SerialPortType::BluetoothPort => {
                        println!("  Type: Bluetooth");
                    }
                    SerialPortType::PciPort => {
                        println!("  Type: PCI");
                    }
                    SerialPortType::Unknown => {
                        println!("  Type: Unknown");
                    }
                }
            }
            Ok(ports)
        }
        Err(e) => {
            eprintln!("Error listing serial ports: {}", e);
            Err(e)
        }
    }
}

pub fn get_port() -> Result<String, io::Error> {
    let ports = list_ports()?;

    loop {
        println!("Please select a port (1-{}):", ports.len());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num > 0 && num <= ports.len() {
                    return Ok(ports[num - 1].port_name.clone())
                }
            }
            _ => {
                eprintln!("Invalid input");
            }
        };
    }
}

