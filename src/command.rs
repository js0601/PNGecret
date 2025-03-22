use std::fs::{read, write};
use std::str::FromStr;

use crate::Result;
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(args: EncodeArgs) -> Result<()> {
    // read file as bytes and turn it into PNG struct
    let img_bytes = read(&args.file)?;
    let mut png = Png::try_from(img_bytes.as_slice())?;

    // build new chunk
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.msg.as_bytes().to_vec());

    // append chunk to png
    png.append_chunk(chunk);

    // save modified png into file
    if let Some(f) = args.output {
        write(f, png.as_bytes())?;
    } else {
        write(args.file, png.as_bytes())?;
    }

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    // read file as bytes and turn it into PNG struct
    let img_bytes = read(&args.file)?;
    let png = Png::try_from(img_bytes.as_slice())?;

    // find chunk in png and print data
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("{}", chunk.data_as_string()?);
    } else {
        println!("No chunk of given type found!");
    }

    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    // read file as bytes and turn it into PNG struct
    let img_bytes = read(&args.file)?;
    let mut png = Png::try_from(img_bytes.as_slice())?;

    // find chunk in png and remove it
    let removed_chunk = png.remove_first_chunk(&args.chunk_type)?;
    println!("Removed chunk: {}", removed_chunk);

    // write changes
    write(args.file, png.as_bytes())?;

    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    // read file as bytes and turn it into PNG struct
    let img_bytes = read(&args.file)?;
    let png = Png::try_from(img_bytes.as_slice())?;

    // print chunks
    println!("{}", png);

    Ok(())
}
