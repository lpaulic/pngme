use crate::chunk_type::{ChunkType, ChunkTypeError};
use crc::{Crc, CRC_32_ISO_HDLC};
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fmt;
use std::mem;
use std::str;

#[derive(Debug)]
pub enum ChunkError {
    InvalidConversion(str::Utf8Error),
    InvalidLength,
    InvalidCrc,
    MismatchCrc,
    InvalidChunkType(ChunkTypeError),
}

impl From<str::Utf8Error> for ChunkError {
    fn from(item: str::Utf8Error) -> ChunkError {
        ChunkError::InvalidConversion(item)
    }
}

impl From<ChunkTypeError> for ChunkError {
    fn from(item: ChunkTypeError) -> ChunkError {
        ChunkError::InvalidChunkType(item)
    }
}

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut type_and_data: Vec<u8> = chunk_type.bytes().to_vec();
        type_and_data.extend(&data);
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(type_and_data.as_slice());
        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        self.chunk_type.borrow()
    }

    pub fn data(&self) -> &[u8] {
        self.data.iter().as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, ChunkError> {
        str::from_utf8(&self.data)
            .map(|s| s.to_owned())
            .map_err(|e| ChunkError::InvalidConversion(e))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
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
    type Error = ChunkError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut value_iter = value.iter().copied();

        let length = u32::from_be_bytes(
            value_iter
                .borrow_mut()
                .take(mem::size_of::<u32>())
                .collect::<Vec<u8>>()
                .try_into()
                .map_err(|_| ChunkError::InvalidLength)?,
        );

        let chunk_type = ChunkType::try_from(
            std::convert::TryInto::<[u8; 4]>::try_into(
                value_iter
                    .borrow_mut()
                    .take(mem::size_of::<ChunkType>())
                    .collect::<Vec<u8>>(),
            )
            .map_err(|_| ChunkError::InvalidChunkType(ChunkTypeError::InvalidLen))?,
        )?;

        let data = value_iter
            .borrow_mut()
            .take(length as usize)
            .collect::<Vec<u8>>();

        let crc = u32::from_be_bytes(
            value_iter
                .borrow_mut()
                .take(mem::size_of::<u32>())
                .collect::<Vec<u8>>()
                .try_into()
                .map_err(|_| ChunkError::InvalidCrc)?,
        );

        let chunk = Self::new(chunk_type, data);
        if chunk.crc != crc {
            return Err(ChunkError::MismatchCrc);
        }

        Ok(chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[length: {}, type: {}, data: {}, crc: {}]",
            &self.length,
            &self.chunk_type,
            str::from_utf8(&self.data).unwrap(),
            &self.crc
        )
    }
}

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