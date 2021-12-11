use std::{fs::File, io::Write};

use fast_text_to_png::{get_num_to_black_pixmaps, Color};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha2::{Digest, Sha256};

const MAX_IMAGES: usize = 60_000_000;
const MAX_COLOR: usize = 255 * 255 * 255;
const FILE_NAME: &str = "image.png";

mod fast_text_to_png;

fn main() {
    let num_to_black_pixmaps = get_num_to_black_pixmaps(MAX_IMAGES / MAX_COLOR);

    let lowest_image = (0..MAX_IMAGES)
        .into_par_iter()
        .map(|i| {
            let num = i / MAX_COLOR;
            let color = Color::from((i % MAX_COLOR) as u32);
            num_to_black_pixmaps[&num].get_colored_png(color)
        })
        .min_by_key(|data| Sha256::digest(&data[..]))
        .unwrap();

    File::create(FILE_NAME)
        .unwrap()
        .write_all(&lowest_image)
        .unwrap();
    println!("hash: {}", hex::encode(Sha256::digest(&lowest_image[..])));
}
