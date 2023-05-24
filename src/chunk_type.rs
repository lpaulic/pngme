use std::fmt;
use std::str;
use std::str::FromStr;

const BIT_OF_INTEREST: u8 = 0b0010_0000;
const BIT_SHIFT_NUM: u8 = 5;
const ANCILLARY_BYTE: usize = 0;
const PRIVATE_BYTE: usize = 1;
const RESERVED_BYTE: usize = 2;
const SAFTE_TO_COPY_BYTE: usize = 3;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    code: [u8; 4],
}

#[derive(Debug)]
pub enum IntoChunkTypeError {
    InvalidByteNumber,
    InvalidFormat,
}

impl fmt::Display for IntoChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntoChunkTypeError::InvalidByteNumber => {
                write!(f, "Input data doesn't have 4 bytes")
            }
            IntoChunkTypeError::InvalidFormat => {
                write!(f, "Input data is not in valid PNG chunk type format")
            }
        }
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.code
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        (self.code[ANCILLARY_BYTE] & BIT_OF_INTEREST) >> BIT_SHIFT_NUM == 0
    }

    pub fn is_public(&self) -> bool {
        (self.code[PRIVATE_BYTE] & BIT_OF_INTEREST) >> BIT_SHIFT_NUM == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.code[RESERVED_BYTE] & BIT_OF_INTEREST) >> BIT_SHIFT_NUM == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (self.code[SAFTE_TO_COPY_BYTE] & BIT_OF_INTEREST) >> BIT_SHIFT_NUM == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = IntoChunkTypeError;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType { code: value };
        let is_valid_chunk_type = chunk_type
            .code
            .iter()
            .filter(|x| (65..90).contains(*x) || (97..122).contains(*x))
            .count()
            == 4;
        match is_valid_chunk_type {
            true => Ok(chunk_type),
            false => Err(IntoChunkTypeError::InvalidFormat),
        }
    }
}

impl FromStr for ChunkType {
    type Err = IntoChunkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 {
            return Err(IntoChunkTypeError::InvalidByteNumber);
        }

        let four_bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        ChunkType::try_from(four_bytes)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.code).unwrap())
    }
}

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
