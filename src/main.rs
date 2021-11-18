use std::{fs::File, io::Write};

use fast_text_to_png::{render_text_to_png_data, Color};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha2::{Digest, Sha256};

const MAX_NUM: u32 = 1000000;
const MAX_COLOR: u32 = 255 * 255 * 255;
const FILE_NAME: &str = "image.png";

mod fast_text_to_png;

fn main() {
    let lowest_image = (0..MAX_NUM)
        .into_par_iter()
        .map(image)
        .min_by_key(|data| Sha256::digest(&data[..]))
        .unwrap();

    File::create(FILE_NAME)
        .unwrap()
        .write_all(&lowest_image)
        .unwrap();
    println!("hash: {}", hex::encode(Sha256::digest(&lowest_image[..])));
}

fn image(i: u32) -> Vec<u8> {
    let num = i / MAX_COLOR;
    let color = i % MAX_COLOR;
    render_text_to_png_data(num.to_string(), 20, Color::from(color))
}
