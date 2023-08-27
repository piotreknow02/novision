use image::GenericImage;
use crate::patterns::{Pattern, get_empty, get_full};

macro_rules! bool_to_pix {
    ($is:expr) => {
        if $is {0} else {255}
    }
}

impl Pattern {
    pub fn to_pixels(&self) -> [[u8; 2]; 2] {
        [
            [bool_to_pix!(self.0.0), bool_to_pix!(self.0.1)],
            [bool_to_pix!(self.1.0), bool_to_pix!(self.1.1)],
        ]
    }
}

pub fn generate_images(source_image: GenericImage) -> (GenericImage, GenericImage) {
    unimplemented!();
}