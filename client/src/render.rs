use std::any::TypeId;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use common::components::{PositionComponent, RenderComponent};
use common::ecs::{Ecs, Entity, EntityMap};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render<T>(&self, ecs: &Ecs<T>, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.3, 0.7, 0.3, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            {
                let comp_constraints = self.comp_constraints();
                let filtered_entities = ecs.entities().filter(|e| {
                    let comp_map = ecs.entity_map.borrow(e).unwrap();
                    for comp_type_id in comp_constraints.iter() {
                        if !comp_map.has_type_id(comp_type_id) {
                            return false;
                        }
                    }
                    true
                });

                for entity in filtered_entities {
                    self.render_single(gl, c, args, &ecs.entity_map, &entity)
                }
            }
        });
    }

    fn render_single(
        &self,
        gl: &mut GlGraphics,
        c: Context,
        args: &RenderArgs,
        entity_map: &EntityMap,
        entity: &Entity,
    ) {
        use graphics::*;

        let comp_map = entity_map.borrow(entity).unwrap();
        let pos_comp = comp_map.get::<PositionComponent>();
        let render_comp = comp_map.borrow::<RenderComponent>();

        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let square = rectangle::square(0.0, 0.0, render_comp.size);
        let transform = c.transform.trans(x, y).trans(-pos_comp.x, -pos_comp.y);
        rectangle(render_comp.color, square, transform, gl);
    }

    fn comp_constraints(&self) -> Vec<TypeId> {
        type_id_vec![PositionComponent, RenderComponent]
    }
}
