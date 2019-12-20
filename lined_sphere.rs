use nannou::prelude::*;
use std::ops::Range;
use nannou::noise::{NoiseFn, Perlin};
fn main() {
    nannou::app(model).update(update).run();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
  
}

const STEP_AMOUNT: f32 = 0.05;
fn key_released(_app: &App, model: &mut Model, key: Key) {
  if key == nannou::winit::VirtualKeyCode::Down {
    model.x_rotation -= STEP_AMOUNT;
  }
  if key == nannou::winit::VirtualKeyCode::Up {
    model.x_rotation += STEP_AMOUNT;
  }
  if key == nannou::winit::VirtualKeyCode::Left {
    model.y_rotation -= STEP_AMOUNT;
  }
   if key == nannou::winit::VirtualKeyCode::Right {
    model.y_rotation += STEP_AMOUNT;
  }
}

fn map_range(value: f32, from_range: &Range<f32>, to_range: &Range<f32>) -> f32 {
  let slope = (to_range.end - to_range.start) / (from_range.start - from_range.end);
  to_range.start + slope * (value - from_range.start)
}


struct Model {
  x_rotation: f32,
  y_rotation: f32
}

struct Slice {
  points: Vec<Point3>
}

fn model(app: &App) -> Model {
  app.set_loop_mode(LoopMode::rate_fps(60.0));
  let _window = app.new_window()
                   .view(view)
                   .key_pressed(key_released)
                   .build()
                   .unwrap();
  Model {
    x_rotation: PI,
    y_rotation: PI,
  }
  
}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    // Clear the background.
    draw.background().color(BLACK);
    let noise = nannou::noise::Perlin::new();
    let r = 200.0;
    let total = 50;
    let point_range = Range {start: 0.0, end: total as f32 };
    let iter_range = Range { start: point_range.start as i32, end: point_range.end as i32 };
    let pi_range = Range { start: 0.0, end: PI};
    let half_pi_range = Range { start: 0.0, end: 2.0 * PI };
    let mut slices = vec![];
    for i in iter_range {
      let lat = map_range(i as f32, &point_range, &pi_range);  

      let iter_range2 = Range { start: point_range.start as i32, end: point_range.end as i32 };
      
      let points = iter_range2.map(|j| {
        
        let lon = map_range(j as f32, &point_range, &half_pi_range);
          
        let mut x = r * lat.sin() * lon.cos();
        let y = r * lat.sin() * lon.sin();
        let mut z = r * lat.cos();
        x += (noise.get([i as f64 * 0.1, j as f64 * 0.1]) * 30.0) as f32;

        let mut point = pt3(x,y,z);
        point = rotate_point_x(&point, model.x_rotation);
        point = rotate_point_y(&point, model.y_rotation);
        point
      });
      let slice = Slice { points: points.collect() };
      slices.push(slice);
    }

    for (i, slice) in slices.into_iter().enumerate() {
      let points = slice.points.into_iter().map( |point| {
        if point.z.is_positive() {

          (pt2(point.x, point.y), WHITE)
        } else {
          (pt2(point.x, point.y), BLACK)
        }
      });
      draw.polyline()
          .weight(1.5)
          .colored_points_closed(points)
          .color(WHITE);
     // for point in slice.points {
     //    draw.polyline(
     //    draw.tri()
     //      .color(WHITE)
     //      .w(1.0)
     //      .h(1.0)
     //      .x_y_z(point.x, point.y, point.z.abs() );


     // }

    }
    draw.to_frame(app, &frame).unwrap();
  }

fn rotate_point_x(point: &Point3, radians: f32) -> Point3 {
  let x = point.x;
  let y = point.y;
  let z = point.z;
  let mut new_point = pt3(0.0,0.0,0.0);
  new_point.x = x;
  new_point.y = y * radians.cos() - z * radians.sin();
  new_point.z = y * radians.sin() + z * radians.cos();
  new_point
}

fn rotate_point_y(point: &Point3, radians: f32) -> Point3 {
  let x = point.x;
  let y = point.y;
  let z = point.z;
  let mut new_point = pt3(0.0,0.0,0.0);
  new_point.x = x * radians.cos() + z * radians.sin();
  new_point.y = y;
  new_point.z = z * radians.cos() - x * radians.sin();
  new_point
}

fn rotate_point_z(point: &Point3, radians: f32) -> Point3 {
  let x = point.x;
  let y = point.y;
  let z = point.z;
  let mut new_point = pt3(0.0,0.0,0.0);
  new_point.x = x * radians.cos() - y * radians.sin();
  new_point.y = x * radians.sin() + y * radians.cos();
  new_point.z = z;
  new_point
}
