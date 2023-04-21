//  LIB.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 19:19:08
//  Last edited:
//    21 Apr 2023, 20:21:41
//  Auto updated?
//    Yes
// 
//  Description:
//!   The crate that implements the `#[derive(...)]` macro for the
//!   `directories` crate.
// 

// Declare the submodules
mod directory;


/***** PROC MACROS *****/
/// Derives the [`directories::Directory`] trait automagically.
#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Directory, attributes(dir, file))]
pub fn derive_directory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro_error::{abort, abort_call_site};
    use quote::quote;
    use syn::{parse_macro_input, Data, DeriveInput, Meta, Token};
    use syn::punctuated::Punctuated;
    use directory::{parse_entry_attributes, DirEntry};


    // Parse the thing we've gotten
    let DeriveInput{ ident, data, attrs, .. } = parse_macro_input!(input);

    // Match that which we're parsing
    match data {
        Data::Struct(s) => {
            // Find the fields in this struct and collect them
            let mut dir_contents: Vec<DirEntry> = Vec::with_capacity(s.fields.len());
            for f in s.fields {
                // Check if this field has any relevant attributes
                let entry: DirEntry = parse_entry_attributes(&f.attrs);

                // OK, let's add it
                dir_contents.push(DirEntry{ field: f, name });
            }

            // Derive an implementation
            quote!{
                impl ::std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "Hello there!")
                    }
                }
            }.into()
        },

        // The rest is not supported (for now)
        _ => { abort_call_site!("Can only derive `Directory` on structs"); },
    }
}
