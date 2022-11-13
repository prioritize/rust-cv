use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use jpeg_decoder as jpeg;
use itertools::enumerate;

pub struct Image {
    buffer: Vec<Pixel>,
    gray_buffer: Vec<u8>,
    width: u32,
    height: u32,
    max: u32
}
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,

}

impl Pixel {
    pub fn clamp (&mut self) {

    }
    pub fn new(in_slice: &[u8]) -> Self {
        Pixel {
            red: in_slice[0],
            green: in_slice[1],
            blue: in_slice[2],
            alpha: 0
        }
    }
}
impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}
pub enum Bits {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour
}

impl Image {
    pub fn new(buffer: Vec<Pixel>, width: u32, height: u32, max: u32) -> Self {
        let mut out = Image {
            buffer,
            width,
            height,
            max,
            gray_buffer: Vec::new()
        };
        out.grayscale();
        out
    }
    pub fn from_jpeg(file_name: &str) -> Image {
        let file = File::open(file_name).expect(&format!("Failed to open {}", &file_name));
        let mut decoder = jpeg::Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect(&format!("failed to decode: {}", file_name));
        assert_eq!(pixels.len() % 3, 0);
        let mut buffer = Vec::new();
        let mut idx = 0;
        while idx < pixels.len() {
            buffer.push(Pixel::new(&pixels[idx..idx+3]));
            idx = idx + 3;
        }
        let metadata = decoder.info().unwrap();
        println!("{:?}", metadata);
        Image::new(buffer, metadata.width as u32, metadata.width as u32, 255)
    }
    pub fn from_png(file_name: &str) {
        todo!()
    }
    fn print_prelude(width: u32, height: u32, max: u32) -> String {
        format!("P3\n\
        {} {}\n\
        {}\n", width, height, max)
    }
    pub fn to_ppm(&self, file_name: String) -> std::io::Result<()>{
        let mut handle = File::create(file_name).unwrap();
        let prelude = Image::print_prelude(self.width, self.height, self.max);
        handle.write(prelude.to_string().as_ref())?;
        let mut idx = 1;
        let mut line = String::new();
        for pixel in &self.buffer {
            line = line + &pixel.to_string() + " ";
            if idx == self.width {
                line = line + "\n";
                handle.write(line.as_ref())?;
                line.clear();
                idx = 1;
            }
            idx = idx + 1;
        }
        Ok(())
    }
    pub fn to_ppm_gray(&self, file_name: String) -> std::io::Result<()> {
        let mut handle = File::create(file_name).unwrap();
        let prelude = format!("P2\n{} {}\n{}\n", self.width, self.height, self.max);
        handle.write(prelude.to_string().as_ref())?;
        let mut idx = 1;
        let mut line = String::new();
        for pixel in &self.gray_buffer{
            line = line + &pixel.to_string() + " ";
            if idx == self.width {
                line = line + "\n";
                handle.write(line.as_ref())?;
                line.clear();
                idx = 1;
            }
            idx = idx + 1;
        }
        Ok(())
    }
    pub fn sobel(&self) {
        let horizontal_kern = [[1, 0, -1], [2, 0, -2], [1, 0, 1]];
        let vert_kern = [[1, 2, 1], [0, 0, 0], [-1, -2, -1]];

    }
    pub fn grayscale(&mut self) {
        self.gray_buffer = self.buffer.iter().map(|x| {
            (x.red as f64 * 0.299 + x.green as f64 * 0.587 + x.blue as f64 * 0.144) as u8
        }).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_open_jpeg() {
       Image::from_jpeg("algorithm-expert.jpeg");
    }
    #[test]
    fn test_print_prelude() {
        let prelude = "P3\n\
        1024 1024\n\
        255\n".to_string();
        assert_eq!(Image::print_prelude(1024, 1024, 255), prelude);
    }
    #[test]
    fn test_write_ppm() {
        let image = Image::from_jpeg("algorithm-expert.jpeg");
        let f_name = "test.ppm".to_string();
        image.to_ppm(f_name).unwrap();
    }
    #[test]
    fn test_write_ppm_gray() {
        let image = Image::from_jpeg("algorithm-expert.jpeg");
        let f_name = "gray_test.ppm".to_string();
        image.to_ppm_gray(f_name).unwrap();
    }
}
