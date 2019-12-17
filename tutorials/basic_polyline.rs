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
    let draw = app.draw();

    draw.background().color(PLUM);

    let points = (0..50).map(|i| {
      let x = (i as f32 - 25.0);          //subtract 25 to center the sine wave
      let point = pt2(x, x.sin()) * 20.0; //scale sine wave by 20.0
      (point, STEELBLUE)
    });

    draw.polyline()
        .weight(3.0)
        .colored_points(points);
    draw.to_frame(app, &frame).unwrap();
}
