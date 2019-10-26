use std::cmp::PartialOrd;

pub fn clamp<T:PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub fn extract_pixel_rgba(pixel: &image::Rgba<u8>) -> (u8, u8, u8, u8) {
    (pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3])
}

// https://stackoverflow.com/a/596241/3894179
pub fn get_brightness(r: u8, g: u8, b: u8) -> u8 {
    ((r as u16 + r as u16 + g as u16 + g as u16 + g as u16 + b as u16) / 6) as u8
}

pub struct ImageKernel {
    pub image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub run_type: i32,
    pub unblur_strength: u8,
    pub refine_strength: u8,
    pub color_data: Vec<u32>,
}

impl ImageKernel {
    pub fn from_image(image: image::DynamicImage) -> ImageKernel {
        ImageKernel {
            image: image.to_rgba(),
            run_type: 0,
            unblur_strength: 0,
            refine_strength: 0,
            color_data: Vec::new(),
        }
    }

    pub fn compute_luminance(&mut self) {
        for y in 0..self.image.height() {
            for x in 0..self.image.width() {
                let mut pixel = self.image.get_pixel_mut(x, y);
                let (r, g, b, _) = extract_pixel_rgba(&pixel);
                let brightness = get_brightness(r, g, b);
                let luminance_value = clamp(brightness, 0, 0xFF);

                pixel.0[0] = r;
                pixel.0[1] = r;
                pixel.0[2] = r;
                pixel.0[3] = luminance_value;
            }
        }
    }
}