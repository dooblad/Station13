use std::any::TypeId;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use components::{PositionComponent, RenderComponent};
use ecs::{Entity, EntityMap};
use event_handler::EventHandler;

pub trait System<T> {
    fn comp_constraints(&self) -> Vec<TypeId>;
    fn run(&self, tick_config: &T, entity_map: &mut EntityMap, entities: &Vec<Entity>);
}
