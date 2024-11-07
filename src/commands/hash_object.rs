use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

use crate::ObjectType;

pub(crate) fn execute(write: bool, file: PathBuf, object_type: ObjectType) {
    let contents = fs::read_to_string(&file).expect("ERROR: could not find file");

    let entry = format!("{} {}\0{}", object_type, contents.len(), contents);

    let mut hasher = Sha1::new();
    hasher.update(entry.as_bytes());
    let hash_bytes = hasher.finalize();
    let hash = format!("{:x}", hash_bytes);

    let x = format!(".git/objects/{}/{}", hash[0..2].to_string(), hash[2..].to_string());
    let output_file = Path::new(&x);

    if write {
        let f = File::open(output_file).expect("ERROR: could not write hash to file");
        let mut encoder = ZlibEncoder::new(f, Compression::none());
        encoder.write_all(&hash_bytes).expect("ERROR: could not write hash to file");
    }

    println!("{}", hash);
}
