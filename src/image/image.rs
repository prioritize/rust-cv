use std::fs::File;
use std::io::{BufReader, Read};

pub struct Image {
    buffer: Vec<Pixel>,
}
pub struct Pixel {
    red: u64,
    green: u64,
    blue: u64,
    alpha: u64,

}

impl Pixel {
    pub fn clamp (&mut self) {

    }
}
pub enum Bits {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour
}

impl Image {
    pub fn new(fname: &str) {
        let mut file = File::open(fname);
        let buf =
        match file {
            Ok(file) => {
                let buf_reader = BufReader::new(file);
                // buf_reader.read_exact()
            }
            Err(e) => {
                println!("{:?}", e)
            }
        };
    }
    pub fn from_jpeg(fname: &str) {
       todo!()
    }
    pub fn from_png(fname: &str) {
        todo!()
    }
}
