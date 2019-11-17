use super::*;

fn anime4k(
    input_filename: &str,
    output_filename: &str,
    scale: f64,
    iteration: u32,
    push_color_strength: f64,
    push_gradient_strength: f64,
) {
    let image = image::open(input_filename).expect("Can't open image.");

    let mut kernel_instance = image_kernel::ImageKernel::from_image(image);
    kernel_instance.scale(
        (kernel_instance.width() as f64 * scale) as u32,
        (kernel_instance.height() as f64 * scale) as u32,
    );
    for _ in 0..iteration {
        kernel_instance.compute_luminance();
        kernel_instance.push_color(image_kernel::clamp(
            (push_color_strength * 255.0) as u16,
            0,
            0xFFFF,
        ));
        kernel_instance.compute_gradient();
        kernel_instance.push_gradient(image_kernel::clamp(
            (push_gradient_strength * 255.0) as u16,
            0,
            0xFFFF,
        ));
    }
    kernel_instance
        .save(output_filename)
        .expect("Can't save image.");
}

#[test]
fn test_small_eye_image_default() {
    anime4k("assets/eye-in.png", "assets/eye-out.png", 2.0, 1, 0.0, 1.0);
}

#[test]
fn test_small_eye_image_more_color_strength() {
    anime4k("assets/eye-in.png", "assets/eye-more-color-strength-out.png", 2.0, 1, 1.0, 1.0);
}

#[test]
fn test_small_eye_image_more_iteration() {
    anime4k("assets/eye-in.png", "assets/eye-more-iteration-out.png", 2.0, 2, 0.0, 1.0);
}

#[test]
fn test_small_eye_image_more_scale() {
    anime4k("assets/eye-in.png", "assets/eye-more-scale-out.png", 4.0, 1, 0.0, 1.0);
}

#[test]
fn test_people_default() {
    anime4k("assets/people-in.png", "assets/people-out.png", 2.0, 1, 0.0, 1.0);
}

#[test]
fn test_scenery_image_default() {
    anime4k("assets/scenery-in.png", "assets/scenery-out.png", 2.0, 1, 0.0, 1.0);
}
