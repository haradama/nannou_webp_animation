use nannou::image::{DynamicImage, ImageBuffer, Rgba};

/// Creates a `DynamicImage` from raw RGBA data.
///
/// This function constructs an image using the provided raw RGBA pixel data.
///
/// # Parameters
///
/// - `width`: The width of the image in pixels.
/// - `height`: The height of the image in pixels.
/// - `rgba_data`: A vector containing RGBA pixel data. The length should be `width * height * 4`.
///
/// # Returns
///
/// - `Some(DynamicImage)`: The created image if the data is valid.
/// - `None`: If the data length does not match `width * height * 4`, or if image creation fails.
///
/// # Examples
///
/// ```rust
/// # use nannou_webp_animation::utils::create_image_from_raw;
/// let width = 200;
/// let height = 200;
/// let rgba_data = vec![255; (width * height * 4) as usize];
/// let image = create_image_from_raw(width, height, rgba_data)
///     .expect("Image creation failed");
/// ```
pub fn create_image_from_raw(width: u32, height: u32, rgba_data: Vec<u8>) -> Option<DynamicImage> {
    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba_data)?;
    Some(DynamicImage::ImageRgba8(buffer))
}
