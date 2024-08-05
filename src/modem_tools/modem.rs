use std::io;
use std::time::Duration;
use std::string::ToString;
use once_cell::sync::Lazy;
use regex::Regex;
use serialport::SerialPort;
use crate::modem_tools::converters::{get_band_lte, hex_to_decimal, parse_bandwidth, convert_rsrp_to_rssi};
use crate::modem_tools::types::{ModemInfo, AtRegexps};

pub static REGEXPS: Lazy<AtRegexps> = Lazy::new(|| AtRegexps {
    cgmi_regex: Regex::new(r#"\+CGMI: "([^"]+)""#).unwrap(),
    fmm_regex: Regex::new(r#"\+FMM: "([^"]+)"(?:,"([^"]+)")?"#).unwrap(),
    gtpkgver_regex: Regex::new(r#"\+GTPKGVER: "([^"]+)""#).unwrap(),
    cfsn_regex: Regex::new(r#"\+CFSN: "([^"]+)""#).unwrap(),
    cgsn_regex: Regex::new(r#"\+CGSN: "([^"]+)""#).unwrap(),
    cimi_regex: Regex::new(r#"\+CIMI: (\d+)"#).unwrap(),
    csq_regex: Regex::new(r#"\+CSQ: (\d+),(\d+)"#).unwrap(),
    ccid_regex: Regex::new(r#"\+CCID: (\d+)"#).unwrap(),
    cgcontrdp_regex: Regex::new(r#"\+CGCONTRDP: (?P<index>\d),(?P<cid>\d+),"(?P<apn>[^"]+)","(?P<ip_addr>\d+\.\d+\.\d+\.\d+)\.(?P<mask>\d+\.\d+\.\d+\.\d+)","(?P<dns_prim>[^"]+)","(?P<dns_sec>[^"]+)","(?P<gw_addr>[^"]+)","(?P<p_cscf_prim>[^"]*)","(?P<p_cscf_sec>[^"]*)",(?P<mtu>\d+)"#).unwrap(),
    cops_regex: Regex::new(r#"\+COPS: (\d),(\d),"([^"]*)",(\d)"#).unwrap(),
    xmci4_regex: Regex::new(r#"\+XMCI: (?P<type>4),(?P<mcc>\d+),(?P<mnc>\d+),"(?P<tac>[^"]*)","(?P<ci_x>[^"]*)","(?P<pci_x>[^"]*)","(?P<dluarfnc_x>[^"]*)","(?P<earfcn_ul>[^"]*)","(?P<pathloss_lte>[^"]*)",(?P<rsrp>\d+),(?P<rsrq>\d+),(?P<sinr>-?\d+),"(?P<timing_advance>[^"]*)","(?P<cqi>[^"]*)""#).unwrap(),
    xmci45_regex: Regex::new(r#"\+XMCI: (?P<type>[45]),(?P<mcc>\d+),(?P<mnc>\d+),"(?P<tac>[^"]*)","(?P<ci_x>[^"]*)","(?P<pci_x>[^"]*)","(?P<dluarfnc_x>[^"]*)","(?P<earfcn_ul>[^"]*)","(?P<pathloss_lte>[^"]*)",(?P<rsrp>\d+),(?P<rsrq>\d+),(?P<sinr>-?\d+),"(?P<timing_advance>[^"]*)","(?P<cqi>[^"]*)""#).unwrap(),
    xlec_regex: Regex::new(r#"\+XLEC: (?:\d+),(?P<no_of_cells>\d+),(?P<bw>(?:\d+,?)+),BAND_LTE_(?P<band>(?:\d+,?)+)"#).unwrap(),
    bands_regex: Regex::new(r#"\+XACT: (?P<umts_flag>\d+),(?P<lte_flag>\d+),\d+,(?P<umts_bands>(?:\d+,)*\d+),(?P<lte_bands>(?:1\d{2},)*1\d{2})\r?"#).unwrap(),
});

fn send_at_command(port: &mut dyn SerialPort, command: &str) -> Result<String, io::Error> {
    port.write_all(command.as_bytes())?;
    port.write_all(b"\r")?;
    port.flush()?;
    let mut response = String::new();
    let mut serial_buf: Vec<u8> = vec![0; 200];
    let timeout = std::time::Duration::from_millis(100);
    let start = std::time::Instant::now();

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                response.push_str(&String::from_utf8_lossy(&serial_buf[..t]));
                if response.contains("OK") || response.contains("ERROR") {
                    break;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                if start.elapsed() > timeout {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }

    Ok(response)
}

pub fn modem_execute(port_name: &String, baud_rate: u32, command: &str) -> Result<String, io::Error>{
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_secs(1))
        .open()?;
    port.write_data_terminal_ready(true)?; // Включение DTR
    send_at_command(&mut *port, command)
}

pub fn get_modem_info_string(port_name: &str, baud_rate: u32) -> Result<String, std::io::Error> {
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_secs(1))
        .open()?;
    port.write_data_terminal_ready(true)?; // Включение DTR

    let mut signal_info_string: String = String::from("");
    signal_info_string.push_str(send_at_command(&mut *port, "AT+CGMI?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+FMM?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+GTPKGVER?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+CFSN?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+CGSN?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+CIMI?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+CCID?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+COPS?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+CGCONTRDP=1")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+CSQ?")?.as_str());
    signal_info_string.push_str(send_at_command(&mut *port, "AT+XCCINFO?; +XLEC?; +XMCI=1")?.as_str());
    Ok(signal_info_string)
}

pub fn get_modem_info(info_string: String) -> Result<ModemInfo, Box<dyn std::error::Error>> {

    let mut signal_info: ModemInfo = Default::default();

    // Modem Manufacturer
    let re_cgmi = &REGEXPS.cgmi_regex;
    if let Some(caps) = re_cgmi.captures(&info_string) {
        let manufacturer = caps.get(1).unwrap().as_str();
        signal_info.manufacturer = manufacturer.parse().unwrap();
    }

    // Modem model
    let re_fmm = &REGEXPS.fmm_regex;
    if let Some(caps) = re_fmm.captures(&info_string) {
        let model = caps.get(1).unwrap().as_str();
        signal_info.model = model.parse().unwrap();
    }

    // Firmware Version
    let re_gtpkgver = &REGEXPS.gtpkgver_regex;
    if let Some(caps) = re_gtpkgver.captures(&info_string) {
        let firmware_ver = caps.get(1).unwrap().as_str();
        signal_info.fw_version = firmware_ver.parse().unwrap();
    }

    // Serial Number
    let re_cfsn = &REGEXPS.cfsn_regex;
    if let Some(caps) = re_cfsn.captures(&info_string) {
        let serial_number = caps.get(1).unwrap().as_str();
        signal_info.serial_number = serial_number.parse().unwrap();
    }

    // IMEI
    let re_cgsn = &REGEXPS.cgsn_regex;
    if let Some(caps) = re_cgsn.captures(&info_string) {
        let imei = caps.get(1).unwrap().as_str();
        signal_info.imei = imei.parse().unwrap();
    }

    let re_cgcontrdp = &REGEXPS.cgcontrdp_regex;
    if let Some(caps) = re_cgcontrdp.captures(&info_string) {
        signal_info.ip = caps.name("ip_addr").unwrap().as_str().parse().unwrap();
        signal_info.mask = caps.name("mask").unwrap().as_str().parse().unwrap();
        signal_info.gw = caps.name("gw_addr").unwrap().as_str().parse().unwrap();
        signal_info.dns_prim = caps.name("dns_prim").unwrap().as_str().parse().unwrap();
        signal_info.dns_sec = caps.name("dns_sec").unwrap().as_str().parse().unwrap();
    }

    // IMSI
    let re_cimi = &REGEXPS.cimi_regex;
    if let Some(caps) = re_cimi.captures(&info_string) {
        let imsi = caps.get(1).unwrap().as_str();
        signal_info.imsi = imsi.parse().unwrap();
    }

    // ICCID
    let re_ccid = &REGEXPS.ccid_regex;
    if let Some(caps) = re_ccid.captures(&info_string) {
        let ccid = caps.get(1).unwrap().as_str();
        signal_info.iccid = ccid.parse().unwrap();
    }

    // Operator and connection mode
    let re_cops = &REGEXPS.cops_regex;
    if let Some(caps) = re_cops.captures(&info_string) {
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
        signal_info.operator = operator.parse().unwrap();
        signal_info.mode = mode.parse().unwrap();
    }

    let re_csq = &REGEXPS.csq_regex;
    if let Some(caps) = re_csq.captures(&info_string) {
        signal_info.csq = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
        signal_info.csq_perc = if signal_info.csq >= 0 && signal_info.csq <= 31 {
            signal_info.csq * 100 / 31
        } else {
            0
        };
    }

    let re_xmci = &REGEXPS.xmci4_regex;
    if let Some(caps) = re_xmci.captures(&info_string) {
        signal_info.rsrp = caps.name("rsrp").unwrap().as_str().parse::<i32>().unwrap_or(0) - 141;
        signal_info.rsrq = caps.name("rsrq").unwrap().as_str().parse::<i32>().unwrap_or(0) / 2 - 20;
        signal_info.sinr = caps.name("sinr").unwrap().as_str().parse::<i32>().unwrap_or(0) / 2;
        signal_info.distance = (hex_to_decimal(caps.name("timing_advance").unwrap().as_str()).unwrap_or(0) as f64 * 78.125).round();
        signal_info.dluarfnc = hex_to_decimal(caps.name("dluarfnc_x").unwrap().as_str()).unwrap_or(0)

    }


    // let re_xmci = Regex::new(r#"\+XMCI: (?P<carrier_id>[45]),(?P<mcc>\d+),(?P<mnc>\d+),"(?P<ci>[^"]*)","(?P<e_ci>[^"]*)","(?P<pci>[^"]*)","(?P<earfcn_dl>[^"]*)","(?P<earfcn_ul>[^"]*)","(?P<band>[^"]*)",(?P<rssi>\d+),(?P<rsrp>\d+),(?P<rsrq>\d+),"(?P<sinr>[^"]*)","(?P<timing_advance>[^"]*)""#).unwrap();
    let re_xmci = &REGEXPS.xmci45_regex;
    let mut dluarfnc = Vec::new();

    for caps in re_xmci.captures_iter(&info_string) {
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

    // +XLEC: 0,2,5,3,BAND_LTE_3
    let re_xlec = &REGEXPS.xlec_regex;

    let mut band ;
    let mut bw: Option<String> = None;

    if let Some(caps) = re_xlec.captures(&info_string) {
        let ca_number = caps.name("no_of_cells").unwrap().as_str().parse::<usize>().unwrap_or(1);

        let ca_bw_x: Vec<_> = caps.name("bw").unwrap().as_str().split(',').collect::<Vec<_>>();
        let ca_band_x: Vec<_> = caps.name("band").unwrap().as_str().split(',').map(|s| s.to_string()).collect();

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
        signal_info.rssi = convert_rsrp_to_rssi(signal_info.rsrp , bw.unwrap().split(',').collect::<Vec<_>>()[0].parse::<i32>().unwrap_or(0)).unwrap();

        signal_info.band = band_info;
    } else if let Some(dluarfnc_x) = dluarfnc.first() {
        let bw_str = bw.as_ref().unwrap_or(&"".to_string()).clone();
        let bw_value = parse_bandwidth(&bw_str);
        let band_str = get_band_lte(*dluarfnc_x);
        signal_info.band = format!("{}@{}MHz", band_str, bw_value);
    }

    Ok(signal_info)
}