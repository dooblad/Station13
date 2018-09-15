use std::collections::HashMap;

use piston::input::*;

use components::{PositionComponent, RenderComponent};
use level::{Components, Entity, Level};
use event_handler::EventHandler;

pub const MOVE_SPEED: f64 = 500.0;
pub const COLOR: [f32; 4] = [0.7, 0.3, 0.3, 1.0];  // Red
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

pub fn new(control_scheme: ControlScheme, level: &mut Level) -> Entity {
    let result = level.create_entity();
    level.components.positions.set(&result, PositionComponent { x: 0.0, y: 0.0 });
    level.components.players.set(&result, PlayerComponent { control_scheme });
    level.components.renderables.set(&result, RenderComponent {
        color: COLOR,
        size: SIZE,
    });
    result
}

pub struct PlayerUpdateSystem;

impl PlayerUpdateSystem {
    pub fn run(&self, event_handler: &EventHandler, args: &UpdateArgs,
               components: &mut Components, entities: &Vec<Entity>) {
        use self::Intent::*;

        let filtered_entities: Vec<&Entity> = entities.iter()
            .filter(|e| {
                components.players.get(e).is_some() &&
                    components.positions.get(e).is_some()
            }).collect();

        for entity in filtered_entities.iter() {
            let player_comp = components.players.get(entity).unwrap();
            let pos_comp = components.positions.get_mut(entity).unwrap();
            let ms_dt = MOVE_SPEED * args.dt;
            if player_comp.control_scheme.intends(Up, event_handler) {
                pos_comp.y += ms_dt;
            }
            if player_comp.control_scheme.intends(Down, event_handler) {
                pos_comp.y -= ms_dt;
            }
            if player_comp.control_scheme.intends(Left, event_handler) {
                pos_comp.x += ms_dt;
            }
            if player_comp.control_scheme.intends(Right, event_handler) {
                pos_comp.x -= ms_dt;
            }
        }
    }
}
