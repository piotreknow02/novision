use image::{ImageBuffer, Pixel, LumaA};
use crate::patterns::{Pattern, get_empty, get_full};

macro_rules! bool_to_pix {
    ($is:expr) => {
        if $is {0} else {255}
    }
}

macro_rules! set_pix {
    ($x:expr, $y:expr, $img:expr, $val:expr) => {
        if $x < $img.width() && $y < $img.height() {
            let pix = $img.get_pixel_mut($x, $y); {
                *pix = LumaA([0, $val])
            }
        } 
        // Just skip index out of bounds
    };
}

impl Pattern {
    pub fn to_pixels(&self) -> [[u8; 2]; 2] {
        [
            [bool_to_pix!(self.0.0), bool_to_pix!(self.0.1)],
            [bool_to_pix!(self.1.0), bool_to_pix!(self.1.1)],
        ]
    }
}

fn insert_pattern(target: &mut ImageBuffer<LumaA<u8>, Vec<u8>>, position: (u32, u32), p: &Pattern) {
    let pattern_values = p.to_pixels();

    
    set_pix!(position.0, position.1, target, pattern_values[0][0]);
    set_pix!(position.0, position.1 + 1, target, pattern_values[0][1]);
    set_pix!(position.0 + 1, position.1, target, pattern_values[1][0]);
    set_pix!(position.0 + 1, position.1 + 1, target, pattern_values[1][1]);

    // target[(position.0, position.1)] = Luma(pattern_values[0][0]);
    // target[(position.0, position.1 + 1)] = Luma(pattern_values[0][1]);
    // target[(position.0 + 1, position.1)] = Luma(pattern_values[1][0]);
    // target[(position.0 + 1, position.1 + 1)] = Luma(pattern_values[1][1]);
}

pub fn generate_images(source_image: ImageBuffer<LumaA<u8>, Vec<u8>>) -> (ImageBuffer<LumaA<u8>, Vec<u8>>, ImageBuffer<LumaA<u8>, Vec<u8>>) {
    // create 2 ciphered images
    let target_size = (source_image.width()*2, source_image.height()*2);

    let mut images = (
       ImageBuffer::new(target_size.0, target_size.1),
       ImageBuffer::new(target_size.0, target_size.1),
    );

    // Map patterns
    for (px, py, pixel) in source_image.enumerate_pixels() {
        let pattern_to_insert = match pixel.channels()[1] {
            0 => get_empty(),
            _ => get_full(),
        };
        insert_pattern(&mut images.0, (px * 2, py * 2), &pattern_to_insert.0);
        insert_pattern(&mut images.1, (px * 2, py * 2), &pattern_to_insert.1);
    }
    
    images
}
