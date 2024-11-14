use crate::decoder::WebpDecoder;
use crate::frame::WebpFrame;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use std::path::Path;
use std::time::Instant;

/// Represents a WebP animation, handling frame data, playback control, and rendering.
pub struct WebpAnimation {
    /// Collection of frames in the animation.
    frames: Vec<WebpFrame>,
    /// Index of the current frame in the animation sequence.
    current_frame_index: usize,
    /// Time when the last frame was rendered, used for timing control.
    last_frame_time: Instant,
    /// Indicates whether the animation should loop when it reaches the end.
    is_looping: bool,
    /// Textures for each frame, generated from the images in the animation.
    textures: Vec<Texture>,
}

impl WebpAnimation {
    /// Creates a new `WebpAnimation` instance by loading frames from a WebP file.
    ///
    /// This function decodes the WebP file at the given path and prepares the animation for playback.
    ///
    /// # Parameters
    ///
    /// - `path`: Path to the WebP file containing the animation.
    /// - `app`: Reference to the Nannou `App` instance, used for creating textures.
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    /// - `Ok(WebpAnimation)`: The animation instance if successful.
    /// - `Err(String)`: An error message if loading fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be decoded or contains no frames.
    pub fn from_file<P: AsRef<Path>>(path: P, app: &App) -> Result<Self, String> {
        let frames = WebpDecoder::decode(path)?;

        if frames.is_empty() {
            return Err("No frames found in the animation".to_string());
        }

        // Create textures from images using Nannou's Texture
        let textures: Vec<Texture> = frames
            .iter()
            .map(|frame| Texture::from_image(app, &frame.image))
            .collect();

        Ok(Self {
            frames,
            current_frame_index: 0,
            last_frame_time: Instant::now(),
            is_looping: true,
            textures,
        })
    }

    /// Updates the animation's current frame based on elapsed time.
    ///
    /// This function should be called in each frame of the main loop to keep the animation
    /// in sync with its intended frame durations.
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

    /// Returns a reference to the current texture.
    ///
    /// # Returns
    ///
    /// A reference to the `Texture` of the current frame.
    pub fn texture(&self) -> &Texture {
        &self.textures[self.current_frame_index]
    }

    /// Returns the width of the current frame.
    ///
    /// # Returns
    ///
    /// The width (in pixels) of the current frame's texture.
    pub fn width(&self) -> u32 {
        self.texture().size()[0]
    }

    /// Returns the height of the current frame.
    ///
    /// # Returns
    ///
    /// The height (in pixels) of the current frame's texture.
    pub fn height(&self) -> u32 {
        self.texture().size()[1]
    }

    /// Sets whether the animation should loop after reaching the final frame.
    ///
    /// # Parameters
    ///
    /// - `looping`: If `true`, the animation will loop indefinitely. If `false`, it will stop at the last frame.
    pub fn set_looping(&mut self, looping: bool) {
        self.is_looping = looping;
    }
}
