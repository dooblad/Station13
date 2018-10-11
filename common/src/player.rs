use std::any::TypeId;
use std::collections::HashMap;

use piston::input::*;

use super::components::{PositionComponent, RenderComponent};
use super::ecs::{Ecs, Entity, EntityMap};
use super::event_handler::EventHandler;
use super::systems::System;

pub const MOVE_SPEED: f64 = 500.0;
pub const COLOR: [f32; 4] = [0.7, 0.3, 0.3, 1.0]; // Red
pub const SIZE: f64 = 50.0;

#[derive(PartialEq, Eq, Hash)]
pub enum Intent {
    Up,
    Down,
    Left,
    Right,
}

pub struct ControlScheme(pub HashMap<Intent, Key>);

impl ControlScheme {
    pub fn new() -> Self {
        ControlScheme(HashMap::new())
    }

    pub fn intends(&self, intent: Intent, event_handler: &EventHandler) -> bool {
        if let Some(&k) = self.0.get(&intent) {
            event_handler.is_key_down(k)
        } else {
            false
        }
    }
}

pub struct PlayerComponent {
    control_scheme: ControlScheme,
}

pub fn new<T>(control_scheme: ControlScheme, level: &mut Ecs<T>) -> Entity {
    let result = level.create_entity();
    let mut comp_map = level.entity_map.borrow_mut(&result).unwrap();
    comp_map.set(PositionComponent { x: 0.0, y: 0.0 });
    comp_map.set(PlayerComponent { control_scheme });
    comp_map.set(RenderComponent {
        color: COLOR,
        size: SIZE,
    });
    result
}

/*
pub struct PlayerUpdateSystem;

impl System for PlayerUpdateSystem {
    fn comp_constraints(&self) -> Vec<TypeId> {
        type_id_vec![PlayerComponent, PositionComponent]
    }

    fn run(&self, event_handler: &EventHandler, args: &UpdateArgs, entity_map: &mut EntityMap,
           entities: &Vec<Entity>) {
        use self::Intent::*;

        for entity in entities {
            let mut comp_map = entity_map.borrow_mut(entity).unwrap();
            let (dx, dy) = {
                let player_comp = comp_map.borrow::<PlayerComponent>();
                let ms_dt = MOVE_SPEED * args.dt;
                let mut dx = 0.0f64;
                let mut dy = 0.0f64;
                if player_comp.control_scheme.intends(Up, event_handler) {
                    dy += ms_dt;
                }
                if player_comp.control_scheme.intends(Down, event_handler) {
                    dy -= ms_dt;
                }
                if player_comp.control_scheme.intends(Left, event_handler) {
                    dx += ms_dt;
                }
                if player_comp.control_scheme.intends(Right, event_handler) {
                    dx -= ms_dt;
                }
                (dx, dy)
            };
            let pos_comp = comp_map.borrow_mut::<PositionComponent>();
            pos_comp.x += dx;
            pos_comp.y += dy;
        }
    }
}
*/
