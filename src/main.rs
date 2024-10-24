use std::{
    fs,
    io::{BufReader, Read, Write},
};
use xxhash_rust::xxh3::xxh3_128;

fn main() {
    let filename = "/home/askiiart/whyy/testing/stuff-100M";
    let mut stuff = fs::File::open(filename).unwrap();
    let file_size: u64 = stuff.metadata().unwrap().len();

    let mut buf = BufReader::new(stuff);
    let mut out = fs::File::create("output").unwrap();

    let mut data = [0; 16];
    buf.read_exact(&mut data);
    out.write(&xxh3_128(&data).to_be_bytes());
    let mut location: u64 = 0;
    while location <= file_size / 16 {
        buf.read_exact(&mut data);
        location += 1;
        out.write(&xxh3_128(&data).to_be_bytes());
    }
}
