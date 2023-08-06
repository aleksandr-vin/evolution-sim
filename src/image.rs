use image::ImageResult;

// Imaging options
pub const IMG_SCALE_X: usize = 2;
pub const IMG_SCALE_Y: usize = 2;

pub trait CreateImage {
    fn save_image(&self, file_name: &str) -> ImageResult<()>;
}
