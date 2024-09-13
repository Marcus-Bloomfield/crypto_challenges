use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
use hex;

fn main() {
    break_repeating_key_xor(".\\challenge_6.txt");
}

fn read_bytes(path: &str) -> Vec<u8> {
    let base64_s = std::fs::read_to_string(path)
        .and_then(|res| Ok(res.replace("\n", "")))
        .expect("Error reading file");
    general_purpose::STANDARD.decode(base64_s).unwrap()
}

pub fn break_repeating_key_xor(path: &str) -> String {
    let text_bytes = read_bytes(path);

    let mut edit_dist: Vec<(usize, f64)> = Vec::new();

    for key_sz in 2..=40 {
        let dist = calc_avg_edit_dist(key_sz, &text_bytes);
        edit_dist.push((key_sz, dist));
    }

    edit_dist.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
    let key_sz = edit_dist.pop().and_then(|x| Some(x.0)).unwrap();

    let mut key_bytes: Vec<u8> = Vec::new();

    let mut idx;
    let mut ith_bytes: Vec<u8> = Vec::new();
    for i in 0..key_sz {

        idx = i;
        ith_bytes.clear();
        while idx < text_bytes.len() {
            ith_bytes.push(text_bytes[idx]);
            idx += key_sz;
        }

        let key_i = break_single_char_xor(&ith_bytes);
        key_bytes.push(key_i);
    }

    let key: String = key_bytes.iter().map(|&b| b as char).collect();

    key

}

pub fn hamming_distance_bytes(bytes_1: &[u8], bytes_2: &[u8]) -> u32 {
    if bytes_1.len() != bytes_2.len() {
        panic!("byte sizes are not equal.")
    }

    bytes_1.iter().zip(bytes_2.iter()).fold(0_u32, |dist, (x1, x2)| {
        let bin1 = format!("{:08b}", x1);
        let bin2 = format!("{:08b}", x2);

        dist + bin1
            .chars()
            .zip(bin2.chars())
            .fold(0_u32, |d, (char1, char2)| if char1 == char2 {d} else {d + 1})
    })
}

pub fn calc_avg_edit_dist(key_sz: usize, txt_bytes: &[u8]) -> f64 {
    let len = txt_bytes.len();
    let mut i: usize = 0;
    let mut dist_sum = 0;
    let mut block_1;
    let mut block_2;

    loop {
        if i * 2 * key_sz >= len {
            break;
        }

        block_1 = &txt_bytes[i * key_sz..(i + 1) * key_sz];
        block_2 = &txt_bytes[(i + 1) * key_sz..(i + 2) * key_sz];
        
        dist_sum += hamming_distance_bytes(block_1, block_2) / (key_sz as u32);

        i += 1
    }

    (dist_sum as f64) / (i as f64 + 1.0)
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

pub fn break_single_char_xor(hex_cipher: &[u8]) -> u8 {
    let cypher_bytes = hex::decode(hex_cipher).unwrap();
    let mut key_byte: u8 = 0;

    let mut best_score = f64::MIN;

    for c in 0..=255 {
        let msg_bytes: Vec<u8> = cypher_bytes.iter().map(|&b| b ^ key_byte).collect();

        let msg = String::from_utf8_lossy(&msg_bytes);
        let score = calc_letter_freq_score(&msg);

        if score > best_score {
            best_score = score;
            key_byte = c
        }
    }

    key_byte
    
}