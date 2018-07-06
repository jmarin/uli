// See https://www.consumerfinance.gov/eregulations/1003-C/2015-26607_20180101#1003-C-1

#[macro_use]
extern crate lazy_static;
//extern crate regex;

//use regex::Regex;
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

//fn is_alphanumeric(text: &str) -> bool {
//    lazy_static! {
//        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
//    }
//    RE.is_match(text)
//}

fn uli_valid_length(uli: &str) -> bool {
    let count = uli.chars().count();
    count >= 23 && count <= 45
}

fn loan_id_valid_length(loan_id: &str) -> bool {
    let count = loan_id.chars().count();
    count >= 21 && count <= 43
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref ULI: String = String::from("10Bx939c5543TqA1144M999143X38");
    }

    #[test]
    fn test_conversion_table() {
        assert_eq!(CONVERSION_TABLE.get(&"A").unwrap(), &10);
        assert_eq!(CONVERSION_TABLE.get(&"M").unwrap(), &22);
        assert_eq!(CONVERSION_TABLE.get(&"Z").unwrap(), &35);
    }

    //    #[test]
    //    fn test_is_alphanumeric() {
    //        assert_eq!(is_alphanumeric(&ULI), true);
    //    }

    #[test]
    fn test_uli_valid_length() {
        assert_eq!(uli_valid_length(&ULI), true);

        let bad_uli = String::from("aaa");
        assert_eq!(uli_valid_length(&bad_uli), false);
    }

    fn test_loan_id_valid_length() {
        let loan_id = "10Bx939c5543TqA1144M999143X";
        assert_eq!(loan_id_valid_length(loan_id), true);

        let bad_loan_id = String::from("aaa");
        assert_eq!(loan_id_valid_length(&bad_loan_id), false);
    }

}
