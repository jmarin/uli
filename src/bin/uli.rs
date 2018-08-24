#[macro_use]
extern crate structopt;
extern crate uli_lib;

use structopt::StructOpt;
use uli_lib::validate_uli;

#[derive(StructOpt, Debug)]
#[structopt(name = "uli")]
struct Opt {
   #[structopt(long = "validate")]
   uli: String,
}


fn main() {
    let opt = Opt::from_args();
    let uli = opt.uli;
    let uli_length = uli.chars().count();

    if uli_length > 0 {
        let uli_check = validate_uli(&uli);
        match uli_check {
            Ok(is_valid) => //println!("{}", valid),
                match is_valid {
                  true => println!("ULI {} is valid", &uli),    
                  false => println!("ULI {} is not valid", &uli),  
                }
            Err(error) => println!("{}", error),
        }
    }
}
