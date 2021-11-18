use std::{fs::File, io::Write};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha2::{Digest, Sha256};
use text_to_png::TextRenderer;

const IMAGES_PER_THREAD: u32 = 1000000;

fn main() {
    let thread_results = (0..32)
        .into_par_iter()
        .map(lowest_in_range)
        .collect::<Vec<_>>();

    let (lowest, lowest_image) = thread_results
        .iter()
        .min_by_key(|(lowest, _)| lowest)
        .unwrap();

    let mut file = File::create("image.png").unwrap();
    file.write_all(lowest_image).unwrap();
    println!("hash: {}", hex::encode(&lowest));
}

fn image(i: u32) -> Vec<u8> {
    TextRenderer::default()
        .render_text_to_png_data(i.to_string(), 10, "#FF00FF")
        .unwrap()
        .data
}

fn hasher_for(data: &[u8]) -> Sha256 {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher
}

fn lowest_in_range(i: u32) -> (Vec<u8>, Vec<u8>) {
    let start = i * IMAGES_PER_THREAD;
    let stop = (i + 1) * IMAGES_PER_THREAD;
    let data_0 = image(start);
    let result = hasher_for(&data_0).finalize();
    let mut lowest_image = data_0;
    let mut lowest = result;

    for i in (start + 1)..stop {
        let data = image(i);
        let result = hasher_for(&data).finalize();
        if result < lowest {
            lowest = result;
            lowest_image = data;
        }
    }

    (lowest.as_slice().to_owned(), lowest_image)
}
