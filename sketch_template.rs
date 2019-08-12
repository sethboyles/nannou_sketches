use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background.
    draw.background().color(BLACK);

}
