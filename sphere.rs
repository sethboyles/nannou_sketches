use nannou::prelude::*;
use std::ops::Range;

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
    
    let r = 400.0;
    let total = 50;
    let point_range = Range {start: 0.0, end: total as f32 };
    let iter_range = Range { start: point_range.start as i32, end: point_range.end as i32 };
    let pi_range = Range { start: -PI, end: PI };
    let half_pi_range = Range { start: PI * -0.5, end: PI * 0.5 };

    for i in iter_range {
      let lon = map_range(i as f32, &point_range, &pi_range);  

      let iter_range2 = Range { start: point_range.start as i32, end: point_range.end as i32 };
      
      for j in iter_range2 {
        
        let lat = map_range(j as f32, &point_range, &half_pi_range);
          
        let x = r * lon.sin() * lat.cos();
        let y = r * lon.sin() * lat.sin();
        let z = r * lon.cos();

        let mut point = pt3(x,y,z);
        point = rotate_point_x(&point, model.x_rotation);
        point = rotate_point_y(&point, model.y_rotation);
        draw.tri()
            .color(WHITE)
            .w(1.0)
            .h(1.0)
            .x_y_z(point.x, point.y, point.z.abs() );
      }
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
