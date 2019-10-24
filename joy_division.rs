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

fn binom(x: f32)  -> f32{
 let max = 45.0;
 let mut num =  (max - x.abs().sqrt() * 2.9 as f32);
 if num < 0.0 {
   num = 0.0;
 }
 num
}

impl Mountain {
  fn render(&self, draw: &nannou::draw::Draw, noise: &nannou::noise::Perlin) {

    let radius = 150.0;
    let points = (0..=1000).map(|i| {
       let radian = deg_to_rad(i as f32);
       let x = (i - 500) as f32 * 0.8;
       let mut y = radian.cos() * 50.0;
       y =  binom(x) * noise.get([(x as f64 + 0.3) * 0.03, (self.y as f64 * 0.2)]) as f32 + binom(x + 0.3) + self.y as f32;
       pt2(x,y)
    });
      let poly_points = points.clone().map(|point| {
        if point.x <= -500.0 * 0.8 {
          pt2(point.x, point.y - 50.0)
        } else if point.x >= 500.0 * 0.8 {
          pt2(point.x, point.y - 50.0)
        } else {
          point
        }
      });
      draw.polygon()
          .color(BLACK)
          .points(poly_points.clone());

    let colored_points = points.map(|point| {
      (point.clone(), STEELBLUE)
    });

    draw.polyline()
        .weight(0.5)
        .colored_points(colored_points);
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
    let y_start = 300;
    let y_spacing = 15;
    let noise = nannou::noise::Perlin::new();
    for i in 0..30 {
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
