use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

use crate::ObjectType;

// Create type for Object (including object_type and hash() method)
pub(crate) fn execute(write: bool, file: PathBuf, object_type: ObjectType) {
    let contents = fs::read_to_string(&file).expect("ERROR: could not find file");

    let entry = format!("{} {}\0{}", object_type, contents.len(), contents);

    let mut hasher = Sha1::new();
    hasher.update(entry.as_bytes());
    let hash_bytes = hasher.finalize();
    let hash = format!("{:x}", hash_bytes);

    let dir_path = format!(".git/objects/{}", &hash[0..2]);
    let dir = Path::new(&dir_path[0..]);
    let output_file = dir.join(hash[2..].to_string());

    if write {
        if !dir.exists() {
            fs::create_dir(dir).expect("could not create dir")
        }

        let f = File::create(output_file).expect("ERROR: could not write hash to file");
        let mut encoder = ZlibEncoder::new(f, Compression::none());
        encoder.write_all(&hash_bytes).expect("ERROR: could not write hash to file");
    }

    println!("{}", hash);
}
