mod patterns;
mod generator;

use std::{env::args, path::PathBuf};
use image::open;
use generator::generate_images;

fn main() {
    let filename: String = args().collect::<Vec<_>>()[1].clone();
    let source_img = open(&filename)
        .expect(format!("given file '{}' does not exist or cannot be read", &filename).as_str())
        .into_luma_alpha8();
    let output_images = generate_images(source_img);
    let name = PathBuf::from(filename).with_extension("");
    output_images.0.save(format!("./{}_1.png", name.to_str().unwrap())).expect("cannot write output to file");
    output_images.1.save(format!("./{}_2.png", name.to_str().unwrap())).expect("cannot write output to file");
}
