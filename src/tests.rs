#[cfg(test)]
mod tests {
    use std::error::Error;
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
    +CGCONTRDP: 1,6,\"3gnet.mnc003.mcc220.gprs\",\"10.179.248.170.255.0.0.0\",\"10.179.248.171\",\"172.22.23.175\",\"172.21.8.175\",\"\",\"\",0\r\n\r\n\
    +CGCONTRDP: 1,6,\"3gnet.mnc003.mcc220.gprs\",\"10.179.248.170.255.0.0.0\",\"10.179.248.171\",\"172.22.23.175\",\"172.22.23.175\",\"\",\"\",0\r\n\r\n\
    +CGCONTRDP: 1,6,\"3gnet.mnc003.mcc220.gprs\",\"10.179.248.170.255.0.0.0\",\"10.179.248.171\",\"172.22.23.175\",\"172.21.8.175\",\"\",\"\",0\r\n\r\n\
    OK\r\n\
    AT+CSQ?\r\r\n+CSQ: 11,2\r\n\r\nOK\r\n\
    AT+XCCINFO?; +XLEC?; +XMCI=1\r\r\n\
    +XCCINFO: 0,220,03,\"00009C03\",3,103,\"FFFF\",1,\"FF\",\"4E91\",0,0,0,0,0,0,0,0\r\n\r\n\
    +XLEC: 0,2,5,3,BAND_LTE_3\r\n\r\n\
    +XMCI: 4,220,03,\"0x4E91\",\"0x00009C03\",\"0x0062\",\"0x000005DC\",\"0x00004C2C\",\"0xFFFFFFFF\",49,17,-4,\"0x00000003\",\"0x00000000\"\r\n\r\n\
    +XMCI: 5,000,000,\"0xFFFE\",\"0xFFFFFFFF\",\"0x0061\",\"0x000005DC\",\"0xFFFFFFFF\",\"0xFFFFFFFF\",42,2,255,\"0x7FFFFFFF\",\"0x00000000\"\r\n\r\n\
    +XMCI: 5,000,000,\"0xFFFE\",\"0xFFFFFFFF\",\"0x006A\",\"0x000005DC\",\"0xFFFFFFFF\",\"0xFFFFFFFF\",43,8,255,\"0x7FFFFFFF\",\"0x00000000\"\r\n\r\n\
    OK\r\n\r\n\
    +XACT: 4,2,1,1,2,4,5,8,101,102,103,104,105,107,108,112,113,117,118,119,120,126,128,129,130,141,166\r\n";



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
fn test_cgcontrdp_regex() {
    let re_cgcontrdp = &REGEXPS.cgcontrdp_regex;
    let caps = re_cgcontrdp.captures(TEST_STRING).unwrap();
    assert_eq!(caps.name("index").unwrap().as_str(), "1");
    assert_eq!(caps.name("cid").unwrap().as_str(), "6");
    assert_eq!(caps.name("apn").unwrap().as_str(), "3gnet.mnc003.mcc220.gprs");
    assert_eq!(caps.name("ip_addr").unwrap().as_str(), "10.179.248.170");
    assert_eq!(caps.name("mask").unwrap().as_str(), "255.0.0.0");
    assert_eq!(caps.name("dns_prim").unwrap().as_str(), "10.179.248.171");
    assert_eq!(caps.name("dns_sec").unwrap().as_str(), "172.22.23.175");
    assert_eq!(caps.name("gw_addr").unwrap().as_str(), "172.21.8.175");
    assert_eq!(caps.name("p_cscf_prim").unwrap().as_str(), "");
    assert_eq!(caps.name("p_cscf_sec").unwrap().as_str(), "");
    assert_eq!(caps.name("mtu").unwrap().as_str(), "0");
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

    #[test]
    fn test_xact(){
        let re_bands = &REGEXPS.bands_regex;
        let caps = re_bands.captures(TEST_STRING).unwrap();
        let umts_flag = caps.name("umts_flag").map_or("", |m| m.as_str());
        let lte_flag = caps.name("lte_flag").map_or("", |m| m.as_str());
        let umts_bands_str = caps.name("umts_bands").map_or("", |m| m.as_str());
        let lte_bands_str = caps.name("lte_bands").map_or("", |m| m.as_str());

        // Определение режима
        let is_umts_enabled = umts_flag == "4";
        let is_lte_enabled = lte_flag == "2";

        // Извлечение и вывод UMTS бэндов
        let umts_bands: Vec<&str> = umts_bands_str.split(',').filter(|s| !s.is_empty()).collect();

        // Извлечение и преобразование LTE бэндов
        let lte_bands: Vec<String> = lte_bands_str
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .map(|n| (n - 100).to_string())
            .collect();

        assert!(is_umts_enabled, "UMTS должен быть включен");
        assert!(is_lte_enabled, "LTE должен быть включен");
        assert_eq!(umts_bands, vec!["1", "2", "4", "5", "8"]);
        assert_eq!(lte_bands, vec!["1", "2", "3", "4", "5", "7", "8", "12", "13", "17", "18", "19", "20", "26", "28", "29", "30", "41", "66"]);

        //
        // let modes = caps.name("modes").map_or("", |m| m.as_str());
        // let umts_bands_str = caps.name("umts_bands").map_or("", |m| m.as_str());
        // let lte_bands_str = caps.name("lte_bands").map_or("", |m| m.as_str());
        //
        // // Определение режима
        // let is_lte_enabled = modes.contains('2');
        // let is_umts_enabled = modes.contains('4');
        //
        // // Извлечение и вывод UMTS бэндов
        // let umts_bands: Vec<&str> = umts_bands_str.split(',').filter(|s| !s.is_empty()).collect();
        //
        // // Извлечение и преобразование LTE бэндов
        // let lte_bands: Vec<String> = lte_bands_str
        //     .split(',')
        //     .filter_map(|s| s.parse::<i32>().ok())
        //     .map(|n| (n - 100).to_string())
        //     .collect();
        //
        // assert_eq!(modes.to_string(), "4");
        // assert_eq!(umts_bands_str.to_string(), "");
        // assert_eq!(lte_bands_str.to_string(), "");
        // // assert!(is_umts_enabled, "UMTS должен быть включен");
        // // assert!(is_lte_enabled, "LTE должен быть включен");
        // assert!(umts_bands.is_empty(), "UMTS бэнды должны быть пустыми");
        // assert_eq!(lte_bands, vec!["1", "3", "20"]);
    }
    #[test]
    fn test_lte_only() {
        let xact_string = "+XACT: 2,2,,101,103,120";
        let re_bands = &REGEXPS.bands_regex;

        if let Some(caps) = re_bands.captures(xact_string) {
            let modes = caps.name("modes").map_or("", |m| m.as_str());
            let umts_bands_str = caps.name("umts_bands").map_or("", |m| m.as_str());
            let lte_bands_str = caps.name("lte_bands").map_or("", |m| m.as_str());

            // Определение режима
            let is_lte_enabled = modes.contains('2');
            let is_umts_enabled = modes.contains('4');

            // Извлечение и вывод UMTS бэндов
            let umts_bands: Vec<&str> = umts_bands_str.split(',').filter(|s| !s.is_empty()).collect();

            // Извлечение и преобразование LTE бэндов
            let lte_bands: Vec<String> = lte_bands_str
                .split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .map(|n| (n - 100).to_string())
                .collect();

            assert!(!is_umts_enabled, "UMTS должен быть выключен");
            assert!(is_lte_enabled, "LTE должен быть включен");
            assert!(umts_bands.is_empty(), "UMTS бэнды должны быть пустыми");
            assert_eq!(lte_bands, vec!["1", "3", "20"]);
        } else {
            panic!("Строка не соответствует ожидаемому формату");
        }
    }

    #[test]
    fn test_both_modes_limited_bands() {
        let xact_string = "+XACT: 4,2,1,1,2,4,5,8,101,103,120";
        let re_bands = &REGEXPS.bands_regex;

        if let Some(caps) = re_bands.captures(xact_string) {
            let modes = caps.name("modes").map_or("", |m| m.as_str());
            let umts_bands_str = caps.name("umts_bands").map_or("", |m| m.as_str());
            let lte_bands_str = caps.name("lte_bands").map_or("", |m| m.as_str());

            // Определение режима
            let is_lte_enabled = modes.contains('2');
            let is_umts_enabled = modes.contains('1');

            // Извлечение и вывод UMTS бэндов
            let umts_bands: Vec<&str> = umts_bands_str.split(',').filter(|s| !s.is_empty()).collect();

            // Извлечение и преобразование LTE бэндов
            let lte_bands: Vec<String> = lte_bands_str
                .split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .map(|n| (n - 100).to_string())
                .collect();

            assert!(is_umts_enabled, "UMTS должен быть включен");
            assert!(is_lte_enabled, "LTE должен быть включен");
            assert_eq!(umts_bands, vec!["1", "1", "2", "4", "5", "8"]);
            assert_eq!(lte_bands, vec!["1", "3", "20"]);
        } else {
            panic!("Строка не соответствует ожидаемому формату");
        }
    }

    #[test]
    fn test_umts_only() {
        let xact_string = "+XACT: 1,1,1,2,5";
        let re_bands = &REGEXPS.bands_regex;

        if let Some(caps) = re_bands.captures(xact_string) {
            let modes = caps.name("modes").map_or("", |m| m.as_str());
            let umts_bands_str = caps.name("umts_bands").map_or("", |m| m.as_str());
            let lte_bands_str = caps.name("lte_bands").map_or("", |m| m.as_str());

            // Определение режима
            let is_lte_enabled = modes.contains('2');
            let is_umts_enabled = modes.contains('1');

            // Извлечение и вывод UMTS бэндов
            let umts_bands: Vec<&str> = umts_bands_str.split(',').filter(|s| !s.is_empty()).collect();

            // Извлечение и преобразование LTE бэндов
            let lte_bands: Vec<String> = lte_bands_str
                .split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .map(|n| (n - 100).to_string())
                .collect();

            assert!(is_umts_enabled, "UMTS должен быть включен");
            assert!(!is_lte_enabled, "LTE должен быть выключен");
            assert_eq!(umts_bands, vec!["1", "2", "5"]);
            assert!(lte_bands.is_empty(), "LTE бэнды должны быть пустыми");
        } else {
            panic!("Строка не соответствует ожидаемому формату");
        }
    }
}
