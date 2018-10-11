extern crate uniq_comp_id;

use uniq_comp_id::UniqCompId;

#[derive(Clone, UniqCompId)]
pub struct PositionComponent {
    pub x: f64,
    pub y: f64,
}

#[derive(UniqCompId)]
pub struct RenderComponent {
    pub color: [f32; 4],
    pub size: f64,
}

#[cfg(test)]
mod tests {
    extern crate uniq_comp_id;

    use uniq_comp_id::UniqCompId;

    #[derive(UniqCompId)]
    struct ComponentOne;
    #[derive(UniqCompId)]
    struct ComponentTwo;
    #[derive(UniqCompId)]
    struct ComponentThree;

    #[test]
    fn increasing_ids() {
        assert_eq!(ComponentOne::id(), 0);
        assert_eq!(ComponentTwo::id(), 1);
        assert_eq!(ComponentThree::id(), 2);
    }
}
