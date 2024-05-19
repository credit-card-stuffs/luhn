/*
 * Original code: https://github.com/cybermatt/fast-luhn/blob/master/src/lib.rs
 * Author: @cybermatt
 * Editor: @0x41337
*/
pub mod utils;

use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

const RADIX: u32 = 10;

#[wasm_bindgen]
pub fn checksum(string: &str) -> u32 {
    let mut fsum: u32 = 0;
    let length = string.chars().count();

    for (idx, c) in string.chars().enumerate() {
        if c.is_digit(RADIX) == false {
            continue;
        }
        let mut dint = c.to_digit(RADIX).unwrap();
        if (length - idx) % 2 == 0 {
            dint *= 2;
            if dint > 9 {
                dint -= 9;
            }
        }
        fsum += dint;
    }
    return fsum;
}

#[wasm_bindgen]
pub fn validate(string: &str) -> bool {
    checksum(&string) % 10 == 0
}

#[wasm_bindgen]
pub fn digit(string: &str) -> u32 {
    let mut valid_string = string.to_string();
    valid_string.push_str("0");
    let chsum = checksum(&valid_string);
    let x = 10 - (chsum % 10);
    let res = if x == 10 { 0 } else { x };
    res
}

#[wasm_bindgen]
pub fn complete(string: &str) -> String {
    let mut valid_string = string.to_string();
    let digit = digit(string);
    valid_string.push_str(&digit.to_string());
    valid_string
}

#[wasm_bindgen]
pub fn generate(length: i32) -> String {
    let len = if length > 1 { length } else { 0 };
    let mut rng = thread_rng();
    let mut new_string = String::new();
    for _ in 0..len - 1 {
        let n1: u8 = rng.gen_range(0..10);
        new_string.push_str(&n1.to_string());
    }
    complete(&new_string)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_digit() {
        assert_eq!(digit("47162930944"), 7);
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("1234567890"), 43);
        assert_eq!(checksum("5457210001000019"), 30);
        assert_eq!(checksum("5457210001000043"), 30);
        assert_eq!(checksum("4716293094402"), 50);
        assert_eq!(checksum("0"), 0);
        assert_eq!(checksum("1"), 1);
    }

    #[test]
    fn test_validate() {
        assert_eq!(validate("5152480083848100"), true);
        assert_eq!(validate("491674804530447"), false);
    }

    #[test]
    fn test_complete() {
        assert_eq!(complete("524402422676340"), "5244024226763402");
        assert_eq!(complete("530449279124605"), "5304492791246052");
    }

    #[test]
    fn test_generate() {
        let mut rng = thread_rng();
        for _ in 1..1000 {
            let length = rng.gen_range(1..20);
            let ns = generate(length);
            assert_eq!(ns.len(), length as usize);
            assert_eq!(checksum(&ns) % 10, 0);
        }
    }
}
