use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt").expect("could not create file");
    file.write_all(b"hello world")
        .expect("can't write to the file");
}
