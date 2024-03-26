//Seven Segment Display

use nannou::prelude::*;

struct Model {
    window: WindowId,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(800, 800)
        .view(view)
        .event(event)
        .build().unwrap();
    Model { window }
}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {
    println!("{:?}", _event);
}

fn from_corner_to_center(x: f32, y:f32, w:f32, h:f32) -> (f32, f32) {
    let x0 = x + w/2.;
    let y0 = y - h/2.;

    return (x0, y0);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // let draw = app.draw();
    let draw = app.draw();//.x_y(-400., 400.).scale_y(-1.0);

    draw.background().color(CORNFLOWERBLUE);

    let s = 20.;
    let l = 120.;

    //--
    // 1
    draw.rect()
        .w_h(l, s)
        .x_y(from_corner_to_center(s, 0., l, s).0, from_corner_to_center(s, 0., l, s).1)
        .color(PINK);

    //2
    draw.rect()
        .w_h(l, s)
        .x_y(from_corner_to_center(s, -(s + l), l, s).0, from_corner_to_center(s, -(s + l), l, s).1)
        .color(PINK);

    //3
    draw.rect()
        .w_h(l, s)
        .x_y(from_corner_to_center(s, - (2.*s + 2.*l), l, s).0, from_corner_to_center(s, - (2.*s + 2.*l), l, s).1)
        .color(PINK);


    // |
    //4
    draw.rect()
        .w_h(s, l)
        .x_y(from_corner_to_center(0., -s, s, l).0, from_corner_to_center(0., -s, s, l).1)
        .color(VIOLET);

    // //5
    draw.rect()
        .w_h(s, l)
        .x_y(from_corner_to_center(s + l, -s, s, l).0, from_corner_to_center(s + l, -s, s, l).1)
        .color(VIOLET);
    //
    // //6
    draw.rect()
        .w_h(s, l)
        .x_y(from_corner_to_center(0., -(2.*s + l), s, l).0, from_corner_to_center(0., -(2.*s + l), s, l).1)
        .color(VIOLET);

    // //7
    draw.rect()
        .w_h(s, l)
        .x_y(from_corner_to_center(s + l, -(2.*s + l), s, l).0, from_corner_to_center(s + l, -(2.*s + l), s, l).1)
        .color(VIOLET);


    draw.ellipse()
        .w_h(10., 10.)
        .color(RED);

    // draw.rect()
    //     .w_h(10., 10.)
    //     .x_y(0.0, 0.0)
    //     .color(BLACK);
    //
    // draw.rect()
    //     .w_h(w, h)
    //     .x_y(h, h + w)
    //     .color(WHITE);
    //
    // draw.rect()
    //     .w_h(w, h)
    //     .x_y(h, 2.*h + 2.*w)
    //     .color(WHITE);
    //
    // draw.rect()
    //     .w_h(h, w)
    //     .x_y(0.0, w)
    //     .color(WHITE);

    // draw.rect()
    //     .w_h(h, w)
    //     .x_y(0.0, 2.*w + h)
    //     .color(WHITE);
    //
    // draw.rect()
    //     .w_h(h, w)
    //     .x_y(w + h, w)
    //     .color(WHITE);
    //
    // draw.rect()
    //     .w_h(h, w)
    //     .x_y(w + h, 2.*w + h)
    //     .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // println!("{:?}", _update);
}

fn main() {
    nannou::app(model)
        .update(update)
        .run(); 
}

