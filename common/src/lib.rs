#[macro_use]
extern crate enum_primitive;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate serde;
#[macro_use]
extern crate serde_derive;

// Load all macros before other modules, because of how macro scoping works.
#[macro_use]
pub mod macros;

pub mod alloc;
pub mod components;
pub mod ecs;
pub mod event_handler;
pub mod net;
pub mod player;
pub mod random_mob;
pub mod systems;
