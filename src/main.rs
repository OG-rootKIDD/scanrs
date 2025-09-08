use std::env;

fn main() {    
    let args: Vec<String> = env::args().collect();
    let target = args[1].clone();
    validate_target_argument(target);

    println!("Validated target argument");
}

fn validate_target_argument(target: String) {
    let octets: Vec<&str> = target.split('.').collect();
    for octet in octets {
        if octet.contains('-') {
            validate_range(octet);
        }
        else {
            validate_octet(octet);
        }      
    }
}

fn validate_range(octet: &str) {
    let range: Vec<&str> = octet.split('-').collect();
    if range.len() != 2 {
        panic!("Invalid range detected!");
    }

    let range_from = range.first().expect("Invalid start of range detected!");
    validate_octet(range_from);
    let range_to = range.last().expect("Invalid end of range detected!");
    validate_octet(range_to);
}

fn validate_octet(octet: &str) {
    let is_valid = octet.parse::<i32>().is_ok();
    if !is_valid {
        panic!("Invalid octet detected!");
    }
}