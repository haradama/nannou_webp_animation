use nannou::image::DynamicImage;
use std::time::Duration;

#[derive(Clone)]
pub struct WebpFrame {
    pub image: DynamicImage,
    pub duration: Duration,
}
