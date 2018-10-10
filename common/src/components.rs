#[derive(Clone)]
pub struct PositionComponent {
    pub x: f64,
    pub y: f64,
}

pub struct RenderComponent {
    pub color: [f32; 4],
    pub size: f64,
}
