//  STD.rs
//    by Lut99
// 
//  Created:
//    24 Jun 2023, 13:52:10
//  Last edited:
//    25 Jun 2023, 12:06:19
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines a few nice types to have on default that implement
//!   [`Directory`].
// 

use std::collections::HashMap;
use std::fs::{self, DirEntry, ReadDir};
use std::io::ErrorKind;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use crate::directory::{Directory, DirectoryExt, Error};


/***** LIBRARY *****/
/// Defines a dynamic directory, like [`HashMap<PathBuf, T>`], except that it only notes nested things for which [`T::exists()`](DirectoryExt::exists()) holds true.
#[derive(Clone, Debug)]
pub struct Dynamic<T>(HashMap<PathBuf, T>);

impl<T: DirectoryExt> Directory for Dynamic<T> where Error: From<T::Error> {
    type Error = Error;

    fn try_init(base: impl Into<PathBuf>) -> Result<Self, Self::Error> {
        let base: PathBuf = base.into();

        // Scan the directory for directories
        let mut result: HashMap<PathBuf, T> = HashMap::new();
        let entries: ReadDir = match fs::read_dir(&base) {
            Ok(entries) => entries,
            Err(err) => {
                // If we failed to read the directory because it does not exist, we conclude no files exist either
                if err.kind() == ErrorKind::NotFound { return Ok(Dynamic(HashMap::new())); }
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

            // Filter out nested types which do not exist
            if !nested.exists() { continue; }

            // Add the entry to the dynamic set
            result.insert(entry_path, nested);
        }

        // Done, return the found entries
        Ok(Self(result))
    }
}
impl<T: DirectoryExt> DirectoryExt for Dynamic<T> where Error: From<T::Error> {
    fn exists(&self) -> bool {
        // Iterate to only check those we found
        let mut exists: bool = true;
        for nested in self.0.values() {
            exists &= nested.exists();
        }
        exists
    }
}

impl<T> AsRef<HashMap<PathBuf, T>> for Dynamic<T> {
    #[inline]
    fn as_ref(&self) -> &HashMap<PathBuf, T> { &self.0 }
}
impl<T> AsMut<HashMap<PathBuf, T>> for Dynamic<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut HashMap<PathBuf, T> { &mut self.0 }
}
impl<T> Deref for Dynamic<T> {
    type Target = HashMap<PathBuf, T>;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> DerefMut for Dynamic<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
impl<T> From<Dynamic<T>> for HashMap<PathBuf, T> {
    #[inline]
    fn from(value: Dynamic<T>) -> Self { value.0 }
}
impl<T: Clone> From<&Dynamic<T>> for HashMap<PathBuf, T> {
    #[inline]
    fn from(value: &Dynamic<T>) -> Self { value.0.clone() }
}
impl<T: Clone> From<&mut Dynamic<T>> for HashMap<PathBuf, T> {
    #[inline]
    fn from(value: &mut Dynamic<T>) -> Self { Self::from(&*value) }
}

impl<T> IntoIterator for Dynamic<T> {
    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, T>;
    type Item     = (PathBuf, T);

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
impl<'d, T> IntoIterator for &'d Dynamic<T> {
    type IntoIter = std::collections::hash_map::Iter<'d, PathBuf, T>;
    type Item     = (&'d PathBuf, &'d T);

    #[inline]
    fn into_iter(self) -> Self::IntoIter { (&self.0).into_iter() }
}
impl<'d, T> IntoIterator for &'d mut Dynamic<T> {
    type IntoIter = std::collections::hash_map::IterMut<'d, PathBuf, T>;
    type Item     = (&'d PathBuf, &'d mut T);

    #[inline]
    fn into_iter(self) -> Self::IntoIter { (&mut self.0).into_iter() }
}
