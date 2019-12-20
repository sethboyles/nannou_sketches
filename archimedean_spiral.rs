use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
  num: i32
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
    let radius = 150.0;                   // store the radius of the circle we want to make
    let points = (0..=360).map(|i| {      // map over an array of integers from 0 to 360 to represent the degrees in a circle

       let radian = deg_to_rad(i as f32); // convert each degree to radians
       let x = radian.sin() * radius;     // get the sine of the radian to find the x-co-ordinate of
                                          // this point of the circle, and multiply it by the radius
       let y = radian.cos() * radius;     // do the same with cosine to find the y co-ordinate
       (pt2(x,y), STEELBLUE)              // construct and return a point object with a color
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .colored_points(points);  
  }
}


fn model(app: &App) -> Model {
  let _window = app.new_window()
                   .view(view)
                   .key_pressed(key_released)
                   .build()
                   .unwrap();
  Model {
    num: 1
  }
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
  if key == nannou::winit::VirtualKeyCode::Down {
    model.num -= 1;
  } 
  if key == nannou::winit::VirtualKeyCode::Up {
    model.num += 1;
  }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    
    // Clear the background.
    draw.background().color(BLACK);
    let grid_size = 200;
    let range = (1..(grid_size * grid_size));
    let spacing = 5;
    let radius = 1;
    let mut x = 0;
    let mut y = 0;
    let mut edge_length = 2;
    let mut edge_point = 1;
    
    let mut x_direction = spacing;
    let mut y_direction = 0;
    let mut edge_length_occurrence = 0;
    let mut radius = 0.0;
    let mut theta = 0.0;
    for number in range {
        
      let x = radius * theta.cos();
      let y = radius * theta.sin();

      if number % model.num  == 0 { //is_prime(number){
        
        draw.ellipse().color(WHITE).w(5.0).h(5.0).x_y(x as f32, y as f32);
      }

      theta += 0.01;
      radius += 0.01;
    }     
    draw.to_frame(app, &frame).unwrap();
}

fn is_perfect_square(n: u32) -> bool {
  let s = (n as f64).sqrt();
  (s as u32) * (s as u32) == n
}

fn is_fibonacci(n: u32) -> bool {

  is_perfect_square(5 * n * n + 4) || is_perfect_square(5 * n * n - 4)

}

fn is_prime(n: u32) -> bool {
    for a in 2..n {
        if n % a == 0 {
            return false; 
        }
    }
    true 
}
