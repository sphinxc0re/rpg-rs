use std::path::Path;
use errors::*;

/// An abtract asset
pub trait Asset {
    /// Load an asset from a file
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> where Self: ::std::marker::Sized;
}
