//  FORMATTERS.rs
//    by Lut99
// 
//  Created:
//    24 Jun 2023, 14:06:03
//  Last edited:
//    24 Jun 2023, 14:19:47
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the formatters for [`std`](::std) types.
// 

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;

use crate::directory::DirectoryExt;


/***** HELPER STRUCTS *****/
/// Generates `n` spaces.
struct Indent(usize);
impl Display for Indent {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        for i in 0..self.0 { write!(f, " ")?; }
        Ok(())
    }
}





/***** LIBRARY *****/
/// Defines a directory formatter for [`PathBuf`].
#[derive(Debug)]
pub struct PathBufFormatter<'p> {
    /// The PathBuf to format.
    pub(crate) path   : &'p PathBuf,
    /// The indentation to use for every line.
    pub(crate) indent : usize,
}
impl<'o> Display for PathBufFormatter<'o> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{} - {}", Indent(self.indent), self.path.display())
    }
}



/// Defines a directory formatter for [`Option<T>`].
#[derive(Debug)]
pub struct OptionFormatter<'o, T> {
    /// The Option to format.
    pub(crate) option : &'o Option<T>,
    /// The indentation to use for every line.
    pub(crate) indent : usize,
}
impl<'o, T: DirectoryExt> Display for OptionFormatter<'o, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // List all of the things in this map
        if let Some(nested) = &self.option {
            write!(f, "{}", nested.display_indented(self.indent))
        } else {
            Ok(())
        }
    }
}



/// Defines a directory formatter for [`HashMap<PathBuf, T>`].
#[derive(Debug)]
pub struct HashMapFormatter<'m, T> {
    /// The HashMap to format.
    pub(crate) map    : &'m HashMap<PathBuf, T>,
    /// The indentation to use for every line.
    pub(crate) indent : usize,
}
impl<'m, T: DirectoryExt> Display for HashMapFormatter<'m, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // List all of the things in this map
        for (path, nested) in self.map {
            writeln!(f, "{} - {}", Indent(self.indent), path.display())?;
            write!(f, "{}", nested.display_indented(self.indent + 3))?;
        }
        Ok(())
    }
}
