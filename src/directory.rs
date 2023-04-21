//  DIRECTORY.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 09:04:29
//  Last edited:
//    21 Apr 2023, 09:22:15
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the Directory trait, which implements a single directory's
//!   layout.
// 

use crate::errors::DirError as Error;


/***** LIBRARY *****/
/// A Directory is a struct that defines the layout of a directory.
pub trait Directory {
    /// Creates this directory.
    /// 
    /// This does not create any of the directory's contents yet. To do so, use [`Directory::initialize()`], or consider using [`Directory::create_and_init()`] instead.
    /// 
    /// # Errors
    /// This function errors if we failed to create ourselves.
    fn create(&self) -> Result<(), Error>;
    /// Creates this directory and its (hardcoded) contents.
    /// 
    /// This effectively calls [`Directory::create()`], then [`Directory::initialize()`]. See the latter function for more details on this process.
    /// 
    /// # Errors
    /// This function may error if we failed to create any hardcoded directory or file.
    #[inline]
    fn create_and_init(&self) -> Result<(), Error> {
        // Do the two promised calls
        self.create()?;
        self.initialize()
    }
    /// Removes this directory and its contents.
    /// 
    /// # Errors
    /// This function errors if we failed to remove any part of it.
    fn remove(&self) -> Result<(), Error>;

    /// Generate the hardcoded contents of this directory if they don't exist already.
    /// 
    /// This is a recursive function, meaning that any nested directories will also be initialized. Note that nested hardcoded files will only be created if they provide a [`Default`] implementation.
    /// 
    /// Does not create the directory itself; see [`Directory::create_and_init()`] for that.
    /// 
    /// # Errors
    /// This function errors if we failed to create any of the directories.
    fn initialize(&self) -> Result<(), Error>;
    /// Clean the contents of this directory, removing everything within.
    /// 
    /// Note that this does not necessarily re-initialize the directory's structure again; either call [`Directory::initialize()`] yourself, or consider using [`Directory::clean_and_init()`].
    /// 
    /// Does not remove the directory itself.
    /// 
    /// # Errors
    /// This function errors if we failed to remove any of the directories.
    fn clean(&self) -> Result<(), Error>;
    /// Clean the contents of the directory, removing everything within, and then re-initializes its contents to the default layout.
    /// 
    /// Essentially calls [`Directory::clean()`], then [`Directory::initialize()`]. See the latter function for more details on this process.
    /// 
    /// # Errors
    /// This function errors if we either failed to remove anything nested, or re-create it.
    #[inline]
    fn clean_and_init(&self) -> Result<(), Error> {
        // Do the two promised calls
        self.clean()?;
        self.initialize()
    }
}
