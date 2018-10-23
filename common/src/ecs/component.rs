extern crate serde;

use crate::random_mob::RandomMobComponent;

#[derive(Debug, Serde)]
pub enum Component {
    PositionComponent(PositionComponent),
    RenderComponent(RenderComponent),
    RandomMobComponent(RandomMobComponent),
}

#[derive(Debug, Serde)]
pub struct PositionComponent {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serde)]
pub struct RenderComponent {
    pub color: [f32; 4],
    pub size: f64,
}
