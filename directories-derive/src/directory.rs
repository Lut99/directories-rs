//  DIRECTORY.rs
//    by Lut99
// 
//  Created:
//    23 Apr 2023, 10:45:48
//  Last edited:
//    23 Apr 2023, 11:25:53
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the derivation for the directory.
// 

use proc_macro::TokenStream;
use proc_macro_error::Diagnostic;
use quote::quote;
use syn::{Attribute, Data, DataStruct, Generics, Ident};

pub use crate::errors::{DirectoryError as Error, DirectoryErrorKind as ErrorKind};


/***** HELPER FUNCTIONS *****/
/// Extracts the information we want from the toplevel attributes.
/// 
/// # Arguments
/// - `attrs`: The list of attributes given at toplevel.
/// 
/// # Returns
/// A new [`DirectoryAttributes`] struct that contains the parsed information.
/// 
/// # Errors
/// This function may errors if the attribute tokens were invalid.
fn parse_toplevel_attrs<'a>(attrs: impl 'a + AsRef<[Attribute]>) -> Result<DirectoryAttributes, Error<'a>> {
    let _attrs: &[Attribute] = attrs.as_ref();

    /* Nothing to do, as of yet */

    // Done
    Ok(DirectoryAttributes {})
}





/***** HELPER STRUCTS *****/
/// Defines everything we might learn from toplevel attributes.
#[derive(Clone, Debug)]
struct DirectoryAttributes {}





/***** LIBRARY *****/
/// Implements the derivation for the [`directories::Directory`] trait.
/// 
/// # Arguments
/// - `ident`: The identifier of the struct/enum/union we are hovering over.
/// - `data`: The parsed struct/enum/union body.
/// - `attrs`: Any attributes attached to this struct/enum/union.
/// - `generics`: Any generics attached to this struct/enum/union.
/// 
/// # Returns
/// A TokenStream that contains the derived `impl`s.
/// 
/// # Errors
/// This function may error if it failed to parse the input properly.
/// 
/// Note that some non-fatal errors or warnings may be emitted during execution of this function.
pub fn derive(ident: Ident, data: Data, attrs: Vec<Attribute>, generics: Generics) -> Result<TokenStream, Error<'static>> {
    // First: let's extract this as a struct
    let data: DataStruct = match data {
        Data::Struct(s) => s,
        Data::Enum(e)   => { return Err(Error::new(e.enum_token.span, ErrorKind::NotAStruct)); },
        Data::Union(u)  => { return Err(Error::new(u.union_token.span, ErrorKind::NotAStruct)); },
    };

    // Next, we can collect any main struct attributes
    let dir_attrs: DirectoryAttributes = parse_toplevel_attrs(attrs)?;

    // Done
    Ok(quote!{}.into())
}
