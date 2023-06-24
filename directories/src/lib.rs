//  LIB.rs
//    by Lut99
// 
//  Created:
//    20 Apr 2023, 19:07:02
//  Last edited:
//    24 Jun 2023, 14:06:15
//  Auto updated?
//    Yes
// 
//  Description:
//!   A niche little crate that simplifies creating large hardcoded
//!   directory structures.
// 

// Declare the submodules
mod directory;
pub mod formatters;
pub mod std;

// Push some of that in the crate namespace
pub use directory::{Directory, DirectoryExt, Error};

// Use the derive macros
#[cfg(feature = "derive")]
pub use directories_derive::*;
