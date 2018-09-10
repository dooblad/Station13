use opengl_graphics::GlGraphics;
use piston::input::{Key, RenderArgs, UpdateArgs};

use event_handler::EventHandler;
use player::{ControlScheme, Intent, Player};
use random_mob::RandomMob;

pub const LEVEL_WIDTH: usize = 32;
pub const LEVEL_HEIGHT: usize = 32;

pub struct Level {
    players: Vec<Player>,
    randos: Vec<RandomMob>,
}

impl Level {
    pub fn new() -> Self {
        let mut players: Vec<Player> = Vec::new();
        // Player One Setup
        let mut cs_one = ControlScheme::new();
        cs_one.0.insert(Intent::Up, Key::W);
        cs_one.0.insert(Intent::Down, Key::S);
        cs_one.0.insert(Intent::Left, Key::A);
        cs_one.0.insert(Intent::Right, Key::D);
        players.push(Player::new(cs_one));
        // Player Two Setup
        let mut cs_two = ControlScheme::new();
        cs_two.0.insert(Intent::Up, Key::Up);
        cs_two.0.insert(Intent::Down, Key::Down);
        cs_two.0.insert(Intent::Left, Key::Left);
        cs_two.0.insert(Intent::Right, Key::Right);
        players.push(Player::new(cs_two));

        let mut randos: Vec<RandomMob> = Vec::new();
        for _ in 0..8 {
            randos.push(RandomMob::new());
        }

        Self {
            players,
            randos,
        }
    }

    pub fn tick(&mut self, args: &UpdateArgs, event_handler: &EventHandler) {
        for player in self.players.iter_mut() {
            player.tick(args, event_handler);
        }
        for rando in self.randos.iter_mut() {
            rando.tick(args);
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.3, 0.7, 0.3, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            for player in self.players.iter_mut() {
                player.render(gl, c, args);
            }
            for rando in self.randos.iter_mut() {
                rando.render(gl, c, args);
            }
        });

    }
}