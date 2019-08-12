use nannou::prelude::*;
use nannou::app::Draw;
use core::iter::Map;
use std::cell::RefCell;
use nannou::noise::{Perlin, NoiseFn};

fn main() {
    nannou::app(model).update(update).run();
}
fn draw_poly(draw: &Draw, t: f32, x: i32, y: i32){
    let radius = 15.0;
    let n_points = 7;
    let points = (0..n_points).map(|i| {
      let fract = i as f32 / n_points as f32;
      let phase = fract;
      let xx = radius * (TAU * phase).cos();
      let yy = radius * (TAU * phase).sin();
      pt3(xx, yy, 0.0)
    });
    draw.polygon()
      .points(points)
      .x(x as f32)
      .y(y as f32)
      .color(WHITE)
      .rotate(-t * 0.1);
}
struct Model {
  circle_steps: usize,
  incrementing: bool,
}


fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(60.0));
     let _window = app
         .new_window()
         .with_dimensions(800, 400)
         .view(view)
         .key_released(key_released)
         .build()
         .unwrap();
     Model {
       circle_steps: 1,
       incrementing: true
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
    if model.circle_steps == 360 {
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
    
    let radius = 250.0;
      
    let rgba = srgba(1.0, 1.0, 1.0,0.0);
    
    let rgba2 = srgba(0.0, 0.3, 1.0,1.0);
    let points = circle_points(&radius);
    let vertexes = points.clone().into_iter().map( |p| {
      
      geom::vertex::Srgba(p, rgba)

    }).collect::<Vec<_>>();
     

    let vertices = points.clone().into_iter().map( |p| {

       let pt = pt2(p.x, p.y);
       geom::vertex::Srgba(pt, rgba)
    }).collect::<Vec<_>>();

    for (i, point) in points.iter().enumerate() {
      if (i <= 360) {
        
      let index = &vertexes.len();

      let point1 = geom::vertex::Srgba(point.clone(), rgba);
      let point2 = geom::vertex::Srgba(points[(i + model.circle_steps) % 360].clone(), rgba2);
      draw.polyline().vertices(0.2, &[&point1, &point2]); 
      }
    }

//    for (i, vertex) in vertexes.iter().step_by(model.circle_steps).enumerate() {
//      //let index = (random_f32() * (vertexes.len() as f32)) as usize;
//      let index = &vertexes.len() - i - 1;
//      draw.polyline().vertices(0.2, &[&vertex, &vertexes[index]]); 
//    }  
    //draw.polyline().vertices(1.0, vertices);
    //draw.polygon().points(points.into_iter().map(|p| { pt2(p.x + 100.0, p.y + 100.0)  } ));

//    for x in 0..100 {
//    //  for y in 0..10 {
//        let x_offset = x * offset - (win.w() / 4.0 ) as i32;
//   //     let y_offset = y * offset - (win.h() / 4.0 ) as i32;
//        let vertices = (0..100)
//             .map(|i| {
//                let x_pos = 20.0 * i as f32 - win_x;
//                let y_pos = 10.0 * x as f32 - win_y;
//                let p = pt2(x_pos, y_pos);
//                let rgba = srgba(1.0, 1.0, 1.0,1.0);
//
//                geom::vertex::Srgba(p, rgba)
//             });
//        draw.polyline().vertices(0.5, vertices);

         //   draw_poly(&draw, t, x_offset, y_offset);
//    }
    //}
   // Create an `ngon` of points
   

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
