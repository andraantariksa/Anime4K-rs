pub fn clamp(val: i32, min: i32, max: i32) -> i32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub struct ImageKernel {
    pub image: image::DynamicImage,
    pub run_type: i32,
    pub unblur_strength: u8,
    pub refine_strength: u8,
}

impl ImageKernel {
    fn from_image(image: image::DynamicImage) -> ImageKernel {
        ImageKernel {
            image,
            run_type: 0,
            unblur_strength: 0,
            refine_strength: 0,
        }
    }
}