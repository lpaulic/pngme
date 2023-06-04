use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::{Chunk, ChunkError};
use crate::chunk_type::{ChunkType, ChunkTypeError};
use crate::png::{Png, PngError};
use std::fs;
use std::io::Error;
use std::str::FromStr;

#[derive(Debug)]
pub enum CommandError {
    Filesystem(Error),
    Png(PngError),
    Chunk(ChunkError),
}

impl From<std::io::Error> for CommandError {
    fn from(item: std::io::Error) -> CommandError {
        CommandError::Filesystem(item)
    }
}

impl From<PngError> for CommandError {
    fn from(item: PngError) -> CommandError {
        CommandError::Png(item)
    }
}

impl From<ChunkError> for CommandError {
    fn from(item: ChunkError) -> CommandError {
        CommandError::Chunk(item)
    }
}

impl From<ChunkTypeError> for CommandError {
    fn from(item: ChunkTypeError) -> CommandError {
        CommandError::Chunk(ChunkError::InvalidChunkType(item))
    }
}

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<(), CommandError> {
    let mut png = Png::try_from(fs::read(&args.file_path)?.as_slice())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;

    if !chunk_type.is_valid() {
        return Err(CommandError::Chunk(ChunkError::InvalidChunkType(
            ChunkTypeError::InvalidFormat,
        )));
    }

    png.append_chunk(Chunk::new(chunk_type, args.message.as_bytes().to_vec()));

    match args.output_file_path {
        Some(p) => fs::write(p, png.as_bytes())?,
        None => fs::write(&args.file_path, png.as_bytes())?,
    };

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<(), CommandError> {
    let png = Png::try_from(fs::read(&args.file_path)?.as_slice())?;

    let chunk = png
        .chunk_by_type(&args.chunk_type)
        .ok_or(CommandError::Png(PngError::NotFoundChunk))?;

    println!(
        "{}",
        std::str::from_utf8(chunk.data()).unwrap_or("No encoded message.")
    );

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<(), CommandError> {
    let mut png = Png::try_from(fs::read(&args.file_path)?.as_slice())?;

    png.remove_chunk(&args.chunk_type)?;

    fs::write(&args.file_path, png.as_bytes())?;

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<(), CommandError> {
    let png = Png::try_from(fs::read(args.file_path)?.as_slice())?;

    png.chunks().iter().for_each(|c| println!("{}", c));

    Ok(())
}
