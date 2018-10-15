extern crate uniq_id;

// TODO: Use custom inner attributes to apply a uniq group to all structs in a module.

#[derive(Clone, UniqId)]
#[UniqGroup = "comp"]
pub struct PositionComponent {
    pub x: f64,
    pub y: f64,
}

#[derive(UniqId)]
#[UniqGroup = "comp"]
pub struct RenderComponent {
    pub color: [f32; 4],
    pub size: f64,
}
