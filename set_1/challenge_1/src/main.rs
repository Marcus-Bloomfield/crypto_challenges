use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use hex;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    if (convert_hex_to_base64(input) == "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"){
        println!("good")
    }
    else {
        println!("bad")
    }
}

pub fn convert_hex_to_base64(hex_num: &str) -> String {
    general_purpose::STANDARD.encode(hex::decode(hex_num).unwrap())
}