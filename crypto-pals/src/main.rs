#![allow(unused)]

use std::{cmp::max, collections::HashMap};

fn hex_char_to_val(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0',
        'a'..='f' => c as u8 - b'a' + 10,
        'A'..='F' => c as u8 - b'A' + 10,
        _ => unreachable!("Invalid hex-char"),
    }
}

fn val_to_hex_char(val: u8) -> char {
    match val {
        0..=9 => (b'0' + val) as char,
        10..=15 => (b'a' + (val - 10)) as char,
        _ => unreachable!("Invalid nibble"),
    }
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_str = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        hex_str.push(val_to_hex_char(b >> 4));
        hex_str.push(val_to_hex_char(b & 0x0f));
    }
    hex_str
}

fn base64_encode(input: &[u8]) -> String {
    const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity((input.len() + 2) / 3 * 4);

    let mut i = 0;
    while i < input.len() {
        let chunk = &input[i..(i + 3).min(input.len())];
        i += 3;

        let mut buffer = [0u8; 3];
        for (j, &b) in chunk.iter().enumerate() {
            buffer[j] = b;
        }

        let b0 = buffer[0] >> 2;
        let b1 = ((buffer[0] & 0b00000011) << 4) | (buffer[1] >> 4);
        let b2 = ((buffer[1] & 0b00001111) << 2) | (buffer[2] >> 6);
        let b3 = buffer[2] & 0b00111111;

        output.push(BASE64_TABLE.chars().nth(b0 as usize).unwrap());
        output.push(BASE64_TABLE.chars().nth(b1 as usize).unwrap());

        if chunk.len() > 1 {
            output.push(BASE64_TABLE.chars().nth(b2 as usize).unwrap());
        } else {
            output.push('=');
        }

        if chunk.len() == 3 {
            output.push(BASE64_TABLE.chars().nth(b3 as usize).unwrap());
        } else {
            output.push('=');
        }
    }

    output
}

fn hex_to_base64(hex_str: &str) -> String {
    let bytes = hex_to_bytes(hex_str);
    base64_encode(&bytes)
}

fn fixed_xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    assert!(b1.len() == b2.len());

    b1.into_iter()
        .zip(b2.into_iter())
        .map(|(one, two)| one ^ two)
        .collect()
}

fn character_frequency(b1: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in b1.chars() {
        // println!("{c}");
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}

fn xor_with_key(bytes: &[u8], key: u8) -> Vec<u8> {
    bytes.iter().map(|&b| b ^ key).collect()
}

fn score_plaintext(bytes: &[u8]) -> f32 {
    let text = String::from_utf8_lossy(bytes).to_lowercase();
    let mut freq = HashMap::new();
    for c in text.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let mut score = 0.0;
    for (c, count) in freq {
        if "etaoin shrdlu".contains(c) {
            score += count as f32 * 2.0;
        } else if c.is_ascii_alphabetic() {
            score += count as f32;
        } else if (c.is_ascii_whitespace() || c.is_ascii_punctuation()) {
            score -= count as f32 * 2.0;
        }
    }
    score
}

fn main() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_c1() {
        let st = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

        let base64 = hex_to_base64(st);

        assert_eq!(
            base64,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn test_c2() {
        let b1 = "1c0111001f010100061a024b53535009181c";
        let b1 = &hex_to_bytes(b1);
        println!("b1: {b1:?}");
        let b2 = "686974207468652062756c6c277320657965";
        let b2 = &hex_to_bytes(b2);

        let res = "746865206b696420646f6e277420706c6179";
        println!("b2: {b2:?}");

        let expected = bytes_to_hex(&fixed_xor(b1, b2));
        assert_eq!(expected, res);
    }

    #[test]
    fn test_c3() {
        let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let bytes = hex_to_bytes(hex_str);

        let mut best_score = f32::MIN;
        let mut best_key = 0;
        let mut best_plaintext = Vec::new();

        for key in 0..=255 {
            let candidate = xor_with_key(&bytes, key);
            let candidate_score = score_plaintext(&candidate);
            // println!(
            //     "Candidate: {} {}",
            //     candidate_score,
            //     String::from_utf8_lossy(&candidate)
            // );
            if candidate_score > best_score {
                best_score = candidate_score;
                best_key = key;
                best_plaintext = candidate;
            }
        }
        let decrypted_message = String::from_utf8_lossy(&best_plaintext);
        println!("Best key: {}", best_key as char);
        println!(
            "Decrypted message: {}",
            String::from_utf8_lossy(&best_plaintext)
        );
        assert_eq!(decrypted_message, "Cooking MC's like a pound of bacon");
    }
}
