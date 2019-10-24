extern crate image;

use std::{env, process, fs};
// use image::ImageBuffer;

mod image_kernel;

fn main() {
    if env::args().len() < 2 {
        println!("Error: Please specify input and output png files.");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let input_filename = &args[1]; // Debug
    let output_filename = &args[2];

    // let input_file_content = fs::read_to_string(input_filename).expect("Can't read the file");

    // let image_input_buffer = ImageBuffer::from_raw(100, 100, );

    let image = image::open(&input_filename).expect("Can't open image");

}
