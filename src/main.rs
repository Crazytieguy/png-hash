use std::{fs::File, io::Write};

use fast_text_to_png::{render_text_to_png_data, Color};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha2::{Digest, Sha256};

const IMAGES_PER_TASK: u32 = 1000;
const NUM_TASKS: u32 = 1000;
const MAX_COLOR: u32 = 255 * 255 * 255;
const FILE_NAME: &str = "image.png";

mod fast_text_to_png;

fn main() {
    let thread_results = (0..NUM_TASKS)
        .into_par_iter()
        .map(lowest_in_range)
        .collect::<Vec<_>>();

    let (lowest_hash, lowest_image) = thread_results.iter().min_by_key(|(hash, _)| hash).unwrap();

    File::create(FILE_NAME)
        .unwrap()
        .write_all(lowest_image)
        .unwrap();
    println!("hash: {}", hex::encode(lowest_hash));
}

fn image(i: u32) -> Vec<u8> {
    let num = i / MAX_COLOR;
    let color = i % MAX_COLOR;
    render_text_to_png_data(num.to_string(), 20, Color::from(color))
}

fn lowest_in_range(i: u32) -> (Vec<u8>, Vec<u8>) {
    let start = i * IMAGES_PER_TASK;
    let stop = start + IMAGES_PER_TASK;

    let (lowest_hash, lowest_image) = (start..stop)
        .into_iter()
        .map(|i| {
            let data = image(i);
            (Sha256::digest(&data), data)
        })
        .min_by_key(|&(hash, _)| hash)
        .unwrap();

    (lowest_hash.as_slice().to_owned(), lowest_image)
}
