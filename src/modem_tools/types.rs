use std::fmt;
use crate::modem_tools::converters::get_band_lte;
use crate::display_tools::bars::get_bar;

#[derive(Default)]
pub struct ModemInfo {
    pub manufacturer: String,
    pub model: String,
    pub fw_version: String,
    pub serial_number: String,
    pub imei: String,
}

impl ModemInfo {
    pub fn print(&self) {
        println!("===Modem info:\n\
        Manufacturer:        {}\n\
        Model:               {}\n\
        Firmware Version:    {}\n\
        Serial Number:       {}\n\
        IMEI:                {}",
        &self.manufacturer,
        &self.model,
        &self.fw_version,
        &self.serial_number,
        &self.imei);
        println!()
    }
}

#[derive(Default)]
pub struct SimInfo {
    pub imsi: String,
    pub iccid: String,
    pub operator: String,
    pub mode: String,
    pub ip: String,
    pub mask: String,
    pub gw: String
}

impl SimInfo {
    pub fn print(&self) {
        println!("===SIM info:\n\
        IMSI:                {}\n\
        ICCID:               {}\n\
        Operator:            {} ({})\n\
        IP:                  {}\n\
        Mask:                {}\n\
        GW:                  {}",
        &self.imsi,
        &self.iccid,
        &self.operator,
        &self.mode,
        &self.ip,
        &self.mask,
        &self.gw);
        println!()
    }
}

#[derive(Default)]
pub struct SignalInfo {
    pub band: String,
    pub bw: String,
    pub csq: i32,
    pub csq_perc: i32,
    pub rssi: i32,
    pub sinr: i32,
    pub rsrp: i32,
    pub rsrq: i32,
    pub ci_x: Vec<i32>,
    pub pci_x: Vec<i32>,
    pub earfcn_x: Vec<i32>,
    pub rsrp_x: Vec<i32>,
    pub rsrq_x: Vec<i32>,
    pub sinr_x: Vec<i32>,
}

impl SignalInfo {
    pub fn display_carrier_info(&self) {
        for (index, &carrier_id) in self.ci_x.iter().enumerate(){
            let band = get_band_lte(self.earfcn_x[index]);
            let rsrp_str = format!("{}dBm", self.rsrp_x[index]);
            let rsrq_str = format!("{}dB", self.rsrq_x[index]);
            let sinr_str = format!("{}dB", self.sinr_x[index]);

            let rsrp_bar = get_bar(self.rsrp_x[index], -12, -50);
            let rsrq_bar = get_bar(self.rsrq_x[index], -25, -1);
            let sinr_bar = get_bar(self.sinr_x[index], -10, 30);

            println!(
                "===Carrier {:2}: CI: {:8} PCI: {:4} Band (EARFCN): {:3} ({:5}) RSRP: {:>5} [{}] RSRQ: {:>5} [{}] SINR: {:2} [{}]",
                index, self.ci_x[index], self.pci_x[index], band, self.earfcn_x[index], rsrp_str, rsrp_bar, rsrq_str, rsrq_bar, sinr_str, sinr_bar
            );
        }
    }
}

impl fmt::Debug for SignalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignalInfo")
            .field("band", &self.band)
            .field("bw", &self.bw)
            .field("csq", &self.csq)
            .field("csq_perc", &self.csq_perc)
            .field("ci", &self.ci_x)
            .field("pci", &self.pci_x)
            .field("earfcn", &self.earfcn_x)
            .field("rsrp", &self.rsrp_x)
            .field("rsrq", &self.rsrq_x)
            .finish()
    }
}
