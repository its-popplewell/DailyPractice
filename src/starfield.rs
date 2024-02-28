use nannou::prelude::*;
use rand::prelude::*;

fn rand_between(mini: f32, maxi: f32, rng_num: f32) -> f32 {
    assert!(mini < maxi); // Ensure the minimum is less than the maximum
    assert!(rng_num >= 0. && rng_num <= 1.);

    (rng_num * (maxi - mini)) + mini
}

#[derive(Clone, Debug)]
struct Star {
    pos: Vec2,
    depth: f32,
    previous_depth: f32,
    delta_depth: f32,
    move_vec: Vec2
}

impl Star {
    fn new(pos: Vec2, depth: f32) -> Self {
        Star {
            pos,
            depth,
            previous_depth: depth,
            delta_depth: 0.1,
            move_vec: pos.normalize()
        }
    }

    fn reset(&mut self, pos: Vec2, depth: f32) {
        self.pos = pos;
        self.depth = depth;
        self.previous_depth = depth;
        self.move_vec = pos.normalize();
    }
    
    // true if in bounds
    // false if out of bounds
    fn update(&mut self) -> bool {
        self.previous_depth = self.depth;
        self.depth += self.delta_depth;

        let interp_depth = std::f32::consts::E.pow(self.depth);
        let draw_pos = self.pos + (self.move_vec * interp_depth);
        
        if draw_pos.x > 500. || draw_pos.x < -500. || draw_pos.y > 500. || draw_pos.y < -500. {
            return false;
        }

        true
    }

    fn show(&self) -> (Vec2, Vec2) {

        
        let interp_depth = std::f32::consts::E.pow(self.depth);
        let draw_pos = self.pos + (self.move_vec * interp_depth);

        let prev_interp_depth = std::f32::consts::E.pow(self.previous_depth);
        let prev_draw_pos = self.pos + (self.move_vec * prev_interp_depth);

        (draw_pos, prev_draw_pos)
    }
}

struct Model {
    window_id: WindowId,
    stars: Vec<Star>,
    rng: ThreadRng
}

fn model(app: &App) -> Model {
    println!("BUILDING MODEL");

    let window_id = app
        .new_window()
        .size(1000, 1000)
        .title("sucadic")
        .view(view)
        .event(event)
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();

    let mut stars = Vec::new();
    let count = 400;

    // println!("{:?}", pt2(rand_between(-400., 400., rng.gen::<f32>()), rand_between(-400., 400., rng.gen::<f32>())));
    // stars.push(Star::new(
    //     pt2(rand_between(-40., 40., rng.gen::<f32>()), rand_between(-40., 40., rng.gen::<f32>())),
    //     rand_between(0., 1., rng.gen::<f32>()) 
    // ));
        
    for _ in 0..count {
        println!("GENERATING STAR");
        let mut pos = pt2(rand_between(-40., 40., rng.gen::<f32>()), rand_between(-40., 40., rng.gen::<f32>()));
        while pos.x == 0. && pos.y == 0. {
            pos = pt2(rand_between(-40., 40., rng.gen::<f32>()), rand_between(-40., 40., rng.gen::<f32>()));
        }


        stars.push(Star::new(
                pos,
                rand_between(0., 4., rng.gen::<f32>()) 
        ));
    }

    println!("{:?}", stars.clone());

    Model {
        window_id,
        stars,
        rng
    }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    // println!("{:?}", event);
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for star in model.stars.iter_mut() {

        // Only runs when start is out of bounds
        if !star.update() {
            let mut pos = pt2(rand_between(-60., 60., model.rng.gen::<f32>()), rand_between(-60., 60., model.rng.gen::<f32>()));
            while pos.x == 0. && pos.y == 0. {
                pos = pt2(rand_between(-40., 40., model.rng.gen::<f32>()), rand_between(-40., 40., model.rng.gen::<f32>()));
            }
            
            star.reset(
                pos,
                rand_between(0., 4., model.rng.gen::<f32>()) 
            );
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window(model.window_id).expect("COULDN'T LOAD WINDOW FROM ID");
    let win = window.rect();

    draw.background().color(BLACK);

    for star in model.stars.clone() {
        let(startp, endp) = star.show();
        
        // draw.ellipse()
        //     .xy(startp)
        //     .radius(5.0)
        //     .color(BLACK);

        draw.line()
            .start(startp)
            .end(endp)
            .weight(1.0)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).expect("ISSUE DRAWING TO FRAME");
}
