//See https://www.consumerfinance.gov/eregulations/1003-C/2015-26607_20180101#1003-C-1

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref CONVERSION_TABLE: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("A", 10);
        m.insert("B", 11);
        m.insert("C", 12);
        m.insert("D", 13);
        m.insert("E", 14);
        m.insert("F", 15);
        m.insert("G", 16);
        m.insert("H", 17);
        m.insert("I", 18);
        m.insert("J", 19);
        m.insert("K", 20);
        m.insert("L", 21);
        m.insert("M", 22);
        m.insert("N", 23);
        m.insert("O", 24);
        m.insert("P", 25);
        m.insert("Q", 26);
        m.insert("R", 27);
        m.insert("S", 28);
        m.insert("T", 29);
        m.insert("U", 30);
        m.insert("V", 31);
        m.insert("W", 32);
        m.insert("X", 33);
        m.insert("Y", 34);
        m.insert("Z", 35);
        m
    };
}

pub fn validate_uli(uli: &str) -> Result<bool, String> {
    if !is_alphanumeric(uli) {
        Err(String::from("ULI is not alphanumeric"))
    } else if !uli_valid_length(uli) {
        Err(String::from(
            "ULI does not have valid length, must be between 23 and 45 characters",
        ))
    } else {
        Ok(calculate_mod(convert(uli)) == 1)
    }
}

pub fn generate_uli(loan_id: &str) -> Result<String, String> {
    let c = check_digit(loan_id);
    match c {
        Ok(s) => Ok(loan_id.to_owned() + &s),
        Err(error) => Err(error),
    }
}

pub fn check_digit(loan_id: &str) -> Result<String, String> {
    if !loan_id_valid_length(loan_id) {
        Err(String::from("Invalid loan id length"))
    } else if !is_alphanumeric(loan_id) {
        Err(String::from("Loan id not alphanumeric"))
    } else {
        let m = calculate_mod(convert(loan_id) * 100);
        let cd = calculate_check_digit(&m);
        Ok(string_length_two(cd))
    }
}

fn is_alphanumeric(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    }
    RE.is_match(text)
}

fn uli_valid_length(uli: &str) -> bool {
    let count = uli.chars().count();
    count >= 23 && count <= 45
}

fn loan_id_valid_length(loan_id: &str) -> bool {
    let count = loan_id.chars().count();
    count >= 21 && count <= 43
}

fn calculate_mod(i: i128) -> i128 {
    i % 97
}

fn convert_to_int(s: &str) -> String {
    let num = s.parse::<i128>();
    match num {
        Ok(i) => i.to_string(),
        Err(_) => CONVERSION_TABLE
            .get::<str>(&s.to_string())
            .unwrap()
            .to_string(),
    }
}

fn convert(text: &str) -> i128 {
    let v: Vec<String> = text
        .chars()
        .map(|c| c.to_string())
        .map(|c| c.to_uppercase())
        .collect();
    let m: Vec<String> = v.iter().map(|s| convert_to_int(&s)).collect();
    m.join("").parse::<i128>().unwrap()
}

fn calculate_check_digit(i: &i128) -> i128 {
    98 - i
}

fn string_length_two(n: i128) -> String {
    if n <= 9 && n >= 0 {
        "0".to_owned() + &n.to_string()
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref ULI: String = String::from("10Bx939c5543TqA1144M999143X38");
        static ref LOAN_ID: String = String::from("10Bx939c5543TqA1144M999143X");
        static ref SHORT_STRING: String = String::from("aaa");
    }

    #[test]
    fn test_conversion_table() {
        assert_eq!(CONVERSION_TABLE.get(&"A").unwrap(), &10);
        assert_eq!(CONVERSION_TABLE.get(&"M").unwrap(), &22);
        assert_eq!(CONVERSION_TABLE.get(&"Z").unwrap(), &35);
    }

    #[test]
    fn test_check_digit() {
        assert_eq!(check_digit(&LOAN_ID), Ok(String::from("38")));
    }

    #[test]
    fn test_validate_uli() {
        assert_eq!(validate_uli(&ULI), Ok(true));
    }

    #[test]
    fn test_is_alphanumeric() {
        assert_eq!(is_alphanumeric(&ULI), true);
    }

    #[test]
    fn test_uli_valid_length() {
        assert_eq!(uli_valid_length(&ULI), true);
        assert_eq!(uli_valid_length(&SHORT_STRING), false);
    }

    #[test]
    fn test_loan_id_valid_length() {
        assert_eq!(loan_id_valid_length(&LOAN_ID), true);
        assert_eq!(loan_id_valid_length(&SHORT_STRING), false);
    }

    #[test]
    fn test_calculate_mod() {
        assert_eq!(calculate_mod(1011339391255432926101144229991433300), 60);
    }

    #[test]
    fn test_convert() {
        assert_eq!(convert(&LOAN_ID), 10113393912554329261011442299914333);
    }

    #[test]
    fn test_string_length_two() {
        let i = 1;
        let i2 = 11;
        assert_eq!(string_length_two(i), "01");
        assert_eq!(string_length_two(i2), "11");
    }

}
