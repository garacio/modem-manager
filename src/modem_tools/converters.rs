pub fn parse_bandwidth(bandwidth: &str) -> String {
    bandwidth.split(',')
        .map(|bw| match bw {
            "0" => "1.4",
            "1" => "3",
            "2" => "5",
            "3" => "10",
            "4" => "15",
            "5" => "20",
            _ => "Unknown",
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn convert_rsrp_to_rssi(rsrp:i32, bandwidth: i32) -> Option<i32> {
    let np = match bandwidth {
        0 => 6,
        1 => 15,
        2 => 25,
        3 => 50,
        4 => 75,
        5 => 100,
        _ => 0,
    };

    if np > 0 {
        Some((rsrp as f64 + 10.0 * ((12 * np) as f64).log10())as i32)
    } else {
        Some(-113)
    }
}


pub fn get_band_lte(earfcn: i32) -> &'static str {
    match earfcn {
        0..=599 => "B1",
        600..=1199 => "B2",
        1200..=1949 => "B3",
        1950..=2399 => "B4",
        2400..=2649 => "B5",
        2750..=3449 => "B7",
        3450..=3799 => "B8",
        3800..=4149 => "B9",
        4150..=4749 => "B10",
        4750..=4949 => "B11",
        5010..=5179 => "B12",
        5180..=5279 => "B13",
        5280..=5379 => "B14",
        5730..=5849 => "B17",
        5850..=5999 => "B18",
        6000..=6149 => "B19",
        6150..=6449 => "B20",
        6450..=6599 => "B21",
        6600..=7399 => "B22",
        7500..=7699 => "B23",
        7700..=8039 => "B24",
        8040..=8689 => "B25",
        8690..=9039 => "B26",
        9210..=9659 => "B28",
        9660..=9769 => "B29",
        9770..=9869 => "B30",
        9870..=9919 => "B31",
        9920..=10359 => "B32",
        36000..=36199 => "B33",
        36200..=36349 => "B34",
        36350..=36949 => "B35",
        36950..=37549 => "B36",
        37550..=37749 => "B37",
        37750..=38249 => "B38",
        38250..=38649 => "B39",
        38650..=39649 => "B40",
        _ => "Unknown",
    }
}

pub fn hex_to_decimal(hex: &str) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(hex.trim_start_matches("0x"), 16)
}

// pub fn calculate_distance(timing_advance: &str) -> String {
//     let computed_value = (timing_advance * 78.125) / 1000.0;
//     let rounded_value = (computed_value * 1000.0).round() / 1000.0;
//
//     // Используем `Option::or` для выбора между значением и None
//     let distance = Some(rounded_value).or(None);
// }