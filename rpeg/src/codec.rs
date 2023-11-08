use csc411_image::{Read, Write, RgbImage, Rgb};
use csc411_rpegio;
use array2::Array2;
use crate::trim::trim_img;
use crate::rgb_float::{rgb_to_float, float_to_rgb};
use crate::float_cv::{frgb_to_cv, cv_to_frgb};
use crate::cv_dct::{cv_to_dct, dct_to_cv};

fn new_rgb(vec: Vec<Rgb>, width: u32, height: u32, denom: u16) -> RgbImage {
    RgbImage {
        pixels: vec,
        width,
        height,
        denominator: denom,
    }
}

fn extract_array2_to_vec(array2: &Array2<Rgb>) -> Vec<Rgb> {
    // Extract the pixels into a vector
    let pixels2: Vec<Rgb> = array2.iter_row_major().map(|(pixel, _, _)| pixel).collect();
    pixels2
}

// Rudimentary compression and decompression
pub fn compress(filename: Option<&str>) {
    
    // Open the image
    let img = RgbImage::read(filename.as_deref()).unwrap();
    let comp_step1 = Array2::from_row_major(img.pixels.clone(), img.height as usize, img.width as usize);

    // Trim the image, turn the u16 rgb to f32 rgb, turn f32 rgb to component video, turn cv to dct
    let comp_step2 = trim_img(&comp_step1);
    let comp_step3 = rgb_to_float(&comp_step2);
    let comp_step4 = frgb_to_cv(&comp_step3);
    let comp_step5 = cv_to_dct(&comp_step4);

    // Turn the dct to cv, component video to f32 rgb, turn f32 rgb to u16 rgb
    let decomp_step1 = dct_to_cv(&comp_step5);
    let decomp_step2 = cv_to_frgb(&decomp_step1);
    let decomp_step3 = float_to_rgb(&decomp_step2);

    // Turn the u16 rgb into a vec of rgb, make a new image from the vec, write the image
    let out1 = extract_array2_to_vec(&decomp_step3);
    let out2 = new_rgb(out1, comp_step2.width() as u32, comp_step2.height() as u32, img.denominator);
    out2.write(None).unwrap();
    
}

pub fn decompress(filename: Option<&str>) {
    let (_raw_bytes, _w, _h) = csc411_rpegio::input_rpeg_data(filename).unwrap();
}