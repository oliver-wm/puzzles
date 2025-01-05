#![allow(unused)]

use std::{
    cmp::max,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
};

use aws_lc_rs::{
    cipher::{
        self, AES_128, Algorithm, DecryptingKey, DecryptionContext, EncryptionContext,
        PaddedBlockDecryptingKey, PaddedBlockEncryptingKey, UnboundCipherKey,
    },
    iv::{FixedLength, IV_LEN_128_BIT},
};

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
    hex_str.shrink_to_fit();
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
fn base64_decode(input: &str) -> Vec<u8> {
    const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut decode_map = [0u8; 256];
    for (i, c) in BASE64_TABLE.chars().enumerate() {
        decode_map[c as usize] = i as u8;
    }

    // IMPORTANT: filter out newlines, spaces, etc.
    let mut chars_iter = input.chars().filter(|c| !c.is_whitespace()).peekable();

    let mut output = Vec::with_capacity(input.len() / 4 * 3);

    while chars_iter.peek().is_some() {
        let mut chunk = ['='; 4];
        for i in 0..4 {
            match chars_iter.next() {
                Some(c) => chunk[i] = c,
                None => break,
            }
        }

        let b0 = if chunk[0] == '=' {
            0
        } else {
            decode_map[chunk[0] as usize]
        };
        let b1 = if chunk[1] == '=' {
            0
        } else {
            decode_map[chunk[1] as usize]
        };
        let b2 = if chunk[2] == '=' {
            0
        } else {
            decode_map[chunk[2] as usize]
        };
        let b3 = if chunk[3] == '=' {
            0
        } else {
            decode_map[chunk[3] as usize]
        };

        let out0 = (b0 << 2) | (b1 >> 4);
        let out1 = ((b1 & 0x0f) << 4) | (b2 >> 2);
        let out2 = ((b2 & 0x03) << 6) | b3;

        output.push(out0);

        if chunk[2] != '=' {
            output.push(out1);
        }
        if chunk[3] != '=' {
            output.push(out2);
        }
    }

    output
}

fn base64_decode1(input: &str) -> Vec<u8> {
    const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut decode_map = [0u8; 256];
    for (i, c) in BASE64_TABLE.chars().enumerate() {
        decode_map[c as usize] = i as u8;
    }

    let mut output = Vec::with_capacity(input.len() / 4 * 3);
    let mut chars_iter = input.chars().filter(|c| !c.is_whitespace()).peekable();

    while chars_iter.peek().is_some() {
        let mut chunk = ['='; 4];
        for i in 0..4 {
            match chars_iter.next() {
                Some(c) => chunk[i] = c,
                None => break,
            }
        }

        let b0 = if chunk[0] == '=' {
            0
        } else {
            decode_map[chunk[0] as usize]
        };
        let b1 = if chunk[1] == '=' {
            0
        } else {
            decode_map[chunk[1] as usize]
        };
        let b2 = if chunk[2] == '=' {
            0
        } else {
            decode_map[chunk[2] as usize]
        };
        let b3 = if chunk[3] == '=' {
            0
        } else {
            decode_map[chunk[3] as usize]
        };

        let out0 = (b0 << 2) | (b1 >> 4);
        let out1 = ((b1 & 0b1111) << 4) | (b2 >> 2);
        let out2 = ((b2 & 0b0011) << 6) | b3;

        output.push(out0);

        if chunk[2] != '=' {
            output.push(out1);
        }
        if chunk[3] != '=' {
            output.push(out2);
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

    b1.iter()
        .zip(b2)
        .map(|(one, two)| one ^ two)
        .collect()
}

fn count_1s(b1: &[u8]) -> usize {
    let mut c = 0;
    for b in b1 {
        for i in 0..8 {
            if *b & (1 << i) != 0 {
                c += 1;
            }
        }
    }
    c
}

fn edit_distance(b1: &[u8], b2: &[u8]) -> usize {
    // TODO: optimize
    count_1s(&fixed_xor(b1, b2))
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

fn rolling_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
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
            score -= count as f32 * 0.5;
        }
    }
    score
}

