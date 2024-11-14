use nannou::image::{DynamicImage, ImageBuffer, Rgba};

pub fn create_image_from_raw(width: u32, height: u32, rgba_data: Vec<u8>) -> Option<DynamicImage> {
    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba_data)?;
    Some(DynamicImage::ImageRgba8(buffer))
}
