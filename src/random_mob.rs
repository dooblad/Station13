extern crate rand;

use self::rand::{thread_rng, Rng};

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::*;

pub const MOVE_SPEED: f64 = 500.0;
const CHANGE_INTERVAL: u32 = 60;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct RandomMob {
    pos: (f64, f64),
    change_cnt: u32,
    curr_dir: Dir,
}

impl RandomMob {
    pub fn new() -> Self {
        Self {
            pos: (0.0, 0.0),
            change_cnt: 0,
            curr_dir: Dir::Up,
        }
    }

    pub fn tick(&mut self, args: &UpdateArgs) {
        if self.change_cnt == 0 {
            let mut rng = thread_rng();
            let rand_num = rng.gen_range(0, 4);
            self.curr_dir = match rand_num {
                0 => Dir::Up,
                1 => Dir::Down,
                2 => Dir::Left,
                3 => Dir::Right,
                _ => panic!(),
            };
        }
        let ms_dt = MOVE_SPEED * args.dt;

        match self.curr_dir {
            Dir::Up => self.pos.1 += ms_dt,
            Dir::Down => self.pos.1 -= ms_dt,
            Dir::Left => self.pos.0 += ms_dt,
            Dir::Right => self.pos.0 -= ms_dt,
        };

        self.change_cnt = (self.change_cnt + 1) % CHANGE_INTERVAL;
    }

    pub fn render(&mut self, gl: &mut GlGraphics, c: Context, args: &RenderArgs) {
        use graphics::*;
        const BLUE: [f32; 4] = [0.3, 0.3, 0.7, 1.0];
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let square = rectangle::square(0.0, 0.0, 50.0);
        let transform = c.transform.trans(x, y)
            .trans(-self.pos.0, -self.pos.1);
        rectangle(BLUE, square, transform, gl);
    }
}
