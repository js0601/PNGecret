use args::Cli;
use clap::Parser;
use command::*;

mod args;
mod chunk;
mod chunk_type;
mod command;
mod crypt;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(c) = cli.command {
        match c {
            args::PngecretArgs::Encode(encode_args) => encode(encode_args)?,
            args::PngecretArgs::Decode(decode_args) => decode(decode_args)?,
            args::PngecretArgs::Remove(remove_args) => remove(remove_args)?,
            args::PngecretArgs::Print(print_args) => print(print_args)?,
        }
    }

    Ok(())
}
