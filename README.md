# nannou_webp_animation

[<img alt="github" src="https://img.shields.io/badge/github-haradama/nannou__webp__animation-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/haradama/nannou_webp_animation)
[<img alt="crates.io" src="https://img.shields.io/crates/v/nannou_webp_animation.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/nannou_webp_animation)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-nannou__webp__animation-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/nannou_webp_animation)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/haradama/nannou_webp_animation/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/haradama/nannou_webp_animation/actions)

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

Install `libwebp` and `pkg-config` using Homebrew.

```bash
brew install webp pkg-config
```

#### Ubuntu/Debian

Install the required packages.

```bash
sudo apt-get update
sudo apt-get install libwebp-dev pkg-config
```

#### Fedora

Install the dependencies.

```bash
sudo dnf install libwebp-devel pkgconf-pkg-config
```

#### Arch Linux

Install the necessary packages.

```bash
sudo pacman -S libwebp pkgconf
```

#### Windows

Install the required packages using MSYS2.

1. **Download and Install MSYS2**

   Visit the [MSYS2 website](https://www.msys2.org/) and follow the installation instructions.

2. **Update Package Database and Core Packages**

   Open the **MSYS2 MinGW 64-bit** terminal and run.

   ```bash
   pacman -Syu
   ```

   If prompted, close and reopen the terminal, then run the command again.

3. **Install Dependencies**

   ```bash
   pacman -S mingw-w64-x86_64-libwebp mingw-w64-x86_64-pkg-config
   ```

### Adding to Your Project

Add the following to your `Cargo.toml`.

```toml
[dependencies]
nannou = "0.19.0" # Or the latest version
nannou_webp_animation = "0.2.0"
```

## Usage

### Loading and Displaying an Animated WebP

Here's a basic example of how to use `nannou_webp_animation` in your application.

```rust
use nannou::prelude::*;
use nannou_webp_animation::WebpAnimation;

struct Model {
    animation: WebpAnimation,
}

fn model(app: &App) -> Model {
    // Create a new window
    app.new_window().view(view).build().unwrap();

    // Load the WEBP animation
    let assets = app.assets_path().expect("Failed to find assets directory");
    // Place 'animation.webp' in the 'assets' directory
    let webp_path = assets.join("animation.webp");

    // Initialize the animation
    let animation =
        WebpAnimation::from_file(&webp_path, app).expect("Failed to load WEBP animation");

    Model { animation }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Update the animation
    model.animation.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Clear the frame
    frame.clear(BLACK);

    let win = app.window_rect();

    // Define the rectangle where the animation will be drawn
    let r = Rect::from_w_h(
        model.animation.width() as f32,
        model.animation.height() as f32,
    )
    .top_left_of(win);

    let draw = app.draw();
    draw.texture(model.animation.texture())
        .xy(r.xy())
        .wh(r.wh());

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
```

Place your animated WebP file named `animation.webp` inside an `assets` directory at the root of your project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
