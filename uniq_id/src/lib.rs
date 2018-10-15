pub mod serde;

/// Data type used to represent unique IDs.
pub type Id = u8;

/// Used to provide a unique ID number.
pub trait UniqId {
    fn id() -> Id;
}
