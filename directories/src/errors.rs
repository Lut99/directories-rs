//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 09:06:35
//  Last edited:
//    21 Apr 2023, 09:09:47
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines errors originating from the `directories` crate.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;


/***** LIBRARY *****/
/// Defines errors that originate from the Directory trait/structs.
#[derive(Debug)]
pub enum DirError {
    /// Failed to create this directory.
    Create{ path: PathBuf, err: std::io::Error },
}
impl Display for DirError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use DirError::*;
        match self {
            Create{ path, err: _ } => write!(f, "Failed to create directory '{}'", path.display()),
        }
    }
}
impl Error for DirError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use DirError::*;
        match self {
            Create{ err, .. } => Some(err),
        }
    }
}
