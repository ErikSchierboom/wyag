use std::io::prelude::*;
use std::path::Path;

use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::io::prelude::*;

use flate2::read::ZlibDecoder;

pub(crate) fn execute(sha: String) {
    let x = format!(".git/objects/{}/{}", sha[0..2].to_string(), sha[2..].to_string());
    let path = Path::new(&x);
    let file = File::open(path).expect("ERROR: could not read find with SHA");
    let mut blob_object = String::new();
    let mut decoder = ZlibDecoder::new(file);
    decoder.read_to_string(&mut blob_object).unwrap();
}
