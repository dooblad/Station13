pub const LEVEL_WIDTH: usize = 32;
pub const LEVEL_HEIGHT: usize = 32;

pub struct Level {
    tiles: [bool; LEVEL_WIDTH * LEVEL_HEIGHT],
}

impl Level {
    pub fn new() -> Self {
        Self {
            tiles: [true; LEVEL_WIDTH * LEVEL_HEIGHT],
        }
    }
}