use crc::Crc;
use std::{
    error::Error,
    fmt::Display,
    io::{BufReader, Read},
};

use crate::chunk_type::ChunkType;

const CRC_PNG: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

pub struct Chunk {
    // length of ONLY the data field
    length: u32,
    chunk_type: ChunkType,
    // can be of zero length
    data: Vec<u8>,
    // CRC including type and data but NOT length
    crc: u32,
}

#[allow(dead_code)] // NOTE: needed?
impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        // NOTE: if the message is longer than 4.2 million characters this will panic
        // but unless you are going to encode War and Peace ~1.4 times this shouldn't happen
        let length = data
            .len()
            .try_into()
            .expect("Your message is too long, what the hell are you trying to hide");
        let mut msg = chunk_type.bytes().to_vec();
        msg.extend_from_slice(&data);
        let crc = CRC_PNG.checksum(&msg);

        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn data_as_string(&self) -> crate::Result<String> {
        Ok(<String>::from_utf8(self.data().to_vec())?)
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(bytes);

        // first 4 bytes should be length
        let mut length_buf = [0; 4];
        reader.read_exact(&mut length_buf)?;
        let length = <u32>::from_be_bytes(length_buf);

        // next 4 bytes should be the chunk type
        let mut ctype_buf = [0; 4];
        reader.read_exact(&mut ctype_buf)?;
        let chunk_type = ChunkType::try_from(ctype_buf)?;

        // read rest, then take last 4 bytes as crc and rest as data
        let mut rest_buf = vec![];
        reader.read_to_end(&mut rest_buf)?;
        let data = rest_buf[..rest_buf.len() - 4].to_vec();
        let crc = <u32>::from_be_bytes(rest_buf[rest_buf.len() - 4..].try_into()?);

        // create new chunk to see if length and CRC checksum are correct
        // the chunk type is already valid here, since the try_from succeeded
        let chunk = Chunk::new(chunk_type, data);
        if length != chunk.length() {
            return Err(Box::new(ChunkError::BadLength(length)));
        }
        if crc != chunk.crc() {
            return Err(Box::new(ChunkError::BadChecksum(crc)));
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}",
            self.chunk_type,
            self.data_as_string().unwrap_or("[data]".to_string())
        )
    }
}

#[derive(Debug)]
enum ChunkError {
    BadLength(u32),
    BadChecksum(u32),
}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::BadLength(l) => write!(f, "Chunk has an incorrect length: {l}"),
            ChunkError::BadChecksum(s) => write!(f, "Chunk has an incorrect checksum: {s}"),
        }
    }
}

impl Error for ChunkError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
