extern crate image;

use std::{env, process};

mod image_kernel;

fn main() {
    if env::args().len() < 2 {
        println!("Error: Please specify input and output png files.");
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let input_filename = &args[1];
    let output_filename = &args[2];

    let image = image::open(&input_filename).expect("Can't open image");

    let mut kernel_instance = image_kernel::ImageKernel::from_image(image);
    kernel_instance.compute_luminance();
    kernel_instance.image.save(output_filename).expect("Can't save image");

}
