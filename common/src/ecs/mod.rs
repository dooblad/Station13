pub mod alloc;
pub mod component;
pub mod system;

use std::any::{Any, TypeId};
use std::collections::HashMap;

use self::alloc::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexArray};
use self::system::System;

pub const LEVEL_WIDTH: usize = 32;
pub const LEVEL_HEIGHT: usize = 32;

pub type Entity = GenerationalIndex;
pub type EntityMap = GenerationalIndexArray<ComponentMap>;

// Here we have to extend `Any`, because otherwise we need to specify a static lifetime bound to
// functions generic on `Component`.
//
// TODO: Why the fuck do we need to do that?
// Is `Any` implied to have a static lifetime?
pub trait Component: Any {}
impl<T: Any> Component for T {}

/// Maps from component type IDs to the corresponding component for a single entity.
pub struct ComponentMap {
    data: HashMap<TypeId, Box<Any>>,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get<C: Component + Clone>(&self) -> C {
        self.borrow::<C>().clone()
    }

    pub fn borrow<C: Component>(&self) -> &C {
        self.data
            .get(&TypeId::of::<C>())
            .map(|c| c.downcast_ref().unwrap())
            .unwrap()
    }

    pub fn borrow_mut<C: Component>(&mut self) -> &mut C {
        self.data
            .get_mut(&TypeId::of::<C>())
            .map(|c| c.downcast_mut().unwrap())
            .unwrap()
    }

    pub fn set<C: Component>(&mut self, comp: C) {
        self.data.insert(TypeId::of::<C>(), Box::new(comp));
    }

    pub fn remove<C: Component>(&mut self) {
        self.data.remove(&TypeId::of::<C>());
    }

    pub fn has<C: Component>(&self) -> bool {
        self.has_type_id(&TypeId::of::<C>())
    }

    pub fn has_type_id(&self, type_id: &TypeId) -> bool {
        self.data.contains_key(type_id)
    }
}

pub struct Ecs<T> {
    pub entity_map: GenerationalIndexArray<ComponentMap>,
    // The order of the systems in the vec defines the order in which the systems will be run.
    systems: Vec<Box<dyn System<T>>>,
    entity_allocator: GenerationalIndexAllocator,
    players: Vec<Entity>,
}

impl<T> Ecs<T> {
    pub fn new() -> Self {
        Self {
            entity_map: GenerationalIndexArray::new(),
            systems: Vec::new(),
            entity_allocator: GenerationalIndexAllocator::new(),
            players: Vec::new(),
        }
    }

    pub fn tick(&mut self, tick_config: &T) {
        for system in self.systems.iter() {
            // Find which components we need to filter on.
            let comp_constraints = system.comp_constraints();
            let filtered_entities: Vec<Entity> = self
                .entity_allocator
                .entries()
                .filter(|e| {
                    let comp_map = self.entity_map.borrow(e).unwrap();
                    for comp_type_id in comp_constraints.iter() {
                        if !comp_map.has_type_id(comp_type_id) {
                            return false;
                        }
                    }
                    true
                })
                .collect();
            system.run(tick_config, &mut self.entity_map, &filtered_entities);
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let result = self.entity_allocator.allocate();
        // Initialize the entity's component map.
        self.entity_map.set(&result, ComponentMap::new());
        result
    }

    /// Returns true if `entity` was successfully destroyed.  Returns false if `entity` was already
    /// destroyed.
    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        let map_rm_success = self.entity_map.remove(&entity);
        let alloc_rm_success = self.entity_allocator.deallocate(&entity);
        // If the entity's been removed from one of these but not the other, we have problems.
        assert_eq!(map_rm_success, alloc_rm_success);
        map_rm_success
    }

    pub fn systems(&mut self) -> &mut Vec<Box<dyn System<T>>> {
        &mut self.systems
    }

    pub fn entities<'a>(&'a self) -> impl Iterator<Item = Entity> + 'a {
        self.entity_allocator.entries()
    }
}
