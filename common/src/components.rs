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

#[cfg(test)]
mod tests {
    extern crate uniq_id;

    use uniq_id::UniqId;

    #[test]
    fn increasing_ids_same_group() {
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TSOne;
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TSTwo;
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TSThree;

        assert_eq!(TSOne::id(), 0);
        assert_eq!(TSTwo::id(), 1);
        assert_eq!(TSThree::id(), 2);
    }

    // TODO: The `should_panic` attribute doesn't work for proc macros.  File an issue?
    //#[test]
    //#[should_panic]
    //fn attempt_use_default_group() {
    //    #[derive(UniqId)]
    //    #[UniqGroup = "default"]
    //    struct TSOne;
    //}

    #[test]
    fn same_ids_diff_groups() {
        #[derive(UniqId)]
        #[UniqGroup = "test1"]
        struct TSOne;
        #[derive(UniqId)]
        #[UniqGroup = "test2"]
        struct TSTwo;

        assert_eq!(TSOne::id(), 0);
        assert_eq!(TSTwo::id(), 0);
    }
}
