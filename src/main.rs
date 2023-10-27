use clap::Parser;
use array2::Array2;
use csc411_image::{Read, Rgb, RgbImage, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long = "row-major")]
    row_map: bool,
    #[clap(long = "col-major")]
    col_map: bool,
    /// Flip
    #[clap(long= "flip", required = false)]
    flip: Option<String>,
    /// Rotation
    #[clap(short= 'r', long = "rotate")]
    rotate: Option<u32>,
    /// Transposition
    #[clap(long = "transpose")]
    transpose: bool,
    /// Positional argument for the input file
    input_fname: String,
    /// Positional argument for the output file
    output_fname: String,
}

fn main() {
    // parse the arguments
    let args = Args::parse();
    // read the image using grayimage crate
    let img = RgbImage::read(Some(&args.input_fname)).unwrap();

    // assign the arguments to variables
    let mapping_type = if args.row_map { "row-major" } else if args.col_map { "col-major" } else { "row-major" };
    let rotate = args.rotate.unwrap_or_else(|| 0);
    let flip = args.flip.unwrap_or_else(|| "empty".to_string());
    let transpose = args.transpose;

    // create a 2D array from the vector
    let array2 = Array2::from_row_major(img.pixels.clone(), img.height as usize, img.width as usize);

    // decide what operations to perform on the image based on user input
    let new_array2 = decider(array2, mapping_type.to_string(), rotate as usize, flip, transpose);

    // Extract the pixels into a vector
    let pixels2: Vec<Rgb> = match mapping_type {
        "row-major" => new_array2.iter_row_major().map(|(pixel, _, _)| pixel).collect(),
        "column-major" => new_array2.iter_col_major().map(|(pixel, _, _)| pixel).collect(),
        _ => Vec::new(),
    };

    // Create a new RgbImage
    let new_image = new_RGB(pixels2, new_array2.width() as u32, new_array2.height() as u32, img.denominator as u16);

    // Write the new image to a file
    new_image.write(Some(&args.output_fname)).unwrap();
}

fn new_RGB(vec: Vec<Rgb>, width: u32, height: u32, denom: u16) -> RgbImage {
    RgbImage {
        pixels: vec,
        width,
        height,
        denominator: denom,
    }
}

fn decider(array2: Array2<Rgb>, map_type: String, rotate: usize, flip: String, transpose: bool) -> Array2<Rgb> {
    // decides what operations to do on the image based on user input

    let mut new_array2 = Array2::set_array_zero(array2.width(),array2.height(), Rgb { red: 0, green: 0, blue: 0 });

    if map_type == "row-major" {
        // do row-major mapping
        if rotate == 90 {
            // do rotation
            for (pixel, row, col) in array2.iter_row_major() {
                let h = array2.height();
                let w = array2.width();

                // Calculate rotated coordinates
                let new_row = col;
                let new_col = h - row - 1;

                // Insert the pixel into the new array
                new_array2.insert(new_row, new_col, pixel);
            }
        }
    }

    new_array2
}


