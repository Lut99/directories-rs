//  LIB.rs
//    by Lut99
// 
//  Created:
//    20 Apr 2023, 19:07:02
//  Last edited:
//    21 Apr 2023, 19:22:54
//  Auto updated?
//    Yes
// 
//  Description:
//!   A niche little crate that simplifies creating large hardcoded
//!   directory structures.
// 

// Declare the submodules
pub mod errors;
pub mod directory;

// Push some of that in the crate namespace
pub use directory::Directory;

// Use the derive macros
#[cfg(feature = "derive")]
pub use directories_derive::*;
