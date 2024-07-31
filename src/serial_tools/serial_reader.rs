use serialport::{available_ports, SerialPortInfo};

pub fn list_ports() -> Result<Vec<SerialPortInfo>, serialport::Error> {
    match available_ports() {
        Ok(ports) => {
            Ok(ports)
        }
        Err(e) => {
            eprintln!("Error listing serial ports: {}", e);
            Err(e)
        }
    }
}
