use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "PNGecret", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<PngecretArgs>,
}

#[derive(Subcommand)]
pub enum PngecretArgs {
    /// Encode a secret message in a PNG file
    Encode(EncodeArgs),

    /// Decode a secret message from a PNG file
    Decode(DecodeArgs),

    /// Remove a secret message from a PNG file
    Remove(RemoveArgs),

    /// Print all chunks in a PNG file
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    /// PNG file to hide message in
    pub file: PathBuf,

    /// Chunk Type where message is hidden  
    /// (for more info, look at the PNG structure doc)
    pub chunk_type: String,

    /// Message to hide
    pub msg: String,

    /// Encrypt the message using a passphrase
    #[arg(short, long, value_name = "PASSPHRASE")]
    pub encrypt: Option<String>,

    /// Optional output file
    #[arg(short, long, value_name = "OUTPUT FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    /// PNG file to decode hidden message from
    pub file: PathBuf,

    /// Type of chunk the message is hidden in
    pub chunk_type: String,

    /// Decrypt the message using a passphrase
    #[arg(short, long, value_name = "PASSPHRASE")]
    pub decrypt: Option<String>,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    /// PNG file to remove message from
    pub file: PathBuf,

    /// Type of chunk the message is hidden in
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    /// PNG file to print
    pub file: PathBuf,
}
