use nannou::prelude::*;
use nannou::app::Draw;
use nannou::geom::*;
use core::iter::Map;
use std::cell::RefCell;
use nannou::noise::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
  circle_steps: usize,
  incrementing: bool,
  //vertices: nannou::draw::Vertices,
}

fn model(app: &App) -> Model {
   // app.set_loop_mode(LoopMode::loop_once());
     let _window = app
         .new_window()
         .with_dimensions(800, 400)
         .view(view)
         .key_released(key_released)
         .build()
         .unwrap();
     Model {
       circle_steps: 1,
       incrementing: true,
      // vertices: nannou::draw::Vertices::new(),
     }

 }

fn key_released(_app: &App, model: &mut Model, key: Key) {
     if key == nannou::winit::VirtualKeyCode::Down {

       model.circle_steps -= 1; 
     }

     if key == nannou::winit::VirtualKeyCode::Up {
       model.circle_steps += 1; 
     }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
  if model.incrementing {
    if model.circle_steps == 180 {
      model.incrementing = false;
    } else {
      model.circle_steps += 1;
    }
  } else {
    if model.circle_steps == 1 {
      model.incrementing = true;

    } else {
      model.circle_steps -= 1;
    }

  }
} 

fn custom_noise(value: f32) -> f32 {
  let pow = value as i32 % 11;
  value.sin().powi(pow)  
}
// fn circle_points<'a>(radius: &f32) -> impl Iterator< Item = &eom::vertex::Srgba<nannou::geom::vector::Vector2>> {
fn circle_points<'a>(radius: &f32) -> Vec<geom::vector::Vector2> {

  let mut new_radius = *radius;
  let noise = Perlin::new();
  let mut radius_noise = 0.0;

  let points = (0..361).step_by(1).map( |p| {
      
      let new_noise = noise.get([1.0, radius_noise as f64]) as f32;
      let cust_noise = custom_noise(radius_noise);
      let to_add = cust_noise * 10.0;
      radius_noise += 0.1;
      let rad = deg_to_rad(p as f32);
      let x = radius * rad.sin();
      let y = radius * rad.cos();
      pt2(x,y)
      
   });
   points.collect::<Vec<_>>()
}

fn view(app: &App,  model: &Model, frame: &Frame) {
    // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);
    let offset = 40;
    let win_x = win.w() / 2.0; 
    let win_y = win.h() / 2.0;
    
    let radius = 400.0;
      
    let rgba = srgba(1.0, 1.0, 1.0,1.0);
    let points = circle_points(&radius);
    let vertexes = points.clone().into_iter().map( |p| {
      
      geom::vertex::Srgba(p, rgba)

    }).collect::<Vec<_>>();
     
    let vertices = points.clone().into_iter().map( |p| {
       geom::vertex::Srgba(p, rgba)
    }).collect::<Vec<_>>();

    for (i, vertex) in points.iter().step_by(2).enumerate() {
      let index = &vertexes.len() - i - 1;
      let verts = [vertex, &points[index]];
      let points = verts.iter().map(|point| {
        pt2(point.x, point.y)
      });
      draw.polyline().points(points); 
    }
   draw.to_frame(app, &frame).unwrap();
}
