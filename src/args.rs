use clap::Parser;
use std::path::PathBuf;

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

#[derive(Debug, clap::Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file_path: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, clap::Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, clap::Args)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
