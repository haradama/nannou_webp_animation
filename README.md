# nannou_webp_animation

A Rust library for decoding and rendering animated WebP images using the [nannou](https://nannou.cc/) creative coding framework.

## Overview

`nannou_webp_animation` allows you to load, decode, and display animated WebP images within your nannou applications. It handles frame decoding, animation playback, and integrates seamlessly with nannou's rendering capabilities.

## Features

- Decode animated WebP files and extract frames.
- Handle frame positioning, blending, and disposal methods for accurate rendering.
- Control animation playback (play, pause, loop).
- Easily integrate with nannou's `App` and `Draw` APIs.

## Installation

### Prerequisites

- **Rust** programming language.
- **libwebp** and **libwebpdemux** libraries installed on your system.
- **pkg-config** utility for discovering library paths and compilation flags.

### Installing Dependencies

#### macOS

Install `libwebp` and `pkg-config` using Homebrew:

```bash
brew install webp pkg-config
```

#### Ubuntu/Debian

Install the required packages:

```bash
sudo apt-get update
sudo apt-get install libwebp-dev pkg-config
```

#### Fedora

Install the dependencies:

```bash
sudo dnf install libwebp-devel pkgconf-pkg-config
```

#### Arch Linux

Install the necessary packages:

```bash
sudo pacman -S libwebp pkgconf
```

### Adding to Your Project

Add the following to your `Cargo.toml`:

```toml
[dependencies]
nannou = "0.19.0" # Or the latest version
libc = "0.2.162"
```

Clone this repository and include it as a local dependency if needed.

## Usage

### Loading and Displaying an Animated WebP

Here's a basic example of how to use `nannou_webp_animation` in your application:

```rust
use nannou::prelude::*;
use nannou_webp_animation::WebpAnimation;

struct Model {
    animation: WebpAnimation,
}

fn model(app: &App) -> Model {
    // Create a new window
    app.new_window().size(800, 600).build().unwrap();

    // Load the WEBP animation
    let assets = app.assets_path().expect("Failed to find assets directory");
    let webp_path = assets.join("animation.webp"); // Place 'animation.webp' in the 'assets' directory

    // Initialize the animation
    let animation = WebpAnimation::from_file(&webp_path, app)
        .expect("Failed to load WEBP animation");

    Model { animation }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Update the animation
    model.animation.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Clear the frame
    let draw = app.draw();
    draw.background().color(BLACK);

    // Draw the animation at the center
    model.animation.draw(&draw, pt2(0.0, 0.0), 1.0, 0.0);

    // Write to the frame
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
```

Place your animated WebP file named `animation.webp` inside an `assets` directory at the root of your project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
