use std::io::{self, Read, Write};
use std::time::Duration;
use std::error::Error;
use regex::Regex;
use serialport::SerialPort;
use crate::modem_tools::converters::{get_band_lte, hex_to_decimal, parse_bandwidth, parse_bandwidth_str, convert_rsrp_to_rssi};
use crate::modem_tools::types::{ModemInfo, SimInfo, SignalInfo};

// AT+XCCINFO?; +XLEC?; +XMCI=1
// +XCCINFO: 0,220,03,"00009C03",3,103,"FFFF",1,"FF","4E91",0,0,0,0,0,0,0,0
//
// +XLEC: 0,2,5,3,BAND_LTE_3
//
// +XMCI: 4,220,03,"0x4E91","0x00009C03","0x0062","0x000005DC","0x00004C2C","0xFFFFFFFF",50,14,0,"0x00000004","0x00000000"
//
// +XMCI: 5,000,000,"0xFFFE","0xFFFFFFFF","0x006A","0x000005DC","0xFFFFFFFF","0xFFFFFFFF",47,6,255,"0x7FFFFFFF","0x00000000"
//
// +XMCI: 5,000,000,"0xFFFE","0xFFFFFFFF","0x0061","0x000005DC","0xFFFFFFFF","0xFFFFFFFF",45,8,255,"0x7FFFFFFF","0x00000000"
//
// OK

