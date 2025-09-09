use std::env;

fn main() {    
    let args: Vec<String> = env::args().collect();
    let target = args[1].clone();
    validate_target_argument(target);

    println!("Validated target argument");
}

fn validate_target_argument(target: String) {
    let octets: Vec<&str> = target.split('.').collect();
    let mut octet_index: u8 = 1;
    if octets.len() != 4 {
        panic!("Invalid number of octets");
    }

    for octet in octets {
        if octet.contains('-') {
            validate_range(octet, octet_index.clone());
        }
        else {
            validate_octet(octet, octet_index.clone());
        }
        octet_index += 1;
    }
}

fn validate_range(octet: &str, octet_index: u8) {
    let range: Vec<&str> = octet.split('-').collect();
    if range.len() != 2 {
        panic!("Invalid range detected in octet: {}", octet_index);
    }

    let range_from = range.first().expect(&("Start of range can not be empty in octet: ".to_string() + octet_index.to_string().as_str()));
    validate_octet(range_from, octet_index.clone());
    let range_to = range.last().expect(&("End of range can not be empty in octet: ".to_string() + octet.to_string().as_str()));
    validate_octet(range_to, octet_index.clone());
}

fn validate_octet(octet: &str, octet_index: u8) {
    let octet_value = octet.parse::<i32>().expect(&("Invalid number for octet: ".to_string() + octet_index.to_string().as_str()));
    let is_valid = octet_value >= 0 && octet_value <= 255;
    if !is_valid {
        panic!("Invalid octet detected at index: {}", octet_index);
    }
}