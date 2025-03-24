// https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher
// needed:
// list of values (a-z, A-Z, 0-9, symbols, space)
// enumerate this list
// get msg and passphrase, extend passphrase to length of msg
// turn both msg and extended passphrase into numbers, using enumerated list
// add every number in passphrase to corresponding number in msg (with wrap around)
// turn modified msg back to "normal" using list and return it
//
// decrypt by doing same thing, but subtracting

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
    let mut symbol_iter = SYMBOLS.iter();
    for c in s.chars() {
        // let pos = symbol_iter.position(|x| *x == c);
        if let Some(idx) = symbol_iter.position(|x| *x == c) {
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

pub fn encrypt(msg: String, pass: String) -> Result<String> {
    todo!()
}

pub fn decrypt(msg: String, pass: String) -> Result<String> {
    todo!()
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
