use regex::Regex;
use crate::modem_tools::converters::get_band_lte;
use crate::display_tools::bars::get_bar;

pub struct AtRegexps {
    pub cgmi_regex: Regex,
    pub fmm_regex: Regex,
    pub gtpkgver_regex: Regex,
    pub cfsn_regex: Regex,
    pub cgsn_regex: Regex,
    pub cimi_regex: Regex,
    pub csq_regex: Regex,
    pub ccid_regex: Regex,
    pub cgcontrdp_regex: Regex,
    pub cops_regex: Regex,
    pub xmci4_regex: Regex,
    pub xmci45_regex: Regex,
    pub xlec_regex: Regex,
    pub bands_regex: Regex,
}

#[derive(Default, Clone)]
pub struct ModemInfo {
    pub manufacturer: String,
    pub model: String,
    pub fw_version: String,
    pub serial_number: String,
    pub imei: String,
    pub imsi: String,
    pub iccid: String,
    pub operator: String,
    pub mode: String,
    pub ip: String,
    pub mask: String,
    pub gw: String,
    pub dns_prim: String,
    pub dns_sec: String,
    pub band: String,
    // pub bw: String,
    pub distance: f64,
    pub dluarfnc: i32,
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

impl ModemInfo {
    pub fn display_modem_info(&self) -> String{
        format!("Manufacturer:        {}\n\
        Model:               {}\n\
        Firmware Version:    {}\n\
        Serial Number:       {}\n\
        IMEI:                {}\n",
        &self.manufacturer,
        &self.model,
        &self.fw_version,
        &self.serial_number,
        &self.imei)
    }

    pub fn display_signal_info(&self) -> String {
        format!("Operator:             {} ({})\n\
            IP/Mask:              {} / {}\n\
            DNS:                  {} {}\n\
            Distance:             {}m\n\n\
            Signal:               {:>2}%     [{}]\n\
            RSSI:                 {:>2}dBm  [{}]\n\
            SINR:                 {:>2}dB    [{}]\n\
            RSRP:                 {:>2}dBm  [{}]\n\
            RSRQ:                 {:>2}db   [{}]\n\
            Band:                 {}\n\
                EARFCN:               {}\n\
            ",
            self.operator, self.mode,
            self.ip, self.mask,
            self.dns_prim, self.dns_sec,
            self.distance,
            self.csq_perc, get_bar(self.csq_perc, 0, 100),
            self.rssi, get_bar(self.rssi, -110, -25),
            self.sinr, get_bar(self.sinr, -10, 30),
            self.rsrp, get_bar(self.rsrp, -120, -50),
            self.rsrq, get_bar(self.rsrq, -25, -1),
            self.band,
            self.dluarfnc
            )
    }
    pub fn display_carrier_info(&self) -> String {
        let mut carrier_info: String = "".to_string();
        for (index, &_) in self.ci_x.iter().enumerate(){
            let band = get_band_lte(self.earfcn_x[index]);
            let rsrp_str = format!("{}dBm", self.rsrp_x[index]);
            let rsrq_str = format!("{}dB", self.rsrq_x[index]);
            let sinr_str = format!("{}dB", self.sinr_x[index]);

            let rsrp_bar = get_bar(self.rsrp_x[index], -12, -50);
            let rsrq_bar = get_bar(self.rsrq_x[index], -25, -1);
            let sinr_bar = get_bar(self.sinr_x[index], -10, 30);

            carrier_info.push_str(format!(
                "===Carrier {:2}: CI: {:8} PCI: {:4} Band (EARFCN): {:3} ({:5}) RSRP: {:>5} [{}] RSRQ: {:>5} [{}] SINR: {:2} [{}]\n",
                index, self.ci_x[index], self.pci_x[index], band, self.earfcn_x[index], rsrp_str, rsrp_bar, rsrq_str, rsrq_bar, sinr_str, sinr_bar
            ).as_str());
        }
        carrier_info
    }
}

// impl fmt::Debug for ModemInfo {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("SignalInfo")
//             .field("band", &self.band)
//             .field("bw", &self.bw)
//             .field("csq", &self.csq)
//             .field("csq_perc", &self.csq_perc)
//             .field("ci", &self.ci_x)
//             .field("pci", &self.pci_x)
//             .field("earfcn", &self.earfcn_x)
//             .field("rsrp", &self.rsrp_x)
//             .field("rsrq", &self.rsrq_x)
//             .finish()
//     }
// }
