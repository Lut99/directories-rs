//  LIB.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 19:19:08
//  Last edited:
//    23 Apr 2023, 10:56:52
//  Auto updated?
//    Yes
// 
//  Description:
//!   The crate that implements the `#[derive(...)]` macro for the
//!   `directories` crate.
// 

// Declare the submodules
mod errors;
mod directory;

// Input stuff
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};


/***** PROC MACROS *****/
/// Derives the [`directories::Directory`] trait automagically.
#[proc_macro_error]
#[proc_macro_derive(Directory, attributes(this, dir, file))]
pub fn derive_directory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Let's parse the input as a Data struct
    let DeriveInput{ ident, data, attrs, generics, .. } = parse_macro_input!(input);

    // Run the derive for the directories
    match directory::derive(ident, data, attrs, generics) {
        Ok(stream) => stream,
        Err(err)   => { err.abort(); },
    }
}
