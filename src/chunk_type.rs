use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

// methods for checking properties of chunk type
// see section 3.3 of PNG spec for more detailed info
//
// the 5th bit of every byte has a specific meaning
// this could be checked by checking if the byte is an uppercase (bit 5 is 0)
// or lowercase (bit 5 is 1) ASCII char, but the spec says that's incorrect to do so I check the bits manually
#[allow(dead_code)] // NOTE: needed?
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    // NOTE: does this need to exist?
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    // bit 5 of byte 1 decides if the chunk is critical (0) or ancillary (1) (safe to ignore)
    pub fn is_critical(&self) -> bool {
        (self.bytes[0] & (1 << 5)) == 0
    }

    // bit 5 of byte 2 decides if chunk is public (0) or private (1)
    pub fn is_public(&self) -> bool {
        (self.bytes[1] & (1 << 5)) == 0
    }

    // bit 5 of byte 3 is reserved and should always be 0
    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.bytes[2] & (1 << 5)) == 0
    }

    // bit 5 of byte 4 decides if chunk is unsafe (0) or safe (1) to copy
    pub fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] & (1 << 5)) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        for b in bytes {
            if !b.is_ascii_alphabetic() {
                return Err(Box::new(ChunkTypeError::NonAsciiAlphabeticByte(b)));
            }
        }
        Ok(ChunkType { bytes })
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::new(ChunkTypeError::BadLength(s.len())));
        }
        let bytes = s.as_bytes();
        for b in bytes {
            if !b.is_ascii_alphabetic() {
                return Err(Box::new(ChunkTypeError::NonAsciiAlphabeticByte(*b)));
            }
        }
        Ok(ChunkType {
            bytes: bytes.try_into()?,
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes).unwrap())
    }
}

// Error type for nicer error messages
#[derive(Debug)]
enum ChunkTypeError {
    // byte is not an ascii alphabetic character
    NonAsciiAlphabeticByte(u8),
    // type is not 4 ascii characters long
    BadLength(usize),
}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::NonAsciiAlphabeticByte(byte) => write!(
                f,
                "Non-ASCII or non-alphabetic byte: {byte} ({byte:b}) is not a valid character"
            ),
            ChunkTypeError::BadLength(l) => write!(f, "Bad chunk type length: {l} (must be 4)"),
        }
    }
}

impl Error for ChunkTypeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
