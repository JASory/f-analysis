use crate::FResult;

// FIXME rename methods to_file and from_file, rename CompVector's to_file to something like to_cfile or
/// Trait for reading and writing structures to files
pub trait Persistent: Clone {
    /// Write to file
    fn to_persistent(&self, filename: &str) -> FResult<()>;

    /// Load from file
    fn from_persistent(x: &str) -> FResult<Self>;
}
