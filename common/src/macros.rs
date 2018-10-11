/// Makes it more ergonomic to define which components a system requires.
///
/// Example:
/// ```ignore
/// let filter = type_id_vec!(PositionComponent, HealthComponent);
/// ```
///
/// Credit goes to "AndyBaron/rustic-ecs" on Github for this one.
#[macro_export]
macro_rules! type_id_vec {
    ($($x:ty),*) => (
        vec![$(TypeId::of::<$x>()),*]
    );
    ($($x:ty,)*) => (type_id_vec![$($x),*])
}

/// For defining a list of boxed systems by name.
///
/// Example:
/// ```ignore
/// let systems = sys_vec![
///     PlayerUpdateSystem,
///     RandomMobUpdateSystem,
/// ];
/// ```
#[macro_export]
macro_rules! sys_vec {
    ($($x:ident),*) => (
        vec![$(Box::new($x {})),*]
    );
    ($($x:ident,)*) => (sys_vec![$($x),*])
}
