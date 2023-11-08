use array2::Array2;
use csc411_image::Rgb;

pub fn trim_img(img: &Array2<Rgb>) -> Array2<Rgb> {
    
    // Get the trimmed width/height (if need be)
    let new_height = if img.height() % 2 != 0 { img.height() - 1 } else { img.height() };
    let new_width = if img.width() % 2 != 0 { img.width() - 1 } else { img.width() };

    // Make a new vector, copy all the old image data into it (with exception of the trimmed dimensions)
    let trimmed_img: Vec<Rgb> = img.iter_row_major()
    .filter(|&(_, row, col)| row < new_height && col < new_width)
    .map(|(pixel, _, _)| pixel)
    .collect();

    // Return the trimmed image
    Array2::from_row_major(trimmed_img, new_height, new_width)

}