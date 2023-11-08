use array2::Array2;
use crate::rpeg_structs::{CompVid, DCTData};

pub fn cv_to_dct(img: &Array2<CompVid>) -> Array2<DCTData> {
    
    // Make a new vector to hold all the DCT data for each 2x2 chunk of pixels
    let dct_img: Vec<DCTData> = img.iter_row_major()
    .filter(|&(_, row, col)| row % 2 == 0 && col % 2 == 0)
    .map(|(pixel1, row, col)| {
        
        // Get the other 3 pixels in the 2x2
        let pixel2 = img.get(row, col + 1).unwrap();
        let pixel3 = img.get(row + 1, col).unwrap();
        let pixel4 = img.get(row + 1, col + 1).unwrap();
        
        // Compute the DCT data values
        let avgpb = (pixel1.pb + pixel2.pb + pixel3.pb + pixel4.pb) / 4.0;
        let avgpr = (pixel1.pr + pixel2.pr + pixel3.pr + pixel4.pr) / 4.0;
        let a = (pixel4.y + pixel3.y + pixel2.y + pixel1.y) / 4.0;
        let b = (pixel4.y + pixel3.y - pixel2.y - pixel1.y) / 4.0;
        let c = (pixel4.y - pixel3.y + pixel2.y - pixel1.y) / 4.0;
        let d = (pixel4.y - pixel3.y - pixel2.y + pixel1.y) / 4.0;

        DCTData { avgpb, avgpr, a, b, c, d }
    })
    .collect();

    // Return the DCT image
    Array2::from_row_major(dct_img, img.height() / 2, img.width() / 2)
    
}

pub fn dct_to_cv(img: &Array2<DCTData>) -> Array2<CompVid> {

    // Make a new vector to hold all the component video data
    let mut cv_img = Array2::set_array_zero(CompVid { y: 0.0, pb: 0.0, pr: 0.0 }, img.height() * 2, img.width() * 2);

    for (pixel, row, col) in img.iter_row_major() {

        // Compute the Y values for the 4 pixels
        let y1 = pixel.a - pixel.b - pixel.c + pixel.d;
        let y2 = pixel.a - pixel.b + pixel.c - pixel.d;
        let y3 = pixel.a + pixel.b - pixel.c - pixel.d;
        let y4 = pixel.a + pixel.b + pixel.c + pixel.d;

        // Create the pixels to place in the 2x2 image
        let pixel1 = CompVid { y: y1, pb: pixel.avgpb, pr: pixel.avgpr };
        let pixel2 = CompVid { y: y2, pb: pixel.avgpb, pr: pixel.avgpr };
        let pixel3 = CompVid { y: y3, pb: pixel.avgpb, pr: pixel.avgpr };
        let pixel4 = CompVid { y: y4, pb: pixel.avgpb, pr: pixel.avgpr };

        // Set the 4 pixels in the image
        cv_img.insert(row * 2, col * 2, pixel1).unwrap();
        cv_img.insert(row * 2, col * 2 + 1, pixel2).unwrap();
        cv_img.insert(row * 2 + 1, col * 2, pixel3).unwrap();
        cv_img.insert(row * 2 + 1, col * 2 + 1, pixel4).unwrap();

    }

    // Return the component video image
    cv_img
    
}