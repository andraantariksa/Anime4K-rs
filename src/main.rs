extern crate clap;
extern crate image;
extern crate raster;

use clap::{App, Arg};

mod image_kernel;

fn main() {
    let matches = App::new("Anime4K-rs")
        .version("0.1")
        .author("Andra Antariksa <andra.antariksa@gmail.com>")
        .about("A High-Quality Real Time Upscaler for Anime Video")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file")
                .required(true),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .default_value("2")
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::with_name("iteration")
                .short("i")
                .long("iteration")
                .default_value("2")
                .help("Sets how many the iteration to do"),
        )
        .arg(
            Arg::with_name("push-color-strength")
                .long("pcs")
                .default_value("0")
                .help("Sets the push color strength"),
        )
        .arg(
            Arg::with_name("push-gradient-strength")
                .long("pgs")
                .default_value("1")
                .help("Sets push gradient strength"),
        )
        .get_matches();

    let input_filename = matches
        .value_of("INPUT")
        .expect("Error: Please specify input and output png files.");
    let output_filename = matches
        .value_of("OUTPUT")
        .expect("Error: Please specify input and output png files.");
    let scale = matches.value_of("scale").unwrap().parse::<f64>().expect("Error on parsing scale to f64");
    let iteration = matches.value_of("iteration").unwrap().parse::<u8>().expect("Error on parsing iteration to u8");
    let push_color_strength = matches.value_of("push-color-strength").unwrap().parse::<f64>().expect("Error on parsing push-color-strength to f64");
    let push_gradient_strength = matches.value_of("push-gradient-strength").unwrap().parse::<f64>().expect("Error on parsing push-gradient-strength to f64");

    let image = image::open(&input_filename).expect("Can't open image.");

    let mut kernel_instance = image_kernel::ImageKernel::from_image(image);
    kernel_instance.scale(
        kernel_instance.width() * scale as u32,
        kernel_instance.height() * scale as u32,
    );
    for _ in 0..iteration {
        kernel_instance.compute_luminance();
        kernel_instance.push_color(image_kernel::clamp((push_color_strength * 255.0) as u16, 0, 0xFFFF));
        kernel_instance.compute_gradient();
        kernel_instance.push_gradient(image_kernel::clamp((push_gradient_strength * 255.0) as u16, 0, 0xFFFF));
    }
    kernel_instance
        .save(output_filename)
        .expect("Can't save image.");
}
