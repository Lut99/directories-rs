//  DIRECTORY.rs
//    by Lut99
// 
//  Created:
//    23 Apr 2023, 10:45:48
//  Last edited:
//    24 Jun 2023, 14:29:08
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the derivation for the directory.
// 

use std::collections::HashMap;
use std::path::PathBuf;

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro_error::{Diagnostic, Level};
use quote::quote;
use syn::{Attribute, Data, DataStruct, Expr, Generics, Ident, ImplGenerics, Lit, Meta, PathArguments, Token, Type, TypePath, Visibility};
use syn::__private::Span;
use syn::parse::ParseBuffer;
use syn::spanned::Spanned as _;

pub use crate::errors::{DirectoryError as Error, DirectoryErrorKind as ErrorKind};


/***** CONSTANTS *****/
lazy_static! {
    /// Defines the list of extensions that we automatically recognize.
    static ref EXTENSIONS: HashMap<&'static str, &'static str> = HashMap::from([
        ("_dat", ".dat"),
        ("_exe", ".exe"),
        ("_py",  ".py"),
        ("_txt", ".txt"),
    ]);
}





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
fn parse_toplevel_attrs(attrs: impl AsRef<[Attribute]>) -> Result<DirectoryAttributes, Error<'static>> {
    let attrs: &[Attribute] = attrs.as_ref();

    // Iterate over them
    for attr in attrs {
        match &attr.meta {
            Meta::List(l) => if l.path.is_ident("directories") {
                /* Nothing to do */
            },

            // Ignore the rest
            Meta::NameValue(_) |
            Meta::Path(_)      => {},
        }
    }

    // Done
    Ok(DirectoryAttributes {})
}

/// Extracts the information we want from field attributes.
/// 
/// # Arguments
/// - `attrs`: The list of attributes given at a field.
/// 
/// # Returns
/// A new [`FieldAttributes`] struct that contains the parsed information.
/// 
/// # Errors
/// This function may error if the attribute tokens were invalid.
fn parse_field_attrs(attrs: impl AsRef<[Attribute]>) -> Result<FieldAttributes, Error<'static>> {
    let attrs: &[Attribute] = attrs.as_ref();

    // Iterate over them
    let mut res: FieldAttributes = FieldAttributes::empty();
    'attrs: for attr in attrs {
        match &attr.meta {
            Meta::List(l) => if l.path.is_ident("file") || l.path.is_ident("dir") {
                // It's a file tag
                let args: Vec<Meta> = match attr.parse_args_with(|buffer: &ParseBuffer| {
                    // Repeatedly parsed metas separated by commands
                    let mut metas: Vec<Meta> = vec![ buffer.parse()? ];
                    while !buffer.is_empty() {
                        // Parse a comma then a meta
                        buffer.parse::<Token!(,)>()?;
                        metas.push(buffer.parse()?);
                    }
                    Ok(metas)
                }) {
                    Ok(args) => args,
                    Err(err) => {
                        // Emit the error and continue to the next attribute
                        Diagnostic::spanned(l.tokens.span(), Level::Error, "Cannot parse arguments for file-attribute".into()).span_error(err.span(), err.to_string()).emit();
                        continue 'attrs;
                    },
                };

                // Examine the found meta
                for arg in args {
                    match arg {
                        Meta::NameValue(nv) => if nv.path.is_ident("path") {
                            // It's the path identifier!

                            // Parse the thing after the equals as a string expression
                            let value: String = match &nv.value {
                                Expr::Lit(lit) => match &lit.lit {
                                    Lit::Str(s) => s.value(),
                                    _ => {
                                        Diagnostic::spanned(l.path.span(), Level::Error, "Expected string literal".into()).emit();
                                        continue 'attrs;
                                    },
                                },

                                _ => {
                                    Diagnostic::spanned(l.path.span(), Level::Error, "Expected string literal".into()).emit();
                                    continue 'attrs;
                                },
                            };

                            // Now we have the path, add it for this identifier
                            if let Some(old) = res.path {
                                Diagnostic::spanned(nv.path.span(), Level::Warning, "Duplicate path attribute".into()).span_note(old.1, "Previous occurrence is given here".into()).emit();
                            }
                            res.path = Some((value.into(), nv.value.span()));

                        } else {
                            Diagnostic::spanned(l.path.span(), Level::Error, format!("Unknown attribute{}", if let Some(i) = nv.path.get_ident() { format!(" '{}'", i.to_string()) } else { String::new() })).emit();
                            continue 'attrs;
                        },

                        // The rest we have nothing for yet
                        Meta::List(l) => {
                            Diagnostic::spanned(l.path.span(), Level::Error, format!("Unknown attribute{}", if let Some(i) = l.path.get_ident() { format!(" '{}'", i.to_string()) } else { String::new() })).emit();
                            continue 'attrs;
                        },
                        Meta::Path(p) => {
                            Diagnostic::spanned(p.span(), Level::Error, format!("Unknown attribute{}", if let Some(i) = p.get_ident() { format!(" '{}'", i.to_string()) } else { String::new() })).emit();
                            continue 'attrs;
                        },
                    }
                }
            },

            // Ignore the rest
            Meta::NameValue(_) |
            Meta::Path(_)      => {},
        }
    }

    // Done
    Ok(res)
}





/***** HELPER STRUCTS *****/
/// Defines everything we might learn from toplevel attributes.
#[derive(Clone, Debug)]
struct DirectoryAttributes {}



/// Defines everything we want to know of each field in a Directory.
#[derive(Clone)]
struct DirectoryField {
    /// Defines the identifier of the field.
    name : Ident,
    /// Defines the type of this field.
    ty   : Type,

