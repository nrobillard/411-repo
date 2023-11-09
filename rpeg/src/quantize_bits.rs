use array2::Array2;
use crate::rpeg_structs::{QuantData};
use bitpack::bitpack::{fitss, fitsu, gets, getu, news, newu};

pub fn quantize_to_bits(img: Array2<QuantData>) -> Vec<u32> {
    todo!();
}

pub fn bits_to_quantize(img: Vec<u32>) -> Array2<QuantData> {
    todo!();
}