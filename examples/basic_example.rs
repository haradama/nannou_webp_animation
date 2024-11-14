use nannou::prelude::*;
use nannou_webp_animation::WebpAnimation;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    animation: WebpAnimation,
    rotation: f32,
    angular_velocity: f32,
    scale: f32,
}

fn model(app: &App) -> Model {
    // Create a new window
    app.new_window().size(800, 600).build().unwrap();

    // Load the WEBP animation
    let assets = app.assets_path().unwrap();
    let webp_path = assets.join("animation.webp"); // Ensure 'animation.webp' is in the assets directory

    // Initialize the animation
    let animation =
        WebpAnimation::from_file(&webp_path, app).expect("Failed to load WEBP animation");

    Model {
        animation,
        rotation: 0.0,
        angular_velocity: 0.01, // Adjust as needed
        scale: 1.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Update the animation frames
    model.animation.update();

    // Update rotation
    model.rotation += model.angular_velocity;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw
    let draw = app.draw();
    draw.background().color(WHITE);

    // Draw the animation with rotation and scaling
    model
        .animation
        .draw(&draw, pt2(0.0, 0.0), model.scale, model.rotation);

    // Render the frame
    draw.to_frame(app, &frame).unwrap();
}
