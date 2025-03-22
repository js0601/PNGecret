use std::fs::{read, write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::Result;
use crate::args::{DecodeArgs, EncodeArgs};
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

// this needs to:
// same as above
// instead of finding and outputting chunk, remove chunk from Png
fn remove(file: PathBuf, chunk_type: String) {
    todo!()
}

// this needs to:
// 1. handle input file (see above)
// 2. print every chunk in Png (make Display nicer for chunks)
fn print(file: PathBuf) {
    todo!()
}