fn read_and_decode_b64(path: &str) -> Vec<u8> {
    let f = File::open(path).expect("Could not open file");
    let mut reader = BufReader::new(f);
    let mut b64 = String::new();
    reader
        .read_to_string(&mut b64)
        .expect("Could not read file");
    base64_decode(&b64)
}

fn read_and_decode_hex(path: &str) -> Vec<Vec<u8>> {
    let f = File::open(path).expect("Could not open file");
    let mut reader = BufReader::new(f);
    let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let dehexed = hex_to_bytes(&line);
        vec.push(dehexed);
    }
    vec
}

fn find_best_key_size(raw: &[u8]) -> usize {
    let mut best_key_size = 0;
    let mut best_norm_dist = f64::MAX;
    let n_blocks = 8;

    for key_size in 2..=40 {
        if (n_blocks * key_size) > raw.len() {
            break;
        }

        let mut total = 0.0;
        let mut blocks_counted = 0;

        for i in 0..(n_blocks - 1) {
            for j in (i + 1)..n_blocks {
                let start1 = i * key_size;
                let start2 = j * key_size;
                if start2 + key_size <= raw.len() {
                    let block1 = &raw[start1..start1 + key_size];
                    let block2 = &raw[start2..start2 + key_size];
                    total += edit_distance(block1, block2) as f64;
                    blocks_counted += 1;
                }
            }
        }

        if blocks_counted > 0 {
            let avg_dist = total / blocks_counted as f64;
            let norm_dist = avg_dist / key_size as f64;
            if norm_dist < best_norm_dist {
                best_norm_dist = norm_dist;
                best_key_size = key_size;
            }
        }
    }

    println!("Smallest normalized distance: {}", best_norm_dist);
    println!("Likely key size: {}", best_key_size);
    best_key_size
}

fn transpose_blocks(raw: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    let blocks = raw.chunks(key_size);
    let mut transpose = vec![Vec::new(); key_size];

    for block in blocks {
        for (i, &byte) in block.iter().enumerate() {
            transpose[i].push(byte);
        }
    }
    transpose
}

fn crack_columns(transposed_blocks: Vec<Vec<u8>>) -> (Vec<char>, Vec<Vec<u8>>) {
    let mut keys = Vec::new();
    let mut decrypted_columns = Vec::new();

    for block in transposed_blocks {
        let mut best_score = f32::MIN;
        let mut best_key = 0;
        let mut best_plaintext = Vec::new();

        for candidate_key in 0..=255 {
            let candidate = xor_with_key(&block, candidate_key);
            let candidate_score = score_plaintext(&candidate);
            if candidate_score > best_score {
                best_score = candidate_score;
                best_key = candidate_key;
                best_plaintext = candidate;
            }
        }

        // println!("Best key: {}", best_key as char);
        keys.push(best_key as char);
        decrypted_columns.push(best_plaintext);
    }

    (keys, decrypted_columns)
}

fn reassemble_text(decrypted_columns: &[Vec<u8>]) -> Vec<u8> {
    let mut final_plaintext = Vec::new();
    let max_len = decrypted_columns
        .iter()
        .map(|col| col.len())
        .max()
        .unwrap_or(0);

    for row_index in 0..max_len {
        for col_index in 0..decrypted_columns.len() {
            if row_index < decrypted_columns[col_index].len() {
                final_plaintext.push(decrypted_columns[col_index][row_index]);
            }
        }
    }

    final_plaintext
}

fn main() {
    println!("Terminator X: Bring the noise");
}

#[cfg(test)]
mod tests {

    use crate::*;
    #[test]
    fn test_aes_128_ebc_round_trip() {
        let key_text = "YELLOW SUBMARINE";
        let key_bytes = key_text.as_bytes();

        let original_message = b"This is a secret message!";
        let mut buffer = Vec::from(&original_message[..]);

        let unbound_key = UnboundCipherKey::new(&AES_128, key_bytes).unwrap();
        let encrypting_key = PaddedBlockEncryptingKey::ecb_pkcs7(unbound_key).unwrap();

        let ciphertext = encrypting_key.encrypt(&mut buffer).unwrap();
        let unbound_key = UnboundCipherKey::new(&AES_128, key_bytes).unwrap();
        let decrypting_key = PaddedBlockDecryptingKey::ecb_pkcs7(unbound_key).unwrap();
        let mut in_out_buffer = buffer;
        let plaintext = decrypting_key
            .decrypt(&mut in_out_buffer, DecryptionContext::None)
            .unwrap();

        assert_eq!(
            "This is a secret message!",
            String::from_utf8_lossy(plaintext)
        );
    }

    #[test]
    fn test_hamming() {
        let s1 = "this is a test".to_string();
        let s2 = "wokka wokka!!!".to_string();

        let r = edit_distance(s1.as_bytes(), s2.as_bytes());

        assert_eq!(37, r);
    }

    #[test]
    fn test_rolling_xor() {
        let b: Vec<u8> = vec![1, 1, 0, 0];
        let k: Vec<u8> = vec![1, 0];

        assert_eq!(rolling_xor(&b, &k), vec![0, 1, 1, 0]);
    }

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

    #[test]
    fn test_c4() {
        let f = File::open("./static/4.txt").expect("open");
        let f = BufReader::new(f);

        let mut best_score = f32::MIN;

        let mut best_key = 0;
        let mut best_plaintext = Vec::new();
        for line in f.lines() {
            let hex_str = line.expect("line?");
            let bytes = hex_to_bytes(&hex_str);

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
        }
        let decrypted_message = String::from_utf8_lossy(&best_plaintext);
        println!("Best key: {}", best_key as char);
        println!(
            "Decrypted message: {}",
            String::from_utf8_lossy(&best_plaintext)
        );
        assert_eq!(decrypted_message, "Now that the party is jumping\n");
    }

    #[test]
    fn test_c5() {
        let mut s1 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
            .to_string();

        let ice = "ICE";
        let xrs1 = rolling_xor(s1.as_bytes(), ice.as_bytes());

        let res = bytes_to_hex(&xrs1);
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(res, expected);
    }

    #[test]
    fn test_c6() {
        let raw = read_and_decode_b64("./static/6.txt");

        let best_key_size = find_best_key_size(&raw);

        let transposed = transpose_blocks(&raw, best_key_size);

        let (key_chars, decrypted_columns) = crack_columns(transposed);

        let final_plaintext = reassemble_text(&decrypted_columns);

        {
            let mut file = File::create("s1-c6.txt").expect("Could not create output file");
            file.write_all(&final_plaintext).expect("Write failed");
        }

        println!("Discovered key: {}", key_chars.iter().collect::<String>());
        println!(
            "Decrypted message:\n{}",
            String::from_utf8_lossy(&final_plaintext)
        );
    }

    #[test]
    fn test_c7() {
        let key_text = "YELLOW SUBMARINE";
        let key_bytes = key_text.as_bytes();

        let raw = read_and_decode_b64("./static/7.txt");

        let mut buffer = Vec::from(&raw[..]);

        let unbound_key = UnboundCipherKey::new(&AES_128, key_bytes).unwrap();
        let decrypting_key = PaddedBlockDecryptingKey::ecb_pkcs7(unbound_key).unwrap();
        let mut in_out_buffer = buffer;
        let plaintext = decrypting_key
            .decrypt(&mut in_out_buffer, DecryptionContext::None)
            .unwrap();

        println!("Decrypted: {}", String::from_utf8_lossy(plaintext));
    }

    #[test]
    fn test_c8() {
        let key_text = "YELLOW SUBMARINE";
        let key_bytes = key_text.as_bytes();

        let raw = read_and_decode_hex("./static/8.txt");

        for (i, cipher) in raw.iter().enumerate() {
            let mut buffer = Vec::from(&cipher[..]);
            let unbound_key = UnboundCipherKey::new(&AES_128, key_bytes).unwrap();
            let decrypting_key = PaddedBlockDecryptingKey::ecb_pkcs7(unbound_key).unwrap();
            let mut in_out_buffer = buffer;
            let plaintext = decrypting_key.decrypt(&mut in_out_buffer, DecryptionContext::None);

            if let Ok(plaintext) = plaintext {
                println!(
                    "Detected AES in ECB mode! Line: {} Text: {}",
                    i,
                    String::from_utf8_lossy(plaintext)
                );
                assert_eq!(i, 166);
            }
        }
    }
}
