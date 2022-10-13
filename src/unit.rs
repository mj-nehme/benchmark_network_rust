fn to_metric(size: f64) -> String {
    const KILO: f64 = 1000 as f64;
    const MEGA: f64 = KILO * KILO;
    const GIGA: f64 = KILO * MEGA;
    const TERA: f64 = KILO * GIGA;
    const PETA: f64 = KILO * TERA;

    let unit: String;
    let formatted_size: f64;
    if size >= PETA {
        unit = String::from("P");
        formatted_size = size / PETA;
    } else if size >= TERA {
        unit = String::from("T");
        formatted_size = size / TERA;
    } else if size >= GIGA {
        unit = String::from("G");
        formatted_size = size / GIGA;
    } else if size >= MEGA {
        unit = String::from("M");
        formatted_size = size / MEGA;
    } else if size >= KILO {
        unit = String::from("K");
        formatted_size = size / KILO;
    } else {
        unit = String::from("");
        formatted_size = size;
    }
    format!("{formatted_size:.2}{unit}")
}

pub fn data_to_string(data_amount: f64) -> String {
    format!(" {}B", to_metric(data_amount))
}

pub fn throughput_to_string(throughput: f64) -> String {
    format!(" {}b/s", to_metric(throughput))
}
