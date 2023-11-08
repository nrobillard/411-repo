#[derive(Clone, Debug)]
pub struct FloatRgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Clone, Debug)]
pub struct CompVid {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

#[derive(Clone, Debug)]
pub struct DCTData {
    pub avgpb: f32,
    pub avgpr: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}