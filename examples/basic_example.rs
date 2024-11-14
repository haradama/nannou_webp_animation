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
    let webp_path = assets.join("animation.webp"); // Place 'animation.webp' in the 'assets' directory

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
