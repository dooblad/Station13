use std::any::TypeId;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use components::{PositionComponent, RenderComponent};
use event_handler::EventHandler;
use ecs::{Entity, EntityMap};

pub trait System {
    fn comp_constraints(&self) -> Vec<TypeId>;
    fn run(&self, event_handler: &EventHandler, args: &UpdateArgs, entity_map: &mut EntityMap,
           entities: &Vec<Entity>);
}

pub struct RenderSystem;

impl RenderSystem {
    pub fn comp_constraints(&self) -> Vec<TypeId> {
        type_id_vec![PositionComponent, RenderComponent]
    }

    pub fn run(&self, gl: &mut GlGraphics, c: Context, args: &RenderArgs, entity_map: &EntityMap,
               entities: &Vec<Entity>) {
        use graphics::*;

        let filtered_entities: Vec<&Entity> = entities.iter()
            .filter(|e| {
                let comp_map = entity_map.borrow(e).unwrap();
                comp_map.has::<PositionComponent>() &&
                    comp_map.has::<RenderComponent>()
            }).collect();

        for entity in filtered_entities.iter() {
            let mut comp_map = entity_map.borrow_mut(entity).unwrap();
            let pos_comp = comp_map.get::<PositionComponent>();
            let render_comp = comp_map.borrow_mut::<RenderComponent>();

            let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, render_comp.size);
            let transform = c.transform.trans(x, y)
                .trans(-pos_comp.x, -pos_comp.y);
            rectangle(render_comp.color, square, transform, gl);
        }
    }
}