    /// Defines the path of the field.
    path : PathBuf,
}

/// Defines everything we might learn from field attributes.
#[derive(Clone, Debug)]
struct FieldAttributes {
    /// An override for the default derived path.
    path : Option<(PathBuf, Span)>,
}
impl FieldAttributes {
    /// Constructor for the FieldAttributes that initializes it to empty (nothing parsed).
    /// 
    /// # Returns
    /// A new FieldAttributes struct with everything empty.
    #[inline]
    fn empty() -> Self {
        Self {
            path : None,
        }
    }
}





/***** LIBRARY *****/
/// Implements the derivation for the [`directories::Directory`] trait.
/// 
/// # Arguments
/// - `ident`: The identifier of the struct/enum/union we are hovering over.
/// - `data`: The parsed struct/enum/union body.
/// - `attrs`: Any attributes attached to this struct/enum/union.
/// - `generics`: Any generics attached to this struct/enum/union.
/// - `vis`: The visibility for this struct/enum/union.
/// 
/// # Returns
/// A TokenStream that contains the derived `impl`s.
/// 
/// # Errors
/// This function may error if it failed to parse the input properly.
/// 
/// Note that some non-fatal errors or warnings may be emitted during execution of this function.
pub fn derive(ident: Ident, data: Data, attrs: Vec<Attribute>, generics: Generics, vis: Visibility) -> Result<TokenStream, Error<'static>> {
    // First: let's extract this as a struct
    let data: DataStruct = match data {
        Data::Struct(s) => s,
        Data::Enum(e)   => { return Err(Error::new(e.enum_token.span, ErrorKind::NotAStruct)); },
        Data::Union(u)  => { return Err(Error::new(u.union_token.span, ErrorKind::NotAStruct)); },
    };

    // Next, we can collect any main struct attributes
    let _dir_attrs: DirectoryAttributes = parse_toplevel_attrs(attrs)?;

    // Time to dive into the struct's fields and get the information we need
    let mut fields: Vec<DirectoryField> = Vec::with_capacity(data.fields.len());
    for field in data.fields {
        // Parse the field attributes to find anything interesting
        let attrs: FieldAttributes = parse_field_attrs(&field.attrs)?;

        // Extract the name of the field
        let name: Ident = match field.ident {
            Some(ident) => ident,
            None => {
                Diagnostic::spanned(field.span(), Level::Error, "Missing of field".into()).emit();
                continue;
            },
        };

        // Deduce a default path if not given
        let path: PathBuf =  attrs.path.map(|(p, _)| p).unwrap_or_else(|| {
            // See if the name ends in a particular suffix
            let name: String = name.to_string();
            let name_len: usize = name.len();
            for (suffix, ext) in EXTENSIONS.iter() {
                if name.ends_with(suffix) {
                    return format!("{}{}", &name[..name_len - suffix.len()], ext).into();
                }
            }

            // Otherwise, just return the name
            PathBuf::from(&name)
        });

        // Construct the final type and add it
        fields.push(DirectoryField {
            name,
            ty : field.ty,

            path,
        });
    }

    // Change the fields into field initializations
    let mut fields_init: Vec<_> = Vec::with_capacity(fields.len());
    let mut fields_exists: Vec<_> = Vec::with_capacity(fields.len());
    for field in fields {
        let DirectoryField { name, ty, path } = field;

        // Preprocess the type into only the name of it (since we don't need to use generics when instantiating).
        let ty: TypePath = match ty {
            Type::Path(mut tp) => {
                // Remove the arguments from this path before returning the type
                for s in &mut tp.path.segments {
                    s.arguments = PathArguments::None;
                }
                tp
            },

            _ => {
                Diagnostic::spanned(ty.span(), Level::Error, "Cannot instantiate type".into()).span_suggestion(name.span(), "suggestion", "Add the 'init = \"<func>\"' attribute to initialize with a custom function".into()).abort();
            },
        };
        // Preprocess the path
        let spath: String = path.display().to_string();

        // Generate the instantiation, which may differ based on whether the path is absolute or not
        if path.is_absolute() {
            fields_init.push(quote! {
                #name : #ty::try_init(#spath)?,
            });
        } else {
            fields_init.push(quote! {
                #name : #ty::try_init(_base.join(#spath))?,
            });
        }
        // And the exists
        fields_exists.push(quote! {
            exists &= self.#name.exists();
        });
    }

    // Get the generics of this struct for writing the new tokenstream
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate a unique formatter name & generics
    let formatter_ident: Ident = Ident::new(&format!("_{ident}Formatter"), Span::call_site());

    // Return a version of the implementation with and one without generics
    Ok(quote! {
        /// Tailored directory formatter for #ident.
        #[derive(Debug)]
        #vis struct #formatter_ident<'d, #impl_generics> #where_clause {
            /// The directory to format.
            dir    : &'d #ident #ty_generics,
            /// The indentation to format with.
            indent : usize,
        }
        impl< ::std::fmt::Display for #formatter_ident<'d, #impl_generics>



        impl #impl_generics ::directories::Directory for #ident #ty_generics #where_clause {
            type Error = ::directories::Error;

            fn try_init(base: impl Into<::std::path::PathBuf>) -> Result<Self, Self::Error> {
                use ::directories::Directory as _;

                let _base: ::std::path::PathBuf = base.into();
                Ok(Self {
                    #(#fields_init)*
                })
            }
        }

        impl #impl_generics ::directories::DirectoryExt for #ident #ty_generics #where_clause {
            type Formatter 


            fn exists(&self) -> bool {
                use ::directories::DirectoryExt as _;

                let mut exists: bool = true;
                #(#fields_exists)*
                exists
            }
        }
    }.into())
}
