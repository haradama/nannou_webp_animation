use nannou::image::DynamicImage;
use std::time::Duration;

/// Represents a single frame in a WebP animation, containing the image data and its display duration.
#[derive(Clone)]
pub struct WebpFrame {
    /// The frame's image data as a `DynamicImage`.
    pub image: DynamicImage,
    /// The duration for which this frame should be displayed.
    pub duration: Duration,
}
