use nannou::prelude::*;
use nannou::app::Draw;
use core::iter::Map;
use std::cell::RefCell;
use nannou::noise::{NoiseFn, Perlin, OpenSimplex, Seedable};

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
  circle_steps: f32,
  incrementing: bool,
  rotation: f32,
  seed: u32,
  noise_step: f64,
}


fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(30.0));
     let _window = app
         .new_window()
         .with_dimensions(800, 400)
         .view(view)
         .key_released(key_released)
         .build()
         .unwrap();
     Model {
       circle_steps: 1.0,
       incrementing: true,
       rotation: 0.0,
       noise_step: 0.01,
       seed: (random_f64() * 1000.0) as u32
     }

 }

fn key_released(_app: &App, model: &mut Model, key: Key) {
     if key == nannou::winit::VirtualKeyCode::Down {
       model.noise_step -= 0.001; 
     }

     if key == nannou::winit::VirtualKeyCode::Up {
       model.noise_step += 0.001; 
     }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
  

  model.rotation += 0.1;
  if model.incrementing {
    if false && model.circle_steps == 360.0{
      model.incrementing = false;

    } else {
      model.circle_steps += 0.01;
    }
  } else {
    if model.circle_steps == 1.0 {
      model.incrementing = true;

    } else {
      model.circle_steps -= 0.01;
    }

  }
} 

fn custom_noise(value: f32) -> f32 {
  let pow = value as i32 % 11;
  value.sin().powi(pow)  
}
fn circle_points<'a>(radius: &f32) -> Vec<geom::vector::Vector2> {

  let mut new_radius = *radius;
  let noise = OpenSimplex::new();
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
     
    let noise = OpenSimplex::new();

    noise.set_seed(model.seed);
    let grid_size = 75;
    
    let cell_size = 15.0;
    let cell_half = cell_size * 2.0;
    let  xstart = 0.5;
    let mut xnoise = xstart;
    let mut ynoise = 0.02;
    let grid_width = cell_size * grid_size as f32;
    for i in 0..grid_size {
      ynoise += model.noise_step;
      xnoise = xstart;
      for j in 0..grid_size {
         let pt_noise = noise.get([model.circle_steps as f64,xnoise, ynoise]);
         draw_point_as_cloud(&draw, i, j, pt_noise, cell_size, grid_width);
         xnoise += model.noise_step;

      }
    }


    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn draw_point_as_rot(draw: &app::Draw, i: i32, j: i32, noise_factor: f64, cell_size: f32, grid_width: f32) {
    let start_x = cell_size * i as f32 - grid_width * 0.5;
    let start_y = cell_size * j as f32 - grid_width * 0.5;
    let rotation = (noise_factor as f32 * 2.0 * PI);
    let length = 20.0;
    let start_pt = pt2(rotation.sin() * length - start_x, rotation.cos() * length - start_y);
    let end_pt = pt2((rotation + PI).sin() * length - start_x, (rotation +  PI).cos() * length - start_y);

    draw.line().start(start_pt)
        .end(end_pt)
        .thickness( 1.0)
        .color(WHITE);
}

fn draw_point_as_box(draw: &app::Draw, i: i32, j: i32, noise_factor: f64, cell_size: f32, grid_width: f32) {
    let start_x = cell_size * i as f32 - grid_width * 0.5;
    let start_y = cell_size * j as f32 - grid_width * 0.5;
    let width = cell_size;
    let size = width * noise_factor as f32;
    draw.rect()
        .x_y(start_x, start_y)
        .w(size)
        .h(size)
        .color(WHITE);
}

fn draw_point_as_circle(draw: &app::Draw, i: i32, j: i32, noise_factor: f64, cell_size: f32, grid_width: f32) {
    let start_x = cell_size * i as f32 - grid_width * 0.5;
    let start_y = cell_size * j as f32 - grid_width * 0.5;
    let width = cell_size;
    let size = width * noise_factor as f32;
    draw.ellipse()
        .x_y(start_x, start_y)
        .w(size)
        .h(size)
        .color(WHITE);
}

fn draw_point_as_tri(draw: &app::Draw, i: i32, j: i32, noise_factor: f64, cell_size: f32, grid_width: f32) {
    let start_x = cell_size * i as f32 - grid_width * 0.5;
    let start_y = cell_size * j as f32 - grid_width * 0.5;
    let width = cell_size;
    let size = width * noise_factor as f32;
    draw.quad()
        .x_y(start_x, start_y)
        .w(size)
        .h(size)
        .color(WHITE);
}


fn draw_point_as_cloud(draw: &app::Draw, i: i32, j: i32, noise_factor: f64, cell_size: f32, grid_width: f32) {
    let start_x = cell_size * i as f32 - grid_width * 0.5;
    let start_y = cell_size * j as f32 - grid_width * 0.5;
    let new_noise = noise_factor * 0.4 + 0.6 ;
    let width = cell_size * 5.0;
    let size = width * (1.0 - new_noise) as f32;
    let rgba = srgba(new_noise,new_noise, new_noise, new_noise);
    draw.rect()
        .x_y(start_x, start_y)
        .w(size )
        .h(size )
        .color(rgba);

}
