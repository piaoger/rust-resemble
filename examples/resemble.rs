use std::path::Path;

use image::ImageFormat;
use rust_resemble::{compare_images, ComparisonOptions};

fn main() {
    let img1 =
        image::open(&Path::new("./examples/people1.jpg")).expect("unable to load people1.jpg");
    let img2 =
        image::open(&Path::new("./examples/people2.jpg")).expect("unable to load people2.jpg");
    let opts = ComparisonOptions::new().ignore_less();

    let result = compare_images(&img1, &img2, &opts);
    println!("diff by {}%", result.mismatch_percent);

    result
        .image
        .save_with_format("./examples/diff.jpg", ImageFormat::Jpeg)
        .unwrap();
}
