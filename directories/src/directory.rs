//  DIRECTORY.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 09:04:29
//  Last edited:
//    24 Jun 2023, 14:20:20
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
    type Formatter<'s> = crate::formatters::PathBufFormatter<'s>;


    #[inline]
    fn exists(&self) -> bool { <Path>::exists(self) }

    #[inline]
    fn display_indented<'s>(&'s self, indent: usize) -> Self::Formatter<'s> {
        crate::formatters::PathBufFormatter { path: self, indent }
    }
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
    type Formatter<'s> = crate::formatters::OptionFormatter<'s, T> where T: 's;


    fn exists(&self) -> bool {
        match self {
            Some(path) => path.exists(),
            // We mark that it exists because a missing optional path counts as all mandatory paths existing
            None       => true,
        }
    }

    #[inline]
    fn display_indented<'s>(&'s self, indent: usize) -> Self::Formatter<'s> {
        crate::formatters::OptionFormatter { option: self, indent }
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
            Err(err)    => { return Err(Error::DirRead { path: base, err }); },
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
    type Formatter<'s> = crate::formatters::HashMapFormatter<'s, T> where T: 's;


    fn exists(&self) -> bool {
        // Iterate to only check those we found
        let mut exists: bool = true;
        for nested in self.values() {
            exists &= nested.exists();
        }
        exists
    }

    #[inline]
    fn display_indented<'s>(&'s self, indent: usize) -> Self::Formatter<'s> {
        crate::formatters::HashMapFormatter { map: self, indent }
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
    /// The formatter to use in [`Self::display()`].
    type Formatter<'s>: Display where Self: 's;


    /// Returns if all mandatory paths in this directory exist.
    /// 
    /// In the case of optional paths, we do check if mandatory sub-paths exist if the optional paths exist.
    /// 
    /// # Arguments
    /// - `base`: A [`Path`] that defines the base for any relative paths in this directory.
    /// 
    /// # Returns
    /// True if they do, false if they don't.
    fn exists(&self) -> bool;



    /// Returns a formatter that formats the directory with all its paths.
    /// 
    /// # Returns
    /// A formatter implementing [`Display`].
    #[inline]
    fn display<'s>(&'s self) -> Self::Formatter<'s> { self.display_indented(0) }

    /// Returns a formatter that formats the directory with all its paths but with a certain indentation.
    /// 
    /// # Arguments
    /// - `indent`: The number of spaces to prefix to each line.
    /// 
    /// # Returns
    /// A formatter implementing [`Display`].
    fn display_indented<'s>(&'s self, indent: usize) -> Self::Formatter<'s>;
}
