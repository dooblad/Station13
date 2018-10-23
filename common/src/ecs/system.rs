use std::any::TypeId;

use crate::event_handler::EventHandler;
use super::{Entity, EntityMap};
use super::component::{PositionComponent, RenderComponent};

pub trait System<T> {
    fn comp_constraints(&self) -> Vec<TypeId>;
    fn run(&self, tick_config: &T, entity_map: &mut EntityMap, entities: &Vec<Entity>);
}
