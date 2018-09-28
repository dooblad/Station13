use std::any::{Any, TypeId};
use std::cell::{Ref, RefMut};
use std::collections::HashMap;

use opengl_graphics::GlGraphics;
use piston::input::{Key, RenderArgs, UpdateArgs};

use alloc::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexArray};
use components::{PositionComponent, RenderComponent};
use event_handler::EventHandler;
use player;
use player::{ControlScheme, Intent, PlayerComponent, PlayerUpdateSystem};
use random_mob;
use random_mob::{RandomMobComponent, RandomMobUpdateSystem};
use systems::RenderSystem;

pub const LEVEL_WIDTH: usize = 32;
pub const LEVEL_HEIGHT: usize = 32;

pub type Entity = GenerationalIndex;
pub type EntityMap<T> = GenerationalIndexArray<T>;

// Here we have to extend `Any`, because otherwise we need to specify a static lifetime bound to
// functions generic on `Component`.
//
// TODO: Why the fuck do we need to do that?
// Is `Any` implied to have a static lifetime?
pub trait Component: Any {}
impl<T: Any> Component for T {}

// TODO: Replace all `GenerationalIndexArray` occurrences with `EntityMap`.
pub struct ComponentMap {
    // TODO: Does it need to be `Any`, or could we do something along the lines of
    // `Box<EntityMap<Any>>`?
    data: HashMap<TypeId, Box<Any>>,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn borrow<C: Component>(&self, entity: &Entity) -> Ref<C> {
        self.entity_map::<C>().borrow(entity).unwrap()
    }

    pub fn borrow_mut<C: Component>(&self, entity: &Entity) -> RefMut<C> {
        self.entity_map::<C>().borrow_mut(entity).unwrap()
    }

    pub fn set<C: Component>(&mut self, entity: &Entity, comp: C) {
        self.entity_map_mut::<C>().set(entity, comp);
    }

    pub fn has_comp<C: Component>(&self, entity: &Entity) -> bool {
        self.entity_map::<C>().has_entry(entity)
    }

    fn entity_map<C: Component>(&self) -> &GenerationalIndexArray<C> {
        self.data.get(&TypeId::of::<C>())
            .map(|gia| gia.downcast_ref().unwrap())
            .unwrap()
    }

    fn entity_map_mut<C: Component>(&mut self) -> &mut GenerationalIndexArray<C> {
        self.data
            .get_mut(&TypeId::of::<C>())
            .map(|gia| gia.downcast_mut().unwrap())
            .unwrap()
    }

    pub fn register<C: Component>(&mut self) {
        let type_id = TypeId::of::<C>();
        if self.data.contains_key(&type_id) {
            // TODO: Is there a macro to grab the name of the struct we're impling?
            panic!("ComponentMap already contains {:?}", type_id);
        }
        self.data.insert(type_id, Box::new(GenerationalIndexArray::<C>::new()));
    }
}

pub struct Level {
    // Entities
    entities: Vec<Entity>,
    // Components
    pub components: ComponentMap,
    // Systems
    player_update_system: PlayerUpdateSystem,
    rando_update_system: RandomMobUpdateSystem,
    render_system: RenderSystem,
    // Entity Allocator
    entity_allocator: GenerationalIndexAllocator,
}

impl Level {
    pub fn new() -> Self {
        let mut result = Self {
            entities: Vec::new(),
            //components: Components::new(),
            components: ComponentMap::new(),
            player_update_system: PlayerUpdateSystem {},
            rando_update_system: RandomMobUpdateSystem {},
            render_system: RenderSystem {},
            entity_allocator: GenerationalIndexAllocator::new(),
        };

        // Register components
        result.components.register::<PlayerComponent>();
        result.components.register::<PositionComponent>();
        result.components.register::<RandomMobComponent>();
        result.components.register::<RenderComponent>();

        // Player One Setup
        let mut cs_one = ControlScheme::new();
        cs_one.0.insert(Intent::Up, Key::W);
        cs_one.0.insert(Intent::Down, Key::S);
        cs_one.0.insert(Intent::Left, Key::A);
        cs_one.0.insert(Intent::Right, Key::D);
        let player_one = player::new(cs_one, &mut result);
        result.entities.push(player_one);

        // Player Two Setup
        let mut cs_two = ControlScheme::new();
        cs_two.0.insert(Intent::Up, Key::Up);
        cs_two.0.insert(Intent::Down, Key::Down);
        cs_two.0.insert(Intent::Left, Key::Left);
        cs_two.0.insert(Intent::Right, Key::Right);
        let player_two = player::new(cs_two, &mut result);
        result.entities.push(player_two);

        // Add randos.
        for _ in 0..8 {
            let rando = random_mob::new(&mut result);
            result.entities.push(rando);
        }

        result
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_allocator.allocate()
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> bool {
        self.entity_allocator.deallocate(&entity)
    }

    pub fn tick(&mut self, args: &UpdateArgs, event_handler: &EventHandler) {
        /*
        {
            let comp_filter = self.player_update_system.comp_filter(&self.components);
            let filtered_entities: Vec<&Entity> = self.entities.iter().filter(comp_filter).collect();
        }
        */
        self.player_update_system.run(event_handler, args, &mut self.components, &self.entities);
        self.rando_update_system.run(args, &mut self.components, &self.entities);
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.3, 0.7, 0.3, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            self.render_system.run(gl, c, args, &self.components, &self.entities);
        });

    }
}