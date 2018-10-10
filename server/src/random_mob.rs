extern crate rand;

use std::any::TypeId;

use self::rand::{thread_rng, Rng};

use common::components::{PositionComponent, RenderComponent};
use common::ecs::{Ecs, Entity, EntityMap};
use common::systems::System;

use super::TickConfig;

pub const MOVE_SPEED: f64 = 500.0;
const CHANGE_INTERVAL: u32 = 60;
pub const COLOR: [f32; 4] = [0.3, 0.3, 0.7, 1.0]; // Blue
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

pub fn new<T>(level: &mut Ecs<T>) -> Entity {
    let result = level.create_entity();
    let mut comp_map = level.entity_map.borrow_mut(&result).unwrap();
    comp_map.set(PositionComponent { x: 0.0, y: 0.0 });
    comp_map.set(RandomMobComponent {
        change_cnt: 0,
        curr_dir: Dir::Up,
    });
    comp_map.set(RenderComponent {
        color: COLOR,
        size: SIZE,
    });
    result
}

pub struct RandomMobUpdateSystem;

impl System<TickConfig> for RandomMobUpdateSystem {
    fn comp_constraints(&self) -> Vec<TypeId> {
        type_id_vec![RandomMobComponent, PositionComponent]
    }

    fn run(&self, tick_config: &TickConfig, entity_map: &mut EntityMap, entities: &Vec<Entity>) {
        for entity in entities {
            let mut comp_map = entity_map.borrow_mut(entity).unwrap();

            let (dx, dy) = {
                let mut rando_comp = comp_map.borrow_mut::<RandomMobComponent>();
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

                let ms_dt = MOVE_SPEED * tick_config.dt;
                let mut dx = 0.0f64;
                let mut dy = 0.0f64;
                match rando_comp.curr_dir {
                    Dir::Up => dy += ms_dt,
                    Dir::Down => dy -= ms_dt,
                    Dir::Left => dx += ms_dt,
                    Dir::Right => dx -= ms_dt,
                };
                rando_comp.change_cnt = (rando_comp.change_cnt + 1) % CHANGE_INTERVAL;

                (dx, dy)
            };

            let pos_comp = comp_map.borrow_mut::<PositionComponent>();
            pos_comp.x += dx;
            pos_comp.y += dy;
        }
    }
}
