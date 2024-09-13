use hex;
fn main() {
    let encrypted_message = key_encryption("Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal", "ICE");
    if "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f" == encrypted_message{
        println!("{}", encrypted_message)
}
    
}

pub fn key_encryption(message: &str, key: &str) -> String {
    let sequence: String = key.chars().cycle().take(message.len()).collect::<String>();
    
    let key_bytes = sequence.as_bytes();
    let message_bytes = message.as_bytes();

    let encrypted_message: Vec<u8> = message_bytes
        .iter()
        .zip(key_bytes.iter())
        .map(|(&message_byte, &key_byte)| message_byte ^ key_byte )
        .collect();

    hex::encode(encrypted_message)
}
