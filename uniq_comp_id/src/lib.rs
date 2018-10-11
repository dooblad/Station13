/// Data type used to represent component IDs.
pub type CompId = u8;

/// Used to provide every component a unique ID number.
pub trait UniqCompId {
    fn id() -> CompId;
}
