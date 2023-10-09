use array2::Array2;
use std::env;
use csc411_image::{Read, GrayImage};

/// Checks if a Sudoku grid is valid
// Function to check for duplicates in sets of 9
fn check_duplicates_sets_of_9(array: &Array2<u16>)->bool {
    for i in 0..array.height() {
        let mut seen_values = Vec::new();
        for j in 0..array.width() {
            let value = array.find(i, j);
            if value != 0 && seen_values.contains(&value) {
                return false;
            }
            seen_values.push(value);
        }
    }
    true
}

fn main() {
    assert!(env::args().len() <= 2, "Too many Arguments!");

    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();

    // Assuming the image contains a 9x9 Sudoku grid, convert it to a 2D array
    let pixels_row_major: Vec<u16> = img.pixels.iter().map(|pixel| pixel.value).collect();
    let array_row_major = Array2::from_row_major(pixels_row_major.clone(), 9, 9);

    let bool = check_duplicates_sets_of_9(&array_row_major);

    if bool {
        println!("Valid Sudoku grid");
    } else {
        println!("Invalid Sudoku grid");
    }



}
