extern crate rand;

use self::rand::{thread_rng, Rng};

use piston::input::*;

use components::{PositionComponent, RenderComponent};
use level::{Components, Entity, Level};

pub const MOVE_SPEED: f64 = 500.0;
const CHANGE_INTERVAL: u32 = 60;
pub const COLOR: [f32; 4] = [0.3, 0.3, 0.7, 1.0];  // Blue
pub const SIZE: f64 = 50.0;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct RandomMobComponent {
    change_cnt: u32,
    curr_dir: Dir,
}

pub fn new(level: &mut Level) -> Entity {
    let result = level.create_entity();
    level.components.positions.set(&result, PositionComponent { x: 0.0, y: 0.0 });
    level.components.randos.set(&result, RandomMobComponent { change_cnt: 0, curr_dir: Dir::Up });
    level.components.renderables.set(&result, RenderComponent {
        color: COLOR,
        size: SIZE,
    });
    result
}

pub struct RandomMobUpdateSystem;

impl RandomMobUpdateSystem {
    pub fn run(&self, args: &UpdateArgs, components: &mut Components, entities: &Vec<Entity>) {
        let filtered_entities: Vec<&Entity> = entities.iter()
            .filter(|e| {
                components.randos.get(e).is_some() &&
                    components.positions.get(e).is_some()
            }).collect();

        for entity in filtered_entities {
            let rando_comp = components.randos.get_mut(entity).unwrap();
            let pos_comp = components.positions.get_mut(entity).unwrap();

            if rando_comp.change_cnt == 0 {
                let mut rng = thread_rng();
                let rand_num = rng.gen_range(0, 4);
                rando_comp.curr_dir = match rand_num {
                    0 => Dir::Up,
                    1 => Dir::Down,
                    2 => Dir::Left,
                    3 => Dir::Right,
                    _ => panic!(),
                };
            }
            let ms_dt = MOVE_SPEED * args.dt;

            match rando_comp.curr_dir {
                Dir::Up => pos_comp.y += ms_dt,
                Dir::Down => pos_comp.y -= ms_dt,
                Dir::Left => pos_comp.x += ms_dt,
                Dir::Right => pos_comp.x -= ms_dt,
            };

            rando_comp.change_cnt = (rando_comp.change_cnt + 1) % CHANGE_INTERVAL;
        }
    }
}
