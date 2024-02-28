use nannou::prelude::*;

struct Model {
    window_id: WindowId,
    grid_size: usize,
    LL: LinkedList
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window(model.window_id).unwrap();
    let win = window.rect();

    draw.background().color(BLACK);

    model.LL.get_pos_vec().iter().for_each(|el| {
        draw.rect()
            .color(WHITE)
            .x_y(el.x * model.grid_size as f32, el.y * model.grid_size as f32)
            .w_h(model.grid_size as f32, model.grid_size as f32);

        println!("{}, {}", el.x, el.y);
    });

    draw.to_frame(app, &frame).unwrap();

}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {
    println!("{:?}", _event);
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.LL.update();
}

fn model(app: &App) -> Model {
    let window_id = app.new_window()
        .size(1000, 1000)
        .view(view)
        .event(event)
        .title("boop bop")
        .build().unwrap();

    let LL = LinkedList::new();

    Model {
        window_id,
        grid_size: 20,
        LL,
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct LinkedList {
    head: Box<Node>,
    tail: Box<Node>,
    len: usize,
}

impl LinkedList {
    fn new() -> Self {
        let node = Box::new(Node::new(pt2(0., 0.), pt2(1., 0.)));
        LinkedList {
            head: node.clone(),
            tail: node,
            len: 1
        }
    }

    fn get_pos_vec(&self) -> Vec<Vec2> {
        let mut curr = self.head.clone();
        let mut outp1 = Vec::new();
        for _ in 0..self.len {
            outp1.push(curr.pos);
            match curr.clone().next {
                Some(_) => curr = curr.next.unwrap().clone(),
                None => {}
            }
        }

        outp1
    }

    fn update(&mut self) {
        let mut is_head = true;
        let mut curr = self.head.clone();

        for _ in 0..self.len {
            if is_head {
                let food = Food {pos: pt2(-300., -300.)};
                let body = self.get_pos_vec();

                curr.move_from_vel();
                curr.check_pos(food, body);
                is_head = false;
                match curr.clone().next {
                    Some(_) => curr = curr.next.unwrap().clone(),
                    None => {}
                }
            } else {
                match curr.clone().next {
                    Some(_) => {
                        let prev = curr.clone();
                        curr = curr.next.unwrap().clone();
                        curr.move_to(prev.pos);
                    }
                    None => {

                    }
                }
            }
        }

        println!("Updating Complete");
    }

    fn change_velocity(&mut self, new: Vec2) {
        assert!(new == pt2(-1., 0.) || new == pt2(0., -1.) || new == pt2(1., 0.) || new == pt2(0., 1.));
        self.head.vel = Some(new)
    }

}

#[derive(Clone)]
struct Node {
    pos: Vec2,
    vel: Option<Vec2>,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(pos: Vec2, vel: Vec2) -> Self {
        Node {
            pos,
            vel: Some(vel),
            next: None,
        }
    }

    fn move_from_vel(&mut self) {
        self.pos += self.vel.unwrap().normalize();
        println!("moved");
    }

    fn move_to(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    fn check_pos(&self, food: Food, body: Vec<Vec2>) {
        if self.pos == food.pos {
            println!("GROW");
        } else if self.pos.x > 950. || self.pos.x < 0. || self.pos.y > 950. || self.pos.y < 0. {
            println!("SCREEN ISSUE, END GAME");
        } else {
            if body.iter().filter(|&&el| el == self.pos).collect::<Vec<_>>().len() != 0 {
                println!("HIT YOURSELF, END GAME");
            }
        }
    }
}

struct Food {
    pos: Vec2
}
