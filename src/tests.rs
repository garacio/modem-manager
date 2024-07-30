#[cfg(test)]
mod tests {
    use crate::modem_tools::modem::REGEXPS;

    // static TEST_STRING: &str = "AT+CSQ?\r\r\n+CSQ: 11,2\r\n\r\nOK\r\n\
    // AT+XCCINFO?; +XLEC?; +XMCI=1\r\r\n\
    // +XCCINFO: 0,220,03,\"00009C03\",3,103,\"FFFF\",1,\"FF\",\"4E91\",0,0,0,0,0,0,0,0\r\n\r\n\
    // +XLEC: 0,2,5,3,BAND_LTE_3\r\n\r\n\
    // +XMCI: 4,220,03,\"0x4E91\",\"0x00009C03\",\"0x0062\",\"0x000005DC\",\"0x00004C2C\",\"0xFFFFFFFF\",49,17,-4,\"0x00000003\",\"0x00000000\"\r\n\r\n\
    // +XMCI: 5,000,000,\"0xFFFE\",\"0xFFFFFFFF\",\"0x0061\",\"0x000005DC\",\"0xFFFFFFFF\",\"0xFFFFFFFF\",42,2,255,\"0x7FFFFFFF\",\"0x00000000\"\r\n\r\n\
    // +XMCI: 5,000,000,\"0xFFFE\",\"0xFFFFFFFF\",\"0x006A\",\"0x000005DC\",\"0xFFFFFFFF\",\"0xFFFFFFFF\",43,8,255,\"0x7FFFFFFF\",\"0x00000000\"\r\n\r\n\
    // OK\r\n";

    static TEST_STRING: &str = "+CGMI: \"Fibocom\"\r\n\r\nOK\r\n\
    +FMM: \"L850 LTE Module\",\"L850\"\r\n\
    +GTPKGVER: \"18500.5001.00.05.27.30_5001.05.001.035\"\r\n\r\nOK\r\n\
    +CFSN: \"D1M2LG1EA3\"\r\n\r\nOK\r\n\
    +CGSN: \"015550006919978\"\r\n\r\nOK\r\n\
    +CIMI: 220033400995562\r\n\r\nOK\r\n\
    +CCID: 89381030000328789401\r\n\r\nOK\r\n\
    +COPS: 0,0,\"mt:s\",7\r\n\r\nOK\r\n\
    AT+CSQ?\r\r\n+CSQ: 11,2\r\n\r\nOK\r\n\
    AT+XCCINFO?; +XLEC?; +XMCI=1\r\r\n\
    +XCCINFO: 0,220,03,\"00009C03\",3,103,\"FFFF\",1,\"FF\",\"4E91\",0,0,0,0,0,0,0,0\r\n\r\n\
    +XLEC: 0,2,5,3,BAND_LTE_3\r\n\r\n\
    +XMCI: 4,220,03,\"0x4E91\",\"0x00009C03\",\"0x0062\",\"0x000005DC\",\"0x00004C2C\",\"0xFFFFFFFF\",49,17,-4,\"0x00000003\",\"0x00000000\"\r\n\r\n\
    +XMCI: 5,000,000,\"0xFFFE\",\"0xFFFFFFFF\",\"0x0061\",\"0x000005DC\",\"0xFFFFFFFF\",\"0xFFFFFFFF\",42,2,255,\"0x7FFFFFFF\",\"0x00000000\"\r\n\r\n\
    +XMCI: 5,000,000,\"0xFFFE\",\"0xFFFFFFFF\",\"0x006A\",\"0x000005DC\",\"0xFFFFFFFF\",\"0xFFFFFFFF\",43,8,255,\"0x7FFFFFFF\",\"0x00000000\"\r\n\r\nOK\r\n";


    #[test]
    fn test_cgmi_regex() {
        let re_cgmi = &REGEXPS.cgmi_regex;
        let caps = re_cgmi.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "Fibocom");
    }

    #[test]
    fn test_fmm_regex() {
        let re_cgmi = &REGEXPS.fmm_regex;
        let caps = re_cgmi.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "L850 LTE Module");
        assert_eq!(caps.get(2).unwrap().as_str(), "L850");
    }

    #[test]
    fn test_gtpkgver_regex() {
        let re_gtpkgver = &REGEXPS.gtpkgver_regex;
        let caps = re_gtpkgver.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "18500.5001.00.05.27.30_5001.05.001.035");
    }

    #[test]
    fn test_cfsn_regex() {
        let re_cfsn = &REGEXPS.cfsn_regex;
        let caps = re_cfsn.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "D1M2LG1EA3");
    }

    #[test]
    fn test_cgsn_regex() {
        let re_cgsn = &REGEXPS.cgsn_regex;
        let caps = re_cgsn.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "015550006919978");
    }

    #[test]
    fn test_cimi_regex() {
        let re_cimi = &REGEXPS.cimi_regex;
        let caps = re_cimi.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "220033400995562");
    }

    #[test]
    fn test_ccid_regex() {
        let re_ccid = &REGEXPS.ccid_regex;
        let caps = re_ccid.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "89381030000328789401");
    }

    #[test]
    fn test_cops_regex() {
        let re_cops = &REGEXPS.cops_regex;
        let caps = re_cops.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(3).unwrap().as_str(), "mt:s");
        assert_eq!(caps.get(4).unwrap().as_str(), "7");
    }

    #[test]
    fn test_csq_regex() {
        let re_csq = &REGEXPS.csq_regex;
        let caps = re_csq.captures(TEST_STRING).unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "11");
        assert_eq!(caps.get(2).unwrap().as_str(), "2");
    }

    #[test]
    fn test_xlec_regex() {
        let re_xlec = &REGEXPS.xlec_regex;
        let caps = re_xlec.captures(TEST_STRING).unwrap();
        assert_eq!(caps.name("no_of_cells").unwrap().as_str(), "2");
        assert_eq!(caps.name("bw").unwrap().as_str(), "5,3");
        assert_eq!(caps.name("band").unwrap().as_str(), "3");
    }

    #[test]
    fn test_xmci_regex() {
        let re_xmci = &REGEXPS.xmci4_regex;
        // let test_string = r#"+XMCI: 4,310,260,"7012","A1B2","C3D4","E5F6","1234","0",97,20,40,"00A1","10""#;
        let caps = re_xmci.captures(TEST_STRING).unwrap();
        assert_eq!(caps.name("type").unwrap().as_str(), "4");
        assert_eq!(caps.name("mcc").unwrap().as_str(), "220");
        assert_eq!(caps.name("mnc").unwrap().as_str(), "03");
        assert_eq!(caps.name("tac").unwrap().as_str(), "0x4E91");
        assert_eq!(caps.name("ci_x").unwrap().as_str(), "0x00009C03");
        assert_eq!(caps.name("pci_x").unwrap().as_str(), "0x0062");
        assert_eq!(caps.name("dluarfnc_x").unwrap().as_str(), "0x000005DC");
        assert_eq!(caps.name("earfcn_ul").unwrap().as_str(), "0x00004C2C");
        assert_eq!(caps.name("pathloss_lte").unwrap().as_str(), "0xFFFFFFFF");
        assert_eq!(caps.name("rsrp").unwrap().as_str(), "49");
        assert_eq!(caps.name("rsrq").unwrap().as_str(), "17");
        assert_eq!(caps.name("sinr").unwrap().as_str(), "-4");
        assert_eq!(caps.name("timing_advance").unwrap().as_str(), "0x00000003");
        assert_eq!(caps.name("cqi").unwrap().as_str(), "0x00000000");
    }
}
