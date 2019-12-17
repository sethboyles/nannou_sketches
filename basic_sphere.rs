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
