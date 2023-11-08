use array2::Array2;
use csc411_image::Rgb;
use crate::rpeg_structs::FloatRgb;

pub fn rgb_to_float(img: &Array2<Rgb>) -> Array2<FloatRgb> {
        
    // Make a new vector to hold the float rgb data
    // Map each rgb value to an f32 between 0 and 1 using the image denominator (hard coded as 255)
    let float_img: Vec<FloatRgb> = img.iter_row_major()
    .map(|(pixel, _, _)| FloatRgb {
        red: (pixel.red as f32) / 255.0,
        green: (pixel.green as f32) / 255.0,
        blue: (pixel.blue as f32) / 255.0,
    })
    .collect();

    // Return the float image
    Array2::from_row_major(float_img, img.height(), img.width())
}

pub fn float_to_rgb(img: &Array2<FloatRgb>) -> Array2<Rgb> {
            
    // Make a new vector to hold the u16 rgb data
    // Map each pixel to a u16 rgb using the float values and the image denominator (hard coded as 255)
    let rgb_img: Vec<Rgb> = img.iter_row_major()
    .map(|(pixel, _, _)| Rgb {
        red: (pixel.red * 255.0) as u16,
        green: (pixel.green * 255.0) as u16,
        blue: (pixel.blue * 255.0) as u16,
    })
    .collect();

    // Return the u16 rgb image
    Array2::from_row_major(rgb_img, img.height(), img.width())

}