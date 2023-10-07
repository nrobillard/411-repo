use array2::Array2;
use std::env;
use csc411_image::{Read, GrayImage};

fn has_duplicates(array: &Array2<u16>, start_row: usize, start_col: usize, size: usize) -> bool {
    let mut seen = vec![false; size + 1]; // Assuming Sudoku numbers are 1-9

    // Check duplicates in the row
    for col in start_col..start_col + size {
        let value = array.find(start_row, col);
        if value != 0 && seen[value as usize] {
            println!("Duplicate in row: {}", col);
            return true;
        }
        seen[value as usize] = true;
    }

    seen.iter_mut().for_each(|flag| *flag = false); // Reset seen flags

    // Check duplicates in the column
    for row in start_row..start_row + size {
        let value = array.find(row, start_col);
        if value != 0 && seen[value as usize] {
            println!("Duplicate in col: {}", row);
            return true;
        }
        seen[value as usize] = true;
    }

    seen.iter_mut().for_each(|flag| *flag = false); // Reset seen flags

    // Check duplicates in the box
    for row in start_row..start_row + size {
        for col in start_col..start_col + size {
            let value = array.find(row, col);
            if value != 0 && seen[value as usize] {
                println!("Duplicate in box: {}, {}", row,col);
                return true;
            }
            seen[value as usize] = true;
        }
    }

    false
}

// Function to check the entire Sudoku
fn sudoku_check(array: &Array2<u16>) -> bool {


    // Check rows and columns
    for i in 0..9 {
        if has_duplicates(array, i, 0, 9) || has_duplicates(array, 0, i, 9) {
            return false;
        }
    }

    // Check boxes
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            if has_duplicates(array, i, j, 3) {
                return false;
            }
        }
    }

    true
}

fn main() {
    assert!(env::args().len() <= 2, "Too many Arguments!");
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let pixels_row_major: Vec<u16> = img.pixels.iter().map(|pixel| pixel.value).collect();
    let array_row_major = Array2::from_row_major(pixels_row_major, 9, 9);

    //let result = sudoku_check(&array_row_major);
    //println!("Sudoku is valid: {}", result);
    //array_row_major.print_row_major();
}
