use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
  
}

struct Mountain {
  id: i32,
  x: i32,
  y: i32,
  radius: f32,
  line_color: Rgba,
  fill_color: Rgba,
}

impl Mountain {
  fn render(&self, draw: &nannou::draw::Draw, noise: &nannou::noise::Perlin) {
    
    let radius = 150.0;
    let points = (0..=1200).map(|i| {
       let radian = deg_to_rad(i as f32);
       let x = (i - 600) as f32 * 0.8;
       let mut y = radian.cos() * 50.0;
       y = 15.0 * noise.get([x as f64 * 0.1, (self.y as f64 * 0.002)]) as f32 + self.y as f32;
       (pt2(x,y), STEELBLUE)
    });
    draw.polyline()
        .weight(0.5)
        .colored_points(points);
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
    let y_start = 500;
    let y_spacing = 15;
    let noise = nannou::noise::Perlin::new();
    for i in 0..100 {
      let mountain = Mountain {
        id: i,
        x: 0,
        y: y_start - i * y_spacing,
        radius: 100.0,
        line_color: srgba(0.0,0.0,0.0,0.0),
        fill_color: srgba(0.0,0.0,0.0,0.0),
      };
      mountain.render(&draw, &noise);
    }
    draw.to_frame(app, &frame).unwrap();
}
