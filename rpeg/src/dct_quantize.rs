use csc411_arith;
use array2::Array2;
use crate::rpeg_structs::{DCTData, QuantData};

pub fn dct_to_quantize(img: Array2<DCTData>) -> Array2<QuantData> {

    // Make a new vector to hold the quantized data
    let quant_img = img.iter_row_major()
    .map(|(pixel, _, _)| {

        // Get index pb/pr from the 411 crate
        let ind_pb = csc411_arith::index_of_chroma(pixel.avgpb) as u64;
        let ind_pr = csc411_arith::index_of_chroma(pixel.avgpr) as u64;

        // Quantize a to be 9 unsigned bits
        let a = (pixel.a * 511.0).round() as u64;

        // Prepare to quantize b, c, and d to be 5 signed bits
        let mut bcd_dct = Vec::new();
        let mut bcd_quant = Vec::new();
        bcd_dct.push(pixel.b);
        bcd_dct.push(pixel.c);
        bcd_dct.push(pixel.d);

        // Fix b c and d to be between -0.3 and 0.3
        for i in 0..bcd_dct.len() {
            if bcd_dct[i] > 0.3 {
                bcd_dct[i] = 0.3;
            } else if bcd_dct[i] < -0.3 {
                bcd_dct[i] = -0.3;
            }
            // Quantize b c and d to be 5 signed bits
            bcd_quant.push((bcd_dct[i] * 50.0).round() as i64);
        }

        QuantData { ind_pb, ind_pr, a, b: bcd_quant[0], c: bcd_quant[1], d: bcd_quant[2] }
    })
    .collect();

    // Return the quantized image
    Array2::from_row_major(quant_img, img.height(), img.width())

}

pub fn quantize_to_dct(img: Array2<QuantData>) -> Array2<DCTData> {

    // Make a new vector to hold all the DCT data for each 2x2 chunk of pixels
    let dct_img = img.iter_row_major()
    .map(|(pixel, _, _)| {

        // Get the average pb and pr values from the 411 crate
        let avgpb = csc411_arith::chroma_of_index(pixel.ind_pb as usize);
        let avgpr = csc411_arith::chroma_of_index(pixel.ind_pr as usize);

        // Turn a, b, c, and d back into f32's
        let a = pixel.a as f32 / 511.0;
        let b = pixel.b as f32 / 50.0;
        let c = pixel.c as f32 / 50.0;
        let d = pixel.d as f32 / 50.0;

        DCTData { avgpb, avgpr, a, b, c, d }
    })
    .collect();

    // Return the DCT image
    Array2::from_row_major(dct_img, img.height(), img.width())

}