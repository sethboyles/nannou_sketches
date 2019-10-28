use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
  back_mountain:   Mountain,
  middle_mountain: Mountain,
  front_mountain:  Mountain,
  clock: i32,
}

struct Circle {
  x: f32,
  y: f32,
  radius: f32,
  line_color: Rgb<u8>,
  fill_color: Rgb<u8>,
  }

impl Circle {
  fn render(&self, draw: &nannou::draw::Draw) {
    draw.ellipse().w(self.radius)
                  .h(self.radius)
                  .color(self.fill_color)
                  .x_y(self.x, self.y);
  }
}

struct Mountain {
  points: Vec<nannou::geom::vertex::Srgba<nannou::geom::vector::Vector2>>,
  seed: u32,
}

impl Mountain {
  fn render(&self, draw: &nannou::draw::Draw) {
    for point in &self.points {
      draw.line().start(pt2(point.x, point.y - 1.0))
                 .end(pt2(point.x, point.y - 500.0))
                 .color(BLACK);
    }
    draw.polyline().vertices(1.0, &self.points);
  }
}

fn generate_points(clock: f32, x_offset: f32, y_offset: f32, seed: u32) ->  Vec<geom::vertex::Srgba<geom::vector::Vector2>> {
  let num_points = 3000; 
  let noise = Perlin::new();

  noise.set_seed(seed);
  let points = (0..num_points).step_by(1).map(|i| {
    let x = (i) as f32 - x_offset;
    let clock_step = clock as f32 * 0.01;
    let Y = i as f32 * 0.01 + clock_step;
   
    let noise2_freq = 0.07;
    let y_noise  = noise.get([seed as f64, i as f64 * 0.01 + clock_step as f64 ])  as f32;
    let y_noise2 = noise.get([(seed + 20) as f64, i as f64 * noise2_freq + clock as f64 * noise2_freq ])  as f32;
    
    let mut y = (i as f32 * 0.01 + clock_step).sin() * 100.0 + y_noise / 0.01 + y_noise2 * 5.0 ;

    if y > 0.0 {
      y = y + (y.powf(3.0)) / 35000.0;
    }

    if y < 0.0 {
     y = y / 1.6;
    }
 
    pt2(x * 2.0, y + y_offset)

  }).collect::<Vec<_>>();
  
  points.clone().into_iter().enumerate().map(|(i, point)| {
    let mut color = srgba(1.0, 1.0, 1.0, 1.0);
    if i > 0 {
      let point2 = points[i - 1];
      let slope = (point2.y - point.y) / (point2.x - point.x);

      if slope < 0.9 {
        color = srgba(0.6, 0.7, 1.0, 1.0);
      }

      if slope < 0.3 {
        color = srgba(0.4, 0.5, 0.7, 1.0);
      }

      if slope < -0.6 {
        color = srgba(0.2, 0.2, 0.4, 1.0);
      }

      if false && point.y < -200.0 && slope > -1.1 && slope < 1.0 {
        color = srgba(0.0,0.0,0.0,0.0);
      }
    }
    geom::vertex::Srgba(point, color)
    
  }).collect::<Vec<_>>()
}

fn model(app: &App) -> Model {
  let _window = app.new_window()
                   .view(view)
                   .build()
                   .unwrap();
  let vertices = generate_points(0.0, app.window_rect().w(), 0.0, 1);
  let mountain = Mountain {
    points: generate_points(0.0, app.window_rect().w() - 100.0, 0.0, 2 ),
    seed: (random_f32() * 100000.0) as u32,
  };

  let middle_mountain = Mountain {
    points: generate_points(0.0, app.window_rect().w() - 100.0, 0.0, 2 ),
    seed: (random_f32() * 100000.0) as u32,
  };

  let front_mountain = Mountain {
    points: vertices,
    seed: (random_f32() * 100000.0) as u32,
  };
 
  Model {
    back_mountain: mountain,
    middle_mountain: middle_mountain,
    front_mountain: front_mountain,
    clock: 0,
  }
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
}

fn update(app: &App, model: &mut Model, _update: Update) {
  model.clock += 1;
  model.back_mountain.points = generate_points(model.clock as f32 * 0.4, app.window_rect().w(), -300.0, model.back_mountain.seed);
  model.middle_mountain.points = generate_points(model.clock as f32 * 0.7, app.window_rect().w(), -250.0, model.middle_mountain.seed);
  model.front_mountain.points = generate_points(model.clock as f32, app.window_rect().w() - 200.0, -200.0, model.front_mountain.seed);
}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    
    // Clear the background.
    draw.background().color(BLACK);
    let circle = Circle {
      x: 32.0,
      y: 32.0,
      radius: 32.0,
      line_color: STEELBLUE,
      fill_color: STEELBLUE,
    };
    circle.render(&draw);
    model.back_mountain.render(&draw);
    model.middle_mountain.render(&draw);
    model.front_mountain.render(&draw);
    draw.to_frame(app, &frame).unwrap();
}
