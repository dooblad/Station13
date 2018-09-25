use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use components::{PositionComponent, RenderComponent};
use level::{ComponentMap, Entity};

pub struct RenderSystem;

impl RenderSystem {
    pub fn run(&self, gl: &mut GlGraphics, c: Context, args: &RenderArgs, components: &ComponentMap, entities: &Vec<Entity>) {
        use graphics::*;

        let filtered_entities: Vec<&Entity> = entities.iter()
            .filter(|e| {
                components.borrow::<PositionComponent>(e).is_some() &&
                    components.borrow::<RenderComponent>(e).is_some()
            }).collect();

        for entity in filtered_entities.iter() {
            let pos_comp = components.borrow::<PositionComponent>(entity).unwrap();
            let render_comp = components.borrow::<RenderComponent>(entity).unwrap();

            let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, render_comp.size);
            let transform = c.transform.trans(x, y)
                .trans(-pos_comp.x, -pos_comp.y);
            rectangle(render_comp.color, square, transform, gl);
        }
    }
}
