use std::any::TypeId;

use super::{Entity, EntityMap};

pub trait System<T> {
    fn comp_constraints(&self) -> Vec<TypeId>;
    fn run(&self, tick_config: &T, entity_map: &mut EntityMap, entities: &Vec<Entity>);
}
