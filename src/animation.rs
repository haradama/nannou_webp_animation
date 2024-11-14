use crate::decoder::WebpDecoder;
use crate::frame::WebpFrame;
use nannou::prelude::*;
use std::path::Path;
use std::time::Instant;

pub struct WebpAnimation {
    frames: Vec<WebpFrame>,
    current_frame_index: usize,
    last_frame_time: Instant,
    is_looping: bool,
    textures: Vec<wgpu::Texture>,
}

impl WebpAnimation {
    pub fn from_file<P: AsRef<Path>>(path: P, app: &App) -> Result<Self, String> {
        let frames = WebpDecoder::decode(path)?;

        if frames.is_empty() {
            return Err("No frames found in the animation".to_string());
        }

        // Create textures from images
        let textures: Vec<wgpu::Texture> = frames
            .iter()
            .map(|frame| wgpu::Texture::from_image(app, &frame.image))
            .collect();

        Ok(Self {
            frames,
            current_frame_index: 0,
            last_frame_time: Instant::now(),
            is_looping: true,
            textures,
        })
    }

    pub fn update(&mut self) {
        let current_frame = &self.frames[self.current_frame_index];
        let duration = current_frame.duration;

        if self.last_frame_time.elapsed() >= duration {
            self.current_frame_index += 1;
            if self.current_frame_index >= self.frames.len() {
                if self.is_looping {
                    self.current_frame_index = 0;
                } else {
                    self.current_frame_index = self.frames.len() - 1;
                }
            }
            self.last_frame_time = Instant::now();
        }
    }

    pub fn draw(&self, draw: &Draw, position: Point2, scale: f32, rotation: f32) {
        if let Some(texture) = self.textures.get(self.current_frame_index) {
            draw.texture(texture)
                .x_y(position.x, position.y)
                .w_h(
                    texture.size()[0] as f32 * scale,
                    texture.size()[1] as f32 * scale,
                )
                .rotate(rotation);
        }
    }

    // Playback control methods
    pub fn set_looping(&mut self, looping: bool) {
        self.is_looping = looping;
    }
}
