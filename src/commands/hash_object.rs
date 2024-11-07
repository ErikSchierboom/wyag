use std::fs;
use std::path::PathBuf;

use sha1::{Digest, Sha1};

use crate::ObjectType;

pub(crate) fn execute(write: bool, file: PathBuf, object_type: ObjectType) {
    let contents = fs::read_to_string(file).expect("ERROR: could not find file");

    let entry = format!("{} {}\0{}", object_type, contents.len(), contents);
    println!("entry: {}", entry);

    let mut hasher = Sha1::new();
    hasher.update(entry.as_bytes());
    let hash = hasher.finalize();

    println!("hash: {:x}", hash);

    // let mut encoder = ZlibEncoder::new(entry, Compression::none());
    // encoder.
}
