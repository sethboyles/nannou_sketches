use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
}

struct Circle {
  x: i32,
  y: i32,
  radius: f32,
  line_color: Rgba,
  fill_color: Rgba,
}

impl Circle {
  fn render(&self, draw: &nannou::draw::Draw) {
    draw.ellipse().w(self.radius).h(self.radius);
  }
}

fn model(app: &App) -> Model {
  let _window = app.new_window()
                   .view(view)
                   .build()
                   .unwrap();
  Model {
  }
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
}

fn update(_app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background.
    draw.background().color(BLACK);

    let circle = Circle {
      x: 1,
      y: 1,
      radius: 100.0,
      line_color: srgba(0.0,0.0,0.0,0.0),
      fill_color: srgba(0.0,0.0,0.0,0.0),
    };
    circle.render(&draw);
    draw.to_frame(app, &frame).unwrap();
}