use std::any::{Any, TypeId};
use std::collections::HashMap;

use opengl_graphics::GlGraphics;
use piston::input::{Key, RenderArgs, UpdateArgs};

use alloc::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexArray};
use event_handler::EventHandler;
use player;
use player::{ControlScheme, Intent, PlayerUpdateSystem};
use random_mob;
use random_mob::RandomMobUpdateSystem;
use systems::{RenderSystem, System};

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
        Self { data: HashMap::new() }
    }

    pub fn get<C: Component + Clone>(&self) -> C {
        self.borrow::<C>().clone()
    }

    pub fn borrow<C: Component>(&self) -> &C {
        self.data.get(&TypeId::of::<C>())
            .map(|c| c.downcast_ref().unwrap())
            .unwrap()
    }

    pub fn borrow_mut<C: Component>(&mut self) -> &mut C {
        self.data.get_mut(&TypeId::of::<C>())
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

pub struct Ecs {
    pub entity_map: GenerationalIndexArray<ComponentMap>,
    // The order of the systems in the vec defines the order in which the systems will be run.
    logic_systems: Vec<Box<System>>,
    render_system: RenderSystem,
    entity_allocator: GenerationalIndexAllocator,
    players: Vec<Entity>,
}

impl Ecs {
    pub fn new() -> Self {
        let mut result = Self {
            entity_map: GenerationalIndexArray::new(),
            logic_systems: sys_vec![
                PlayerUpdateSystem,
                RandomMobUpdateSystem,
            ],
            render_system: RenderSystem,
            entity_allocator: GenerationalIndexAllocator::new(),
            players: Vec::new(),
        };

        // Player One Setup
        let mut cs_one = ControlScheme::new();
        cs_one.0.insert(Intent::Up, Key::W);
        cs_one.0.insert(Intent::Down, Key::S);
        cs_one.0.insert(Intent::Left, Key::A);
        cs_one.0.insert(Intent::Right, Key::D);
        let player_one = player::new(cs_one, &mut result);
        result.players.push(player_one);

        // Player Two Setup
        let mut cs_two = ControlScheme::new();
        cs_two.0.insert(Intent::Up, Key::Up);
        cs_two.0.insert(Intent::Down, Key::Down);
        cs_two.0.insert(Intent::Left, Key::Left);
        cs_two.0.insert(Intent::Right, Key::Right);
        let player_two = player::new(cs_two, &mut result);
        result.players.push(player_two);

        // Add randos.
        for _ in 0..8 {
            random_mob::new(&mut result);
        }

        result
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

    pub fn tick(&mut self, args: &UpdateArgs, event_handler: &EventHandler) {
        for system in self.logic_systems.iter() {
            // Find which components we need to filter on.
            let comp_constraints = system.comp_constraints();
            let filtered_entities: Vec<Entity> = self.entity_allocator.iter()
                .filter(|e| {
                    let comp_map = self.entity_map.borrow(e).unwrap();
                    for comp_type_id in comp_constraints.iter() {
                        if !comp_map.has_type_id(comp_type_id) {
                            return false;
                        }
                    }
                    true
                }).collect();
            system.run(event_handler, args, &mut self.entity_map, &filtered_entities);
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.3, 0.7, 0.3, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            {
                let comp_constraints = self.render_system.comp_constraints();
                let filtered_entities: Vec<Entity> = self.entity_allocator.iter()
                    .filter(|e| {
                        let comp_map = self.entity_map.borrow(e).unwrap();
                        for comp_type_id in comp_constraints.iter() {
                            if !comp_map.has_type_id(comp_type_id) {
                                return false;
                            }
                        }
                        true
                    }).collect();
                self.render_system.run(gl, c, args, &mut self.entity_map, &filtered_entities);
            }
        });

    }
}