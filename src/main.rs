#![allow(dead_code)]
#![allow(unused_variables)]
use clap::Parser;
use csc411_image::{Read, RgbImage};
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // ./executable <File1> <File2>
    #[arg(required = true)]
    file1: Option<String>,

    #[arg(required = true)]
    file2: Option<String>,
}

fn main() {
    let args = Args::parse();

    // // Parse file into a GrayImage variable
    let file1_slice: &str = &args.file1.unwrap()[..];
    let file2_slice: &str = &args.file2.unwrap()[..];
    let img1 = RgbImage::read(Some(file1_slice).as_deref()).unwrap();
    let img2 = RgbImage::read(Some(file2_slice).as_deref()).unwrap();

    // Error Checking: Width and height of img1 and img2 can differ by at most 1
    if (img1.width as i32).abs_diff(img2.width as i32) > 1
        || (img1.height as i32).abs_diff(img2.height as i32) > 1
    {
        eprintln!("1.0");
        process::exit(0);
    }

    // Compute sum of:
    // (img1 red values + img2 red values)^2 + (img1 green values + img2 green values)^2 + (img1 blue values + img2 blue values)^2
    let numerator: f64 = img1
        .pixels
        .into_iter()
        .zip(img2.pixels.into_iter()) // Zips img1's and img2's pixels into an iterator of tuples
        .map(|(img1_pixel, img2_pixel)| { // Map through the iterator of tuples and add all color values together and square them
            (img1_pixel.red as f64 - img2_pixel.red as f64).powi(2) 
                + (img1_pixel.green as f64 - img2_pixel.green as f64).powi(2)
                + (img1_pixel.blue as f64 - img2_pixel.blue as f64).powi(2)
        })
        .sum(); // Put it all into one sum

    let smaller_img_width: u32;
    let smaller_img_height: u32;
    let smaller_img_denom: u16;

    if img1.width < img2.width || img1.height < img2.height {
        smaller_img_width = img1.width;
        smaller_img_height = img1.height;
        smaller_img_denom = img1.denominator;
    } else {
        smaller_img_width = img2.width;
        smaller_img_height = img2.height;
        smaller_img_denom = img1.denominator;
    }

    let root_mean_sq_diff =
        (numerator / (3 * smaller_img_width * smaller_img_height) as f64).sqrt();

    // Print root mean square difference to std output with 4 decimal places
    println!("{:.4}", root_mean_sq_diff / smaller_img_denom as f64);
}
