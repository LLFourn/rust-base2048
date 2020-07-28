#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![no_std]

#[macro_use]
extern crate alloc;
use alloc::{string::String, vec::Vec};

pub const ENC_TABLE: &'static [char; 2048] = &include!("./enc_table.src");
pub const DEC_TABLE: &'static [u16; 4347] = &include!("./dec_table.src");
pub const TAIL: &'static [char; 8] = &['།', '༎', '༏', '༐', '༑', '༆', '༈', '༒'];
/// Encode some bytes using base2048 encoding
///
/// # Example
/// ```
/// let some_bytes = b"some utf8 bytes to encode more compactly";
/// assert_eq!(
///     base2048::encode(some_bytes),
///     "ظڝөПәΰศƪڍথԅПяޒߡೲױɡјවၕάێנלۻӥ༟ಖ༎"
/// );
/// ```
pub fn encode(bytes: &[u8]) -> String {
    let mut ret = String::new();
    let mut stage = 0x0000u16;
    let mut remaining = 0;

    for byte in bytes {
        let byte = *byte as u16;
        // how many more bits do we need to complete the next character?
        let need = 11 - remaining;
        if need <= 8 {
            // if we need a byte or less then take what we need and push it
            remaining = 8 - need;
            let index = (stage << need) | (byte >> remaining);
            ret.push(ENC_TABLE[index as usize]);
            // put what remains in stage
            stage = byte & ((1 << remaining) - 1);
        } else {
            // we need more than a byte so just shift it into stage
            stage = (stage << 8) | byte;
            remaining += 8;
        }
    }

    // there are some bits that haven't been put into the string
    // (happens whenever 8 * bytes.len() is not divisible by 11).
    if remaining > 0 {
        // We need to disambiguate between a terminating character conveying =< 3 or > 8 bits.
        // e.g. is this character just finishing the last byte or is it doing that and adding another byte.
        if remaining <= 3 {
            // we're adding 1-3 bits so add special tail character
            ret.push(TAIL[stage as usize]);
        } else {
            // we're adding > 3 bits no need for a tail since it's not ambigious
            ret.push(ENC_TABLE[stage as usize])
        }
    }

    ret
}

/// Decode a base2048 encoded string
/// # Example
/// ```
/// let encoded_message = "ظڝөПәΰศƪڍথԅПяޒߡೲױɡјවၕάێנלۻӥ༟ಖ༎";
/// assert_eq!(
///     base2048::decode(encoded_message),
///     Some(b"some utf8 bytes to encode more compactly".to_vec())
/// );
/// ```
pub fn decode(string: &str) -> Option<Vec<u8>> {
    let mut ret = vec![];
    let mut remaining = 0u8;
    let mut stage = 0x00u32;
    let mut chars = string.chars().peekable();
    let mut residue = 0;

    while let Some(c) = chars.next() {
        // keep track of the misalignment between byte boundary.  This is useful when we get to the
        // last character and it's NOT a tail character.
        residue = (residue + 11) % 8;
        let (n_new_bits, new_bits) = match DEC_TABLE[c as usize] {
            0xFFFF => {
                if chars.peek().is_some() {
                    return None;
                }

                match TAIL.iter().enumerate().find(|(_, t)| *t == &c) {
                    // so we're at the last character and it's a tail character
                    Some((index, _)) => {
                        let need = 8 - remaining;
                        if index < (1 << need) {
                            (need, index as u16)
                        } else {
                            return None;
                        }
                    }
                    None => return None,
                }
            }
            new_bits => {
                if chars.peek().is_none() {
                    (11 - residue, new_bits)
                } else {
                    (11, new_bits)
                }
            }
        };

        remaining += n_new_bits;
        stage = (stage << n_new_bits) | new_bits as u32;
        while remaining >= 8 {
            //NOTE: This loop runs at most twice
            remaining -= 8;
            ret.push((stage >> remaining) as u8);
            stage = stage & ((1 << remaining) - 1)
        }
    }

    if remaining > 0 {
        ret.push((stage >> (8 - remaining)) as u8)
    }

    Some(ret)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn encode_decode_roundtrip() {
        for tv in &[
            vec![],
            vec![0],
            vec![216, 110, 27, 35, 23, 49, 153, 7, 161, 234, 63],
            vec![0b11111111, 0b11111111],
            vec![
                0b11111000, 0b00011111, 0b00000011, 0b11100000, 0b01111100, 0b00001111, 0b10000001,
                0b11110000, 0b00111110, 0b00000111, 0b11000000,
            ],
            vec![
                0b11111000, 0b00011111, 0b00000011, 0b11100000, 0b01111100, 0b00001111, 0b10000001,
                0b11110000, 0b00111110, 0b00000111, 0b11000000, 0b11111000,
            ],
            vec![0b10101010; 3],
            vec![0b10101010; 7],
            vec![0b10101010; 10],
            vec![0b00000000; 3],
            vec![0b00000000; 7],
            vec![0b00000000; 10],
            vec![0b11111111; 3],
            vec![0b11111111; 7],
            vec![0b11111111; 10],
            vec![0b01010101; 3],
            vec![0b01010101; 7],
            vec![0b01010101; 10],
            vec![0b11110100; 3],
            vec![0b11110100; 7],
            vec![0b11110100; 10],
            vec![0b11110110; 3],
            vec![0b11110110; 7],
            vec![0b11110110; 10],
            vec![0b11110001; 3],
            vec![0b11110001; 7],
            vec![0b11110001; 10],
        ] {
            let encoded = encode(&tv[..]);
            let decoded = decode(&encoded).unwrap();
            assert_eq!(tv[..], decoded[..]);
        }
    }

    #[test]
    fn wrong_tail_character() {
        assert!(decode("ၐߥۻ༞ɵ༎").is_some());
        // this is a valid tail character but conveys too many bits.
        assert!(decode("ၐߥۻ༞ɵ༑").is_none());
        // these are both invalid
        assert!(decode("ၐߥۻ༞ɵX").is_none());
        assert!(decode("ၐߥۻ༞ɵ༎X").is_none());
    }
}
