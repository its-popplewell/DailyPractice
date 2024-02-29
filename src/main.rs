use nannou::prelude::*;

enum UpdateResult {
    DIE,
    EAT,
    CONTINUE,
}

struct Model {
    window: WindowId,
    snek: Snek,
    food: Food,
    grid_size: usize
}

struct SnekPart {
    x: isize,
    y: isize,
}

struct Snek {
    parts: Vec<SnekPart>,
    len: usize,
    vel: (isize, isize)
}

impl Snek {
    fn new() -> Snek {
        Snek {
            parts: vec![SnekPart { x: 10, y: 10 }],
            len: 1,
            vel: (0, 0)
        }
    }

    fn get_body_loc(&self) -> Vec<(isize, isize)> {
        self.parts.iter().map(|part| (part.x, part.y)).collect()
    }

    fn move_around(&mut self) {
        let mut prev_x = self.parts[0].x;
        let mut prev_y = self.parts[0].y;

        self.parts[0].x = (self.parts[0].x + self.vel.0);
        self.parts[0].y = (self.parts[0].y + self.vel.1);

        for part in self.parts.iter_mut().skip(1) {
            let tx = part.x;
            let ty = part.y;
            part.x = prev_x;
            part.y = prev_y;
            prev_x = tx;
            prev_y = ty;
        }
    }

    fn update(&mut self, food: &Food) -> UpdateResult {
        self.move_around();
        let head = &self.parts[0];

        if head.x == food.x && head.y == food.y {
            println!("ate food");
            self.len += 1;
            if self.vel == (1, 0) {
                self.parts.push(SnekPart { x: head.x - 1, y: head.y });
            } else if self.vel == (-1, 0) {
                self.parts.push(SnekPart { x: head.x + 1, y: head.y });
            } else if self.vel == (0, 1) {
                self.parts.push(SnekPart { x: head.x, y: head.y + 1 });
            } else if self.vel == (0, -1) {
                self.parts.push(SnekPart { x: head.x, y: head.y - 1 });
            }
            return UpdateResult::EAT;
        } else if self.get_body_loc().iter().skip(1).filter(|el| (head.x, head.y) == **el).count() > 0 {
            println!("DEAD -- HIT BODY");
            self.vel = (0,  0);
            return UpdateResult::DIE;
        } else if head.x > 1000 / 20 || head.x < 0 || head.y > 1000 / 20 || head.y < 0 {
            println!("DEAD -- HIT WALL");
            self.vel = (0,  0);
            return UpdateResult::DIE;
        }

        UpdateResult::CONTINUE
    }

    fn display(&self, draw: &Draw, grid_size: usize) {
        self.get_body_loc().iter().for_each(|el| {
            draw.rect()
                .x_y(el.0 as f32 * grid_size as f32, el.1 as f32 * grid_size as f32)
                .w_h(grid_size as f32, grid_size as f32)
                .stroke(GRAY);
        })
    }
}

struct Food {
    x: isize,
    y: isize,
}

impl Food {
    fn new(grid_size: usize, width: i32, height: i32) -> Food {
        Food {
            x: (random_range(0, width / grid_size as i32)).try_into().unwrap(),
            y: (random_range(0, height / grid_size as i32)).try_into().unwrap()
            // x: 17,
            // y: 38
        }
    }

    fn show(&self, draw: &Draw, grid_size: usize) {
        draw.rect()
            .x_y(self.x as f32 * grid_size as f32, self.y as f32 * grid_size as f32)
            .w_h(grid_size as f32, grid_size as f32)
            .color(RED);
    }

    fn update(&self, body_positions: Vec<(isize, isize)>, grid_size: usize) {
        let mut x = (random_range(0, 1000 / grid_size as i32)).try_into().unwrap();
        let mut y = (random_range(0, 1000 / grid_size as i32)).try_into().unwrap();

        loop {
            if !(body_positions.iter().filter(|el| **el == (x, y)).count() > 0) {
                break;
            }

        x = (random_range(0, 1000 / grid_size as i32)).try_into().unwrap();
        y = (random_range(0, 1000 / grid_size as i32)).try_into().unwrap();
        }
    }
}

fn model(app: &App) -> Model {
    let window = app.new_window()
        .size(1000, 1000)
        .view(view)
        .event(event)
        .title("Snek")
        .build().unwrap();

    Model {
        window,
        snek: Snek::new(),
        food: Food::new(20, 1000, 1000),
        grid_size: 20
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().x_y(-500. + 10., 500. - 10.).scale_y(-1.0);
    draw.background().color(BLACK);

    model.food.show(&draw, model.grid_size);

    model.snek.display(&draw, model.grid_size);

    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::KeyPressed(key) => {
                if key == Key::Up { 
                    model.snek.vel = (0, -1) 
                } else if key == Key::Down { 
                    model.snek.vel = (0, 1) 
                } else if key == Key::Left { 
                    model.snek.vel = (-1, 0) 
                } else if key == Key::Right {
                    model.snek.vel = (1, 0) 
                } else {
                    println!("{:?}", key)
                }
        },
        _ => println!("{:?}", event)
    }

}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // println!("update");

    match model.snek.update(&model.food) {
        UpdateResult::DIE => panic!("DEAD"),
        UpdateResult::EAT => model.food.update(model.snek.get_body_loc(), model.grid_size),
        _ => {}
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::rate_fps(5.0))
        .run();
}

