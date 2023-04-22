//  LIB.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 19:19:08
//  Last edited:
//    22 Apr 2023, 11:19:57
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


/***** PROC MACROS *****/
/// Derives the [`directories::Directory`] trait automagically.
#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Directory, attributes(this, dir, file))]
pub fn derive_directory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro_error::{abort_call_site, Diagnostic, Level};
    use quote::quote;
    use syn::{parse_macro_input, Data, DeriveInput};
    use syn::spanned::Spanned as _;
    use directory::{parse_entry_attributes, DirectoryField, DirectoryFieldKind};


    // Parse the thing we've gotten
    let DeriveInput{ ident, data, attrs, .. } = parse_macro_input!(input);

    // Match that which we're parsing
    match data {
        Data::Struct(s) => {
            // Find the fields in this struct and collect them
            let mut fields: Vec<DirectoryField> = Vec::with_capacity(s.fields.len());
            let mut seen_this: bool = false;
            for f in s.fields {
                // Check if this field has any relevant attributes
                let kind: DirectoryFieldKind = match parse_entry_attributes(&f) {
                    Ok(kind) => kind,
                    Err(err) => { err.abort(); },
                };

                // If it's `#[this]`, then mark we've already seen it
                if kind.is_this() {
                    if seen_this {
                        Diagnostic::spanned(f.span(), Level::Warning, "Duplicate `#[this]` marking".into()).emit();
                        continue;
                    } else {
                        seen_this = true;
                    }
                }

                // OK, let's add it as a full entry
                fields.push(DirectoryField {
                    field : f,
                    kind,
                })
            }

            // Derive an implementation of the constructor from our fields
            let constructor = quote!{
                impl #ident {
                    /// Constructor for the #ident.
                    /// 
                    /// # Arguments
                    /// - `path`: The base path that serves as the root to this directory.
                    /// 
                    /// # Returns
                    /// A new #ident instance which has its child paths fully initialized.
                    pub fn new(path: impl Into<::std::path::PathBuf>) -> Self {
                        
                    }
                }
            };

            // Combine everything and return
            quote!{
                #constructor
            }.into()
        },

        // The rest is not supported (for now)
        _ => { abort_call_site!("Can only derive `Directory` on structs"); },
    }
}
