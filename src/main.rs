use nannou::prelude::*;

struct Player {
    pos: Point2,
    vel: f32,
    w: f32,
    h: f32,
}

impl Player {
    fn new(y: f32, side: PlayerSide) -> Player {
        let mut x = 0.;
        if let PlayerSide::Right = side {
            x = 380.;
        } else {
            x = -380.;
        }
        Player {
            pos: pt2(x, y),
            vel: 0.0,
            w: 20.0,
            h: 100.0,
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.rect()
            .x_y(self.pos.x - self.w / 2., self.pos.y - self.h / 2.)
            .w_h(self.w, self.h)
            .color(WHITE);
    }

    fn update(&mut self) {
        self.pos.y += self.vel * 5.;
    }
}

struct Ball {
    pos: Point2,
    vel: Point2, 
}

impl Ball {
    fn new() -> Ball {
        Ball {
            pos: pt2(0.0, 0.0),
            vel: pt2(-1.0, 1.0),
        }
    }

    fn update(&mut self, (w, h): (f32, f32), positions: (Point2, Point2)) {
        self.pos += self.vel * 5.0;

        if self.pos.x < -w / 2.0 || self.pos.x > w / 2.0 {
            self.vel.x *= -1.0;
        }
        
        if self.pos.y < -h / 2.0 || self.pos.y > h / 2.0 {
            self.vel.y *= -1.0;
        }

        if self.pos.x < positions.0.x + 10.0 && self.pos.y < positions.0.y + 50.0 && self.pos.y > positions.0.y - 50.0 {
            self.vel.x *= -1.0;
        }

        if self.pos.x > positions.1.x - 10.0 && self.pos.y < positions.1.y + 50.0 && self.pos.y > positions.1.y - 50.0 {
            self.vel.x *= -1.0;
        }

        self.vel = self.vel.normalize();
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .x_y(self.pos.x, self.pos.y)
            .radius(10.0)
            .color(WHITE);
    }
}

struct Model {
    window: WindowId,
    player: Player,
    cpu: Player,
    ball: Ball,
}

impl Model {
    fn get_loc_tuple(&self) -> (Point2, Point2) {
        (self.player.pos, self.cpu.pos)
    }
}

enum PlayerSide {
    Left,
    Right,
}

fn model(app: &App) -> Model {
    let window = app.new_window()
        .size(800, 800)
        .title("Pong")
        .build().unwrap();
    let ball = Ball::new();
    let player = Player::new(0.0, PlayerSide::Left);
    let cpu = Player::new(0.0, PlayerSide::Right);

    Model { window, ball, cpu, player }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let dim = _app.window(model.window).unwrap().inner_size_pixels();

    model.player.update();
    model.cpu.update();

    model.ball.update((dim.0 as f32, dim.1 as f32), model.get_loc_tuple());

    if model.ball.pos.y < model.cpu.pos.y {
        model.cpu.vel = -1.0;
    } else if model.ball.pos.y > model.cpu.pos.y {
        model.cpu.vel = 1.0;
    } else {
        model.cpu.vel = 0.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.ball.draw(&draw);
    model.player.draw(&draw);
    model.cpu.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple: Some(event), .. } => {
            match event {
                KeyPressed(key) => {
                    match key {
                        Key::W => model.player.vel = 1.0,
                        Key::S => model.player.vel = -1.0,
                        _ => (),
                    }
                }
                KeyReleased(key) => {
                    match key {
                        Key::W => model.player.vel = 0.0,
                        Key::S => model.player.vel = 0.0,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        _ => (),
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .event(event)
        .run();
}

