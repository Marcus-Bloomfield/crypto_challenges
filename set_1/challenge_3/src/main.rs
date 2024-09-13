use hex;

fn main() {
    decipher_message("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
}

const LETTER_FREQ: [f64; 27] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.19181, // V-Z & space char
];

pub fn calc_letter_freq_score(s: &str) -> f64 {
    let mut counts = vec![0_u32; 27];
    let mut score: f64 = 0_f64;

    s.chars().for_each(|c| match c {
        'a'..='z' => {
            counts[c as usize - 97] += 1;
        }
        'A'..='Z' => {
            counts[c as usize - 65] += 1;
        }
        ' ' => counts[26] += 1,
        _ => {}
    });

    for i in 0..27{
        score += (counts[i] as f64) * LETTER_FREQ[i];
    }

    score
}

pub fn decipher_message(hex_cipher: &str) -> (String, f64) {
    let cypher_bytes = hex::decode(hex_cipher).unwrap();
    let mut key_byte: u8;

    let mut message = String::new();
    let mut best_score = f64::MIN;

    for c in 0..=255 {
        key_byte = c as u8;

        let msg_bytes: Vec<u8> = cypher_bytes.iter().map(|&b| b ^ key_byte).collect();

        let msg = String::from_utf8_lossy(&msg_bytes);
        let score = calc_letter_freq_score(&msg);

        if score > best_score {
            best_score = score;
            message = String::from(msg);
        }
    }

    (message, best_score)
    
}
