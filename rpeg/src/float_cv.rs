use array2::Array2;
use crate::rpeg_structs::{FloatRgb, CompVid};

pub fn frgb_to_cv(img: &Array2<FloatRgb>) -> Array2<CompVid> {
    
    // Make a new vector to hold all the component video data
    // Compute and map component video values for each pixel given the rgb values
    let cv_img: Vec<CompVid> = img.iter_row_major()
    .map(|(pixel, _, _)| {
        let y = 0.299 * pixel.red + 0.587 * pixel.green + 0.114 * pixel.blue;
        let pb = -0.168736 * pixel.red + -0.331264 * pixel.green + 0.5 * pixel.blue;
        let pr = 0.5 * pixel.red + -0.418688 * pixel.green + -0.081312 * pixel.blue;
        CompVid { y, pb, pr }
    })
    .collect();

    // Return the component video image
    Array2::from_row_major(cv_img, img.height(), img.width())

}

pub fn cv_to_frgb(img: &Array2<CompVid>) -> Array2<FloatRgb> {

    // Make a new vector to hold the rgb data
    // Compute and map the rgb values for each pixel given the y, pb, and pr values
    let rgb_img: Vec<FloatRgb> = img.iter_row_major()
    .map(|(pixel, _, _)| {
        let red = pixel.y + 1.402 * pixel.pr;
        let green = pixel.y + -0.344136 * pixel.pb + -0.714136 * pixel.pr;
        let blue = pixel.y + 1.772 * pixel.pb;
        FloatRgb { red, green, blue }
    })
    .collect();

    // Return the rgb float image
    Array2::from_row_major(rgb_img, img.height(), img.width())

}