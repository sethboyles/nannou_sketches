use nannou::prelude::*;
fn main() {
    nannou::app(model).run();
}

struct Model {
    // Store the window ID so we can refer to this specific window later if needed.
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let _window = app
        .new_window()
        .with_dimensions(956, 520)
        .with_title("nannou")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .build()
        .unwrap();
    Model { }
}

fn view(app: &App, _model: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app
        .draw();
    let start_point = pt2(-30.0, -20.0);
let end_point   = pt2(40.0, 40.0);

draw.line()
    .start(start_point)
    .end(end_point)
    .weight(4.0)
    .color(STEELBLUE);
       // Clear the background.
    draw.background().color(PLUM);
    draw.to_frame(app, &frame).unwrap();

}
