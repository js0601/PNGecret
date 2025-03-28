use std::{error::Error, fmt::Display};

use crate::Result;

// just all symbols on my keyboard, don't care about anything else let's be real
// if this is changed and longer than 255, need to change u8 in string_to_numbers and numbers_to_string
const SYMBOLS: [char; 106] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'ä', 'ö', 'ü', 'ß', '0',
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '^', '°', '!', '"', '²', '§', '³', '$', '%', '&',
    '/', '{', '(', '[', ')', ']', '=', '}', '?', '\\', '´', '`', '@', '€', '+', '*', '~', '\'',
    '#', '<', '>', '|', 'µ', ',', ';', '.', ':', '-', '_', ' ',
];

/// takes a string and turns it into vec of corresponding numbers
fn string_to_numbers(s: &str) -> Result<Vec<u8>> {
    let mut vals: Vec<u8> = Vec::new();
    for c in s.chars() {
        if let Some(idx) = SYMBOLS.iter().position(|x| *x == c) {
            // NOTE: panics if more than 255 symbols in SYMBOLS
            vals.push(idx.try_into().unwrap());
        } else {
            return Err(Box::new(CryptoError::UnknownChar(c)));
        }
    }
    Ok(vals)
}

/// reverse of string_to_numbers
fn numbers_to_string(vals: Vec<u8>) -> String {
    let mut s: String = String::new();
    for i in vals {
        s.push(SYMBOLS[i as usize]);
    }
    s
}

/// encrypt msg using pass and return encrypted msg
pub fn encrypt(msg: &str, pass: &str) -> Result<String> {
    let msg_vals = string_to_numbers(msg)?;
    let pass_vals = string_to_numbers(pass)?;
    let mut encrypted_msg_vals = Vec::new();

    // use pass_idx to repeatedly iterate over pass_vals
    let mut pass_idx = 0;
    for v in msg_vals {
        // NOTE: if SYMBOLS has more than 255 chars this panics, change u8
        encrypted_msg_vals.push((v + pass_vals[pass_idx]) % SYMBOLS.len() as u8);
        // update pass_idx
        pass_idx = (pass_idx + 1) % pass_vals.len();
    }

    Ok(numbers_to_string(encrypted_msg_vals))
}

pub fn decrypt(msg: &str, pass: &str) -> Result<String> {
    let msg_vals = string_to_numbers(msg)?;
    let pass_vals = string_to_numbers(pass)?;
    let mut encrypted_msg_vals = Vec::new();

    // use pass_idx to repeatedly iterate over pass_vals
    let mut pass_idx = 0;
    for v in msg_vals {
        // NOTE: if SYMBOLS has more than 255 chars this panics, change u8
        // not very pretty? but works
        // basically see if subtraction would overflow, if yes subtract (u8::MAX - SYMBOLS.len()+1) from result
        if let (n, true) = v.overflowing_sub(pass_vals[pass_idx]) {
            encrypted_msg_vals.push(n - (u8::MAX - SYMBOLS.len() as u8 + 1));
        } else {
            encrypted_msg_vals.push((v - pass_vals[pass_idx]) % SYMBOLS.len() as u8);
        }
        // update pass_idx
        pass_idx = (pass_idx + 1) % pass_vals.len();
    }

    Ok(numbers_to_string(encrypted_msg_vals))
}

#[derive(Debug)]
enum CryptoError {
    UnknownChar(char),
}

impl Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::UnknownChar(c) => {
                write!(
                    f,
                    "Couldn't encrypt or decrypt the message, because {c} is unknown."
                )
            }
        }
    }
}

impl Error for CryptoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_numbers() {
        let actual = string_to_numbers("Hello World!").unwrap();
        let expected = vec![33, 4, 11, 11, 14, 105, 48, 14, 17, 11, 3, 68];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_unknown_char() {
        let result = string_to_numbers("é");

        assert!(result.is_err());
    }

    #[test]
    fn test_numbers_to_string() {
        let actual = numbers_to_string(vec![33, 4, 11, 11, 14, 105, 48, 14, 17, 11, 3, 68]);
        let expected = "Hello World!";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encrypt() {
        let actual = encrypt("Hello World!", "password").unwrap();
        let expected = "WeDDKn9rGlv´";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decrypt() {
        let actual = decrypt("WeDDKn9rGlv´", "password").unwrap();
        let expected = "Hello World!";

        assert_eq!(actual, expected);
    }
}
