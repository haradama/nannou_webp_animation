/// The module handling the animation playback, including rendering and frame management.
pub mod animation;

/// The module responsible for decoding WebP files and extracting frames for animation.
pub mod decoder;

/// The module defining the structure and properties of a single animation frame.
pub mod frame;

/// The module containing utility functions for image processing.
pub mod utils;

/// Re-exports the `WebpAnimation` struct for easy access.
///
/// `WebpAnimation` is the primary structure for managing and displaying WebP animations.
pub use crate::animation::WebpAnimation;
