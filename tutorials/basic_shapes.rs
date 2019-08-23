use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background.
    draw.background().color(PLUM);


    
    let point1 = pt2(-10.0, -20.0);
    let point2 = pt2(10.0, -30.0);
    let point3 = pt2(15.0, 40.0);
    let point4 = pt2(-20.0, 35.0);

    draw.quad()
        .color(STEELBLUE)
        .w(300.0)
        .h(200.0)
        .points(point1, point2, point3, point4); 

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap(); 
}
