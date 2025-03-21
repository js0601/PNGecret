use std::path::PathBuf;

// this needs to:
// 1. handle input file
//    - find file
//    - turn file into [u8]
//    - turn it into Png struct
// 2. build new chunk
//    - turn chunk_type into ChunkType
//    - turn msg into [u8]
//    - build Chunk from these two
// 3. add new chunk to Png
// 4. save Png with new chunk into file
//    - optionally save it into output file instead
fn encode(file: PathBuf, chunk_type: String, msg: String, output: Option<PathBuf>) {
    todo!()
}

// this needs to:
// 1. handle input file (see encode)
// 2. turn chunk_type into ChunkType
// 3. find chunk of type chunk_type in Png
// 4. output message inside of data field of chunk
fn decode(file: PathBuf, chunk_type: String) {
    todo!()
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
