//  DIRECTORY.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 20:17:21
//  Last edited:
//    21 Apr 2023, 20:21:49
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the derive macro for the [`directories::Directory`]
//!   trait.
// 

use proc_macro::TokenStream;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Meta, Token};
use syn::punctuated::Punctuated;


/***** HELPER FUNCTIONS *****/
/// Parses the attribute(s) of a field in a directory structs.
/// 
/// # Arguments
/// - `attrs`: The list of attributes to parse.
/// 
/// # Returns
/// A [`DirEntry`] struct with the information we parsed.
/// 
/// # Errors
/// This function may error if we failed to parse the attributes for some reason.
pub(crate) fn parse_entry_attributes(attrs: &[Attribute]) -> Result<DirEntry, Error> {
    let mut name: String = String::new();
    for a in &f.attrs {
        match &a.meta {
            Meta::List(l) => {
                // Match on which of our attributes we are parsing
                if l.path.is_ident("dir") {
                    // Parse whatever the user wrote in there
                    let nested = match l.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                        Ok(nested) => nested,
                        Err(err)   => { abort!(l, "Invalid syntax for `dir` attribute: {} (expected Rust attribute syntax)", err); },
                    };

                    // Now match what they wrote
                    for nested_a in nested {
                        match nested_a {
                            Meta::List(l) => {

                            },

                            Meta::NameValue(nv) => {

                            },
    
                            Meta::Path(p) => {
    
                            },
                        }
                    }
                } /* Ignore otherwise */
            },

            Meta::NameValue(nv) => {

            },

            Meta::Path(p) => {

            },
        }
    }
}





/***** HELPER STRUCTS *****/
/// Defines what we need to know about each field in a struct.
pub(crate) struct DirEntry {
    /// The field that we're referencing (used to convey the span).
    field : Field,
    /// The name of the file/folder itself we will use on the disk (influenced by `#[dir(...)]` and `#[file(...)]`)
    name  : String,
}





/***** LIBRARY *****/

