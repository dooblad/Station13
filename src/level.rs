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

pub struct Components {
    pub positions: EntityMap<PositionComponent>,
    pub players: EntityMap<PlayerComponent>,
    pub randos: EntityMap<RandomMobComponent>,
    pub renderables: EntityMap<RenderComponent>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            positions: GenerationalIndexArray(Vec::new()),
            players: GenerationalIndexArray(Vec::new()),
            randos: GenerationalIndexArray(Vec::new()),
            renderables: GenerationalIndexArray(Vec::new()),
        }
    }
}

pub struct Level {
    // Entities
    entities: Vec<Entity>,
    // Components
    pub components: Components,
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
            components: Components::new(),
            player_update_system: PlayerUpdateSystem {},
            rando_update_system: RandomMobUpdateSystem {},
            render_system: RenderSystem {},
            entity_allocator: GenerationalIndexAllocator::new(),
        };

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