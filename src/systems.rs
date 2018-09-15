use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use level::{Components, Entity};

pub struct RenderSystem;

impl RenderSystem {
    pub fn run(&self, gl: &mut GlGraphics, c: Context, args: &RenderArgs, components: &Components, entities: &Vec<Entity>) {
        use graphics::*;

        let filtered_entities: Vec<&Entity> = entities.iter()
            .filter(|e| {
                components.positions.get(e).is_some() &&
                    components.renderables.get(e).is_some()
            }).collect();

        for entity in filtered_entities.iter() {
            let pos_comp = components.positions.get(entity).unwrap();
            let render_comp = components.renderables.get(entity).unwrap();

//            const RED: [f32; 4] = [0.7, 0.3, 0.3, 1.0];
            let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, render_comp.size);
            let transform = c.transform.trans(x, y)
                .trans(-pos_comp.x, -pos_comp.y);
            rectangle(render_comp.color, square, transform, gl);
        }
    }
}
