use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    let radius = 100.0;
    // Clear the background.
    draw.background().color(BLACK);
    let s = 0.0;
    
    let lastx = 0.0;
    let lasty = 0.0;
    let lastz = 0.0;

    for t in 1..180 {
      s += 18.0;
    
      let radian_s = deg_to_rad(s as f32);
      let radian_t = deg_to_rad(t as f32);
      
      let thisx = radius * radian_s.cos() * radian_t.sin();
      let thisy = radius * radian_s.sin() * radian_t.sin();
      let thisz = radius * radian_t.cos();

      
      if lastx != 0.0 {

        draw.line().start(pt3(lastx, lasty, lastz ))
                   .end(pt3(thisx, thisy, thisz));
      }



    }
}
