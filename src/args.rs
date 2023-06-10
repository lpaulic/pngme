/*!
 * # Args crate
 *
 * Defines the command line arguments that are available for the user to invoke.
 *
 */
use clap::Parser;
use std::path::PathBuf;

/**
 *
 * Defines available subcommands.
 */
#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(bin_name = "pngme")]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

/**
*
* Encode operation writes a specified message to a PNG file under a specific chunk type.
*
* To invoke the encode functionality the user must provide the following:
* - a valid file path, absolute or relative, to the PNG file in which the message wants to be encoded
* - a valid string representation of the chunk type under which the message is going to be stored, that matches the requirements described in the [PNG specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
* - the string message that wants to be encoded in the specified PNG file
*
* The optional value that can be specified is:
* - a valid file path to the output file of the PNG file with the message will be stored in
*
* NOTE: if the output file path is not specified thant the modified PNG file will be stored in the source file provided as the first argument
*
*/
#[derive(Debug, clap::Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file_path: Option<PathBuf>,
}

/**
*
* Decode operation reads a message written to a PNG file under a specific chunk type.
*
* To invoke the decode functionality the user must provide the following:
* - a valid file path, absolute or relative, to the PNG file from which the message wants to be decoded
* - a valid string representation of the chunk type under which the message is stored, that matches the requirements described in the [PNG specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
*
*/
#[derive(Debug, clap::Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

/**
*
* Remove operation removes a chunk from the PNG file. Chunk type is used to reference the chunk that wants to be removed.
*
* To invoke the remove functionality the user must provide the following:
* - a valid file path, absolute or relative, to the PNG file from which the message wants to be decoded
* - a valid string representation of the chunk type under which the message is stored, that matches the requirements described in the [PNG specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
*
*/
#[derive(Debug, clap::Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

/**
*
* Print operation prints the information of the PNG in human readable format. The information that is printed is:
* - chunk type in string format
* - chunk length represented as an integer number
* - number of data bytes represented as an integer number
* - CRC of a chunk represented as an integer number
*
* The above info is printed for each chunk in the PNG file.
*
* To invoke the remove functionality the user must provide the following:
* - a valid file path, absolute or relative, to the PNG file from which the message wants to be decoded
* - a valid string representation of the chunk type under which the message is stored, that matches the requirements described in the [PNG specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
*
*/
#[derive(Debug, clap::Args)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
