//  DIRECTORY.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 09:04:29
//  Last edited:
//    25 Jun 2023, 12:05:55
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the Directory trait, which implements a single directory's
//!   layout.
// 

use std::collections::HashMap;
use std::error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::{self, DirEntry, ReadDir};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};


/***** ERRORS *****/
/// Defines errors that may occur when generating directories or when initializing dynamic types (such as [`HashMap<PathBuf, T>`]).
#[derive(Debug)]
pub enum Error {
    // Dynamic initialization
    /// Failed to read a directory.
    DirRead { path: PathBuf, err: std::io::Error },
    /// Failed to read an entry within a directory.
    DirEntryRead { path: PathBuf, entry: usize, err: std::io::Error },
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            DirRead { path, .. }             => write!(f, "Failed to read directory '{}'", path.display()),
            DirEntryRead { path, entry, .. } => write!(f, "Failed to read entry {} in directory '{}'", entry, path.display()),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            DirRead { err, .. }      => Some(err),
            DirEntryRead { err, .. } => Some(err),
        }
    }
}
impl From<std::convert::Infallible> for Error {
    #[inline]
    fn from(_value: std::convert::Infallible) -> Self { unreachable!(); }
}





/***** DEFAULT IMPLEMENTATIONS *****/
// Default implementation for the [`PathBuf`].
impl Directory for PathBuf {
    type Error = std::convert::Infallible;

    #[inline]
    fn try_init(base: impl Into<PathBuf>) -> Result<Self, Self::Error> { Ok(PathBuf::from(base.into())) }
}
impl DirectoryExt for PathBuf {
    #[inline]
    fn exists(&self) -> bool { <Path>::exists(self) }
}

// Default implementation for the [`Option<impl Directory>`] type, which can be used to only instantiate it if it exists.
impl<T: Directory> Directory for Option<T> {
    type Error = std::convert::Infallible;

    fn try_init(base: impl Into<PathBuf>) -> Result<Self, Self::Error> {
        let base: PathBuf = base.into();
        if base.exists() {
            Ok(Some(T::init(base)))
        } else {
            Ok(None)
        }
    }
}
impl<T: DirectoryExt> DirectoryExt for Option<T> {
    fn exists(&self) -> bool {
        match self {
            Some(path) => path.exists(),
            // We mark that it exists because a missing optional path counts as all mandatory paths existing
            None       => true,
        }
    }
}

// Default implementation for the [`HashMap<PathBuf, impl Directory>`] type, which can be used to dynamically scan for directories.
impl<T: Directory> Directory for HashMap<PathBuf, T> where Error: From<T::Error> {
    type Error = Error;

    fn try_init(base: impl Into<PathBuf>) -> Result<Self, Self::Error> {
        let base: PathBuf = base.into();

        // Scan the directory for directories
        let mut result: HashMap<PathBuf, T> = HashMap::new();
        let entries: ReadDir = match fs::read_dir(&base) {
            Ok(entries) => entries,
            Err(err) => {
                // If we failed to read the directory because it does not exist, we conclude no files exist either
                if err.kind() == ErrorKind::NotFound { return Ok(HashMap::new()); }
                // Otherwise, error hard
                return Err(Error::DirRead { path: base, err });
            },
        };
        for (i, entry) in entries.enumerate() {
            // Unwrap the entry
            let entry: DirEntry = match entry {
                Ok(entry) => entry,
                Err(err)  => { return Err(Error::DirEntryRead { path: base, entry: i, err }); },
            };

            // Initialize the entry
            let entry_path: PathBuf = entry.path();
            let nested: T = T::try_init(&entry_path)?;

            // Add the entry to the dynamic set
            result.insert(entry_path, nested);
        }

        // Done, return the found entries
        Ok(result)
    }
}
impl<T: DirectoryExt> DirectoryExt for HashMap<PathBuf, T> where Error: From<T::Error> {
    fn exists(&self) -> bool {
        // Iterate to only check those we found
        let mut exists: bool = true;
        for nested in self.values() {
            exists &= nested.exists();
        }
        exists
    }
}





/***** LIBRARY *****/
/// A Directory defines the layout of a file or directory. It can be used as a convenient hardcoded structure or files in a configuration folder.
pub trait Directory: Sized {
    /// The error type to use if initialization fails.
    type Error: error::Error;


    /// Initializes the directory by deducing all of the paths.
    /// 
    /// # Arguments
    /// - `base`: A [`Path`] that defines the base for any relative paths in this directory.
    /// 
    /// # Returns
    /// A new instance of Self with all the [`PathBuf`] fields (and other Directory fields) properly initialized.
    /// 
    /// # Panics
    /// This function may panic if this field failed to initialize or any of the fields panics when initializing them. This may be in the case of dynamic fields, such as a [`HashMap<PathBuf, T>`].
    fn init(base: impl Into<PathBuf>) -> Self { Self::try_init(base).unwrap_or_else(|err| panic!("Failed to initialize {}: {}", std::any::type_name::<Self>(), err)) }

    /// Initializes the directory by deducing all of the paths.
    /// 
    /// # Arguments
    /// - `base`: A [`Path`] that defines the base for any relative paths in this directory.
    /// 
    /// # Returns
    /// A new instance of Self with all the [`PathBuf`] fields (and other Directory fields) properly initialized.
    /// 
    /// # Errors
    /// This function may errors if this field failed to initialize or any of the fields errors when initializing them. This may be in the case of dynamic fields, such as a [`HashMap<PathBuf, T>`].
    fn try_init(base: impl Into<PathBuf>) -> Result<Self, Self::Error>;
}



/// Defines nice, additional things to implement for [`Directory`]s.
pub trait DirectoryExt: Directory {
    /// Returns if all mandatory paths in this directory exist.
    /// 
    /// In the case of optional paths, we do check if mandatory sub-paths exist if the path itself exists.
    /// 
    /// # Returns
    /// True if they do, false if they don't.
    fn exists(&self) -> bool;
}
