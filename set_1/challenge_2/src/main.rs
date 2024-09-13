use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use hex;
use std::u8;

fn main() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";

    let bytes1 = hex::decode(input1).unwrap();
    let bytes2 = hex::decode(input2).unwrap();

    let the_final_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();

    let result = hex::encode(the_final_bytes);

    if (result == "746865206b696420646f6e277420706c6179"){
        println!("good")
    }
    else {
        println!("bad")
    }
}
