use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub type EntityId = u64;

// Here we have to extend `Any`, because otherwise we need to specify a static lifetime bound to
// functions generic on `Component`.
//
// TODO: Why the fuck do we need to do that?
// Is `Any` implied to have a static lifetime?
pub trait Component: Any {}
impl<T: Any> Component for T {}

pub trait System: Any {
    fn comp_constraints(&self) -> Vec<TypeId>;
    fn act(&mut self, entities: Vec<EntityId>,
           mut entity_to_components: HashMap<EntityId, ComponentMap>);
}

pub struct EcsManager {
    id_counter: EntityId,
    entity_to_components: HashMap<EntityId, ComponentMap>,
    systems: Vec<Box<System>>,
}

impl EcsManager {
    pub fn new() -> Self {
        EcsManager {
            id_counter: 0,
            entity_to_components: HashMap::new(),
            systems: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> EntityId {
        let id = self.id_counter;
        self.id_counter += 1;
        self.entity_to_components.insert(id, ComponentMap::new());
        id
    }

    pub fn component_map(&mut self, id: EntityId) -> &mut ComponentMap {
        self.entity_to_components.get_mut(&id).expect(
            &format!("no `ComponentMap` for id \"{}\"", id))
    }

    pub fn add_system<S: System>(&mut self, system: S) {
        // TODO: You could potentially have duplicate systems with this method.
        self.systems.push(Box::new(system));
    }

    pub fn tick(&mut self) {
        // TODO: Make a struct just for transporting (entity id, filtered_components).
        let entities: Vec<&EntityId> = self.entity_to_components.keys().collect();
        for system in self.systems.iter() {
            let constraints: Vec<TypeId> = system.comp_constraints();
            let mut sat_entities: Vec<EntityId> = Vec::new();
            for entity in entities.iter() {
                let cmp_map = self.entity_to_components.get_mut(entity).unwrap();
                if constraints.iter().all(|&c| cmp_map.contains_id(c)) {
                    sat_entities.push(**entity);
                }
            }
            // TODO: Call `act` on system.
        }
    }
}

#[derive(Default)]
pub struct ComponentMap {
    map: HashMap<TypeId, Box<Any>>,
}

impl ComponentMap {
    pub fn new() -> Self { Default::default() }

    pub fn set<C: Component>(&mut self, component: C) {
        self.map.insert(TypeId::of::<C>(), Box::new(component));
    }

    pub fn borrow<C: Component>(&self) -> &C {
        self.map
            .get(&TypeId::of::<C>())
            .map(|c| c.downcast_ref().unwrap())
            .unwrap()
    }

    fn borrow_mut<C: Component>(&mut self) -> &mut C {
        self.map
            .get_mut(&TypeId::of::<C>())
            .map(|c| { c.downcast_mut().unwrap() })
            .unwrap()
    }

    pub fn get<C: Component + Clone>(&self) -> C {
        self.borrow::<C>().clone()
    }

    pub fn contains<C: Component>(&self) -> bool {
        self.contains_id(TypeId::of::<C>())
    }

    pub fn contains_id(&self, id: TypeId) -> bool {
        self.map.contains_key(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Nothing {}
    struct IntWrapper(i32);
    #[derive(Clone, Debug, PartialEq)]
    struct Position(i32, i32);
    #[derive(Clone, Debug)]
    struct Velocity(i32, i32);

    #[derive(Default)]
    struct UpdatePositionSystem {}

    impl UpdatePositionSystem {
        pub fn new() -> Self { Default::default() }
    }

    impl System for UpdatePositionSystem {
        fn comp_constraints(&self) -> Vec<TypeId> {
            vec![TypeId::of::<Position>(), TypeId::of::<Velocity>()]
        }

        fn act(&mut self, entities: Vec<EntityId>,
               mut entity_to_components: HashMap<EntityId, ComponentMap>) {
            for entity in entities {
                let cm = entity_to_components.get_mut(&entity).unwrap();
                let old_pos = cm.get::<Position>();
                let vel = cm.get::<Velocity>();
                cm.set(Position(old_pos.0 + vel.0, old_pos.1 + vel.1));
            }
        }
    }


    #[test]
    fn entities_not_equal() {
        let mut ecs = EcsManager::new();
        let entity_one = ecs.new_entity();
        let entity_two = ecs.new_entity();
        assert_ne!(entity_one, entity_two);
    }

    #[test]
    fn attach_component() {
        let mut ecs = EcsManager::new();
        let entity_one = ecs.new_entity();
        let entity_one_cm = ecs.component_map(entity_one);
        entity_one_cm.set(Nothing {});
    }

    #[test]
    fn overwrite_component() {
        let mut ecs = EcsManager::new();
        let entity_one = ecs.new_entity();
        let entity_one_cm = ecs.component_map(entity_one);
        entity_one_cm.set(IntWrapper(0));
        entity_one_cm.set(IntWrapper(1));
        assert_eq!(entity_one_cm.borrow::<IntWrapper>().0, 1)
    }

    #[test]
    fn pos_system_updates() {
        let mut ecs = EcsManager::new();
        let entity_one = ecs.new_entity();
        let entity_one_cm = ecs.component_map(entity_one);
        entity_one_cm.set(Position(1, 2));
        entity_one_cm.set(Velocity(3, 4));
        assert_eq!(*entity_one_cm.borrow::<Position>(), Position(4, 6))
    }
}
