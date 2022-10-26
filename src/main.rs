#![allow(dead_code)]
#![allow(unused_variables)]
mod array2;
use csc411_image::{Read, RgbImage};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    file1: Option<String>,

    #[arg(required = true)]
    file2: Option<String>,
}
    
fn main() {
    // Everything else is extra credit but rotate90, rotate180
    let args = Args::parse();

    // // Parse file into a GrayImage variable
    let file1_slice: &str = &args.file1.unwrap()[..];
    let file2_slice: &str = &args.file2.unwrap()[..];  
    let img1 = RgbImage::read(Some(file1_slice).as_deref()).unwrap();
    let img2 = RgbImage::read(Some(file2_slice).as_deref()).unwrap();

    let mut red_sum1: f64;
    let mut green_sum1: f64;
    let mut blue_sum1: f64;

    for pixel in img1.pixels {
        red_sum1 += pixel.red as f64;
        green_sum1 += pixel.green as f64;
        blue_sum1 += pixel.blue as f64;
    }

    let mut red_sum2: f64;
    let mut green_sum2: f64;
    let mut blue_sum2: f64;

    for pixel in img2.pixels {
        red_sum2 += pixel.red as f64;
        green_sum2 += pixel.green as f64;
        blue_sum2 += pixel.blue as f64;
    }


}