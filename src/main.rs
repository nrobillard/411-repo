use array2::Array2;
use std::env;
use csc411_image::{Read, GrayImage};

fn main() {
    assert!(env::args().len() <= 2, "Too many Arguments!");
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let pixels_row_major: Vec<u16> = img.pixels.iter().map(|pixel| pixel.value).collect();

    let array_row_major = Array2::from_row_major(pixels_row_major, 9, 9);
    println!("Row-Major Order:");
    array_row_major.print_col_major();
}