fn send_at_command(port: &mut dyn SerialPort, command: &str) -> Result<String, io::Error> {
    port.write_all(command.as_bytes())?;
    port.write_all(b"\r")?;
    port.flush()?;
    let mut response = String::new();
    let mut serial_buf: Vec<u8> = vec![0; 200];
    let timeout = std::time::Duration::from_millis(300);
    let start = std::time::Instant::now();

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                response.push_str(&String::from_utf8_lossy(&serial_buf[..t]));
                if response.contains("OK") || response.contains("ERROR") {
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                if start.elapsed() > timeout {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(response)
}

pub fn get_modem_info(port_name: &str, baud_rate: u32) -> Result<ModemInfo, Box<dyn Error>> {
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_secs(5))
        .open()?;

    let mut modem_info: ModemInfo = Default::default();
    port.write_data_terminal_ready(true)?; // Включение DTR

    // Modem Manufacturer
    let cgmi_response = send_at_command(&mut *port, "AT+CGMI?")?;
    let re_cgmi = Regex::new(r#"\+CGMI: "([^"]+)""#)?;
    if let Some(caps) = re_cgmi.captures(&cgmi_response) {
        let manufacturer = caps.get(1).unwrap().as_str();
        modem_info.manufacturer = manufacturer.parse().unwrap();
    }

    // Modem model
    let fmm_response = send_at_command(&mut *port, "AT+FMM?")?;
    let re_fmm = Regex::new(r#"\+FMM: "([^"]+)"(?:,"([^"]+)")?"#)?;
    if let Some(caps) = re_fmm.captures(&fmm_response) {
        let model = caps.get(1).unwrap().as_str();
        modem_info.model = model.parse().unwrap();
    }

    // Firmware Version
    let gtpkgver_response = send_at_command(&mut *port, "AT+GTPKGVER?")?;
    let re_gtpkgver = Regex::new(r#"\+GTPKGVER: "([^"]+)""#)?;
    if let Some(caps) = re_gtpkgver.captures(&gtpkgver_response) {
        let firmware_ver = caps.get(1).unwrap().as_str();
        modem_info.fw_version = firmware_ver.parse().unwrap();
    }

    // Serial Number
    let cfsn_response = send_at_command(&mut *port, "AT+CFSN?")?;
    let re_cfsn = Regex::new(r#"\+CFSN: "([^"]+)""#)?;
    if let Some(caps) = re_cfsn.captures(&cfsn_response) {
        let serial_number = caps.get(1).unwrap().as_str();
        modem_info.serial_number = serial_number.parse().unwrap();
    }

    // IMEI
    let cgsn_response = send_at_command(&mut *port, "AT+CGSN?")?;
    let re_cgsn = Regex::new(r#"\+CGSN: "([^"]+)""#)?;
    if let Some(caps) = re_cgsn.captures(&cgsn_response) {
        let imei = caps.get(1).unwrap().as_str();
        modem_info.imei = imei.parse().unwrap();
    }
    Ok(modem_info)
}

pub fn get_sim_info(port_name: &str, baud_rate: u32) -> Result<SimInfo, Box<dyn Error>> {
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_secs(5))
        .open()?;

    let mut sim_info: SimInfo = Default::default();
    port.write_data_terminal_ready(true)?; // Включение DTR

    sim_info.ip = "---".parse().unwrap();
    sim_info.mask = "---".parse().unwrap();
    sim_info.gw = "---".parse().unwrap();

     // SIM Information
    let sim_response = send_at_command(&mut *port, "AT+CIMI?; AT+CCID?")?;

    // IMSI
    let re_cimi = Regex::new(r#"\+CIMI: "([^"]+)""#)?;
    if let Some(caps) = re_cimi.captures(&sim_response) {
        let imsi = caps.get(1).unwrap().as_str();
        sim_info.imsi = imsi.parse().unwrap();
    }

    // ICCID
    let re_ccid = Regex::new(r#"\+CCID: "([^"]+)""#)?;
    if let Some(caps) = re_ccid.captures(&sim_response) {
        let ccid = caps.get(1).unwrap().as_str();
        sim_info.iccid = ccid.parse().unwrap();
    }

    // Operator and connection mode
    let cops_response = send_at_command(&mut *port, "AT+COPS?")?;
    let re_cops = Regex::new(r#"\+COPS: (\d),(\d),"([^"]*)",(\d)"#)?;
    if let Some(caps) = re_cops.captures(&cops_response) {
        let operator = caps.get(3).unwrap().as_str();
        let tech = caps.get(4).unwrap().as_str();
        let mode = match tech {
            "0" => "EDGE",
            "2" => "UMTS",
            "3" => "LTE",
            "4" => "HSDPA",
            "5" => "HSUPA",
            "6" => "HSPA",
            "7" => "LTE",
            _ => "Unknown",
        };
        sim_info.operator = operator.parse().unwrap();
        sim_info.mode = mode.parse().unwrap();
    }
    Ok(sim_info)
}

pub fn get_signal_info(port_name: &str, baud_rate: u32) -> Result<SignalInfo, Box<dyn std::error::Error>> {
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_secs(5))
        .open()?;

    let mut signal_info: SignalInfo = Default::default();
    port.write_data_terminal_ready(true)?; // Включение DTR

    // Signal quality
    let csq_response = send_at_command(&mut *port, "AT+CSQ?")?;
    let re_csq = Regex::new(r#"\+CSQ: (\d+),(\d+)"#)?;
    if let Some(caps) = re_csq.captures(&csq_response) {
        signal_info.csq = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
        signal_info.csq_perc = if signal_info.csq >= 0 && signal_info.csq <= 31 {
            signal_info.csq * 100 / 31
        } else {
            0
        };
    }

    let xmci_response = send_at_command(&mut *port, "AT+XMCI=1")?;

    let re_xmci = Regex::new(r#"\+XMCI: (?P<type>4),(?P<mcc>\d+),(?P<mnc>\d+),"(?P<tac>[^"]*)","(?P<ci_x>[^"]*)","(?P<pci_x>[^"]*)","(?P<dluarfnc_x>[^"]*)","(?P<earfcn_ul>[^"]*)","(?P<pathloss_lte>[^"]*)",(?P<rsrp>\d+),(?P<rsrq>\d+),(?P<sinr>\d+),"(?P<timing_advance>[^"]*)","(?P<cqi>[^"]*)""#).unwrap();
    let caps = re_xmci.captures(&xmci_response).unwrap();


    // let caps = re_xmci.captures("").unwrap();
    signal_info.rsrp = caps.name("rsrp").unwrap().as_str().parse::<i32>().unwrap_or(0) - 141;
    signal_info.rsrq = caps.name("rsrq").unwrap().as_str().parse::<i32>().unwrap_or(0) / 2 - 20;
    signal_info.sinr = caps.name("sinr").unwrap().as_str().parse::<i32>().unwrap_or(0) / 2;

    // signal_info.rssi = convert_rsrp_to_rssi(signal_info.rsrp as f64, parse_bandwidth_str(&bw).unwrap()).unwrap_or(0.0);
    // dbg!(convert_rsrp_to_rssi(signal_info.rsrp as f64, parse_bandwidth_str(&bw).unwrap()));
    // let rssi_values = convert_rsrp_to_rssi(Some(signal_info.rsrp as f64), parse_bandwidth_str(&bw.unwrap()));
    // println!("RSSI: {:?}", rssi_values);

    // let re_xmci = Regex::new(r#"\+XMCI: (?P<carrier_id>[45]),(?P<mcc>\d+),(?P<mnc>\d+),"(?P<ci>[^"]*)","(?P<e_ci>[^"]*)","(?P<pci>[^"]*)","(?P<earfcn_dl>[^"]*)","(?P<earfcn_ul>[^"]*)","(?P<band>[^"]*)",(?P<rssi>\d+),(?P<rsrp>\d+),(?P<rsrq>\d+),"(?P<sinr>[^"]*)","(?P<timing_advance>[^"]*)""#).unwrap();
    let re_xmci = Regex::new(r#"\+XMCI: (?P<type>[45]),(?P<mcc>\d+),(?P<mnc>\d+),"(?P<tac>[^"]*)","(?P<ci_x>[^"]*)","(?P<pci_x>[^"]*)","(?P<dluarfnc_x>[^"]*)","(?P<earfcn_ul>[^"]*)","(?P<pathloss_lte>[^"]*)",(?P<rsrp>\d+),(?P<rsrq>\d+),(?P<sinr>\d+),"(?P<timing_advance>[^"]*)","(?P<cqi>[^"]*)""#).unwrap();
    let mut dluarfnc = Vec::new();
    for caps in re_xmci.captures_iter(&xmci_response) {
        signal_info.ci_x.push(hex_to_decimal(caps.name("ci_x").unwrap().as_str()).unwrap_or(0));
        signal_info.pci_x.push(hex_to_decimal(caps.name("pci_x").unwrap().as_str()).unwrap_or(0));
        let dluarfnc_x: i32 = hex_to_decimal(caps.name("dluarfnc_x").unwrap().as_str()).unwrap_or(0);
        signal_info.earfcn_x.push(dluarfnc_x);
        // let earfcn_ul = hex_to_decimal(caps.name("earfcn_ul").unwrap().as_str()).unwrap_or(0);
        signal_info.rsrp_x.push(caps.name("rsrp").unwrap().as_str().parse::<i32>().unwrap_or(0) - 141);
        signal_info.rsrq_x.push(caps.name("rsrq").unwrap().as_str().parse::<i32>().unwrap_or(0) / 2 - 20);
        signal_info.sinr_x.push(caps.name("sinr").unwrap().as_str().parse::<i32>().unwrap_or(0) /2 );
        dluarfnc.push(dluarfnc_x);

    }

    // Преобразование команды AT+XLEC и определение полосы и диапазонов
    let xlec_response = send_at_command(&mut *port, "AT+XLEC?")?;
    // +XLEC: 0,2,5,3,BAND_LTE_3
    let re_xlec = Regex::new(r#"\+XLEC: (?:\d+),(?P<no_of_cells>\d+),(?P<bw>(?:\d+,?)+),BAND_LTE_(?P<band>(?:\d+,?)+)"#)?;

    let mut band = "--".to_string();
    let mut bw: Option<String> = None;

    if let Some(caps) = re_xlec.captures(&xlec_response) {
        let ca_number = caps.name("no_of_cells").unwrap().as_str().parse::<usize>().unwrap_or(1);

        let ca_bw_x: Vec<_> = caps.name("bw").unwrap().as_str().split(',').collect::<Vec<_>>();
        let mut ca_band_x: Vec<_> = caps.name("band").unwrap().as_str().split(',').map(|s| s.to_string()).collect();

        let mut band_info = String::new();

        if ca_band_x.len() != ca_number {
            // println!("Warning: mismatch in number of cells and band data");
            for i in 0..ca_number {
                let bw_str = ca_bw_x[i].to_string();
                let bw = parse_bandwidth(&bw_str);
                band = dluarfnc.get(i).map_or("--".to_string(), |b| format!("{}", get_band_lte(*b)));
                band_info.push_str(&format!("{}@{}MHz ", band, bw));
            }
        } else {
            for i in 0..ca_number {
                let bw_str = ca_bw_x[i].to_string();
                let bw = parse_bandwidth(&bw_str);
                let band = ca_band_x.get(i).map_or("--".to_string(), |b| format!("B{}", b));
                band_info.push_str(&format!("{}@{}MHz ", band, bw));
            }
        }

        bw = Some(ca_bw_x.join(","));
        // dbg!(parse_bandwidth_str(&bw.unwrap().split(',').collect::<Vec<_>>()[0]));
        signal_info.rssi = convert_rsrp_to_rssi(signal_info.rsrp , bw.unwrap().split(',').collect::<Vec<_>>()[0].parse::<i32>().unwrap_or(0)).unwrap();
        // dbg!(convert_rsrp_to_rssi(signal_info.rsrp as f64, parse_bandwidth_str(&bw).unwrap()));
        // let rssi_values = convert_rsrp_to_rssi(Some(signal_info.rsrp as f64), parse_bandwidth_str(&bw.unwrap()));
        // println!("RSSI: {:?}", rssi_values);

        signal_info.band = band_info;
    } else if let Some(dluarfnc_x) = dluarfnc.first() {
        let bw_str = bw.as_ref().unwrap_or(&"".to_string()).clone();
        let bw_value = parse_bandwidth(&bw_str);
        let band_str = get_band_lte(*dluarfnc_x);
        signal_info.band = format!("{}@{}MHz", band_str, bw_value);
    }

    Ok(signal_info)
}