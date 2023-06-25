//  DIRECTORY.rs
//    by Lut99
// 
//  Created:
//    23 Apr 2023, 10:45:48
//  Last edited:
//    25 Jun 2023, 12:04:01
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the derivation for the directory.
// 

use std::collections::HashMap;
use std::path::PathBuf;

use enum_debug::EnumDebug;
use proc_macro::TokenStream;
use proc_macro_error::{Diagnostic, Level};
use quote::quote;
use syn::{Attribute, Data, DataStruct, Expr, Generics, Ident, Lit, Meta, Token, Type, Visibility};
use syn::__private::Span;
use syn::parse::ParseBuffer;
use syn::spanned::Spanned as _;

pub use crate::errors::{DirectoryError as Error, DirectoryErrorKind as ErrorKind};


/***** CONSTANTS *****/
/// Defines the list of extensions that we automatically recognize.
const EXTENSIONS: [ &str; 32 ] = [
    // Executable files
    "out", "exe",

    // Source files
    "rs", "c", "cpp", "cxx", "h", "hpp", "hxx", "py", "lua", "java", "R", "md", "html", "css", "js", "ts",

    // Configuration files
    "json", "yaml", "yml", "cfg",

    // Data files
    "csv", "pickle", "dat",

    // Images
    "jpg", "jpeg", "png", "gif", "svg", "tiff",

    // Miscellaneous
    "txt",
];





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
    let mut res: DirectoryAttributes = DirectoryAttributes::empty();
    'attrs: for attr in attrs {
        match &attr.meta {
            Meta::List(l) => if l.path.is_ident("directories") {
                // Parse the arguments of this tag
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
                        Diagnostic::spanned(l.tokens.span(), Level::Error, "Cannot parse arguments for directory-attribute".into()).span_error(err.span(), err.to_string()).emit();
                        continue 'attrs;
                    },
                };

                // Examine the found meta
                for arg in args {
                    match arg {
                        Meta::NameValue(nv) => if nv.path.is_ident("ext") {
                            // It's the extension identifier!

                            // Parse the value as a string
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

                            // Either split the value or compute the joint suffix/extension
                            let (suffix, ext): (String, String) = match value.find(':') {
                                Some(pos) => (value[..pos].into(), value[pos + 1..].into()),
                                None => (format!("_{value}"), format!(".{value}")),
                            };

                            // Add it to the database
                            res.exts.insert(suffix, ext);

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
    let mut seen_file_dir: Option<Span> = None;
    let mut res: FieldAttributes = FieldAttributes::empty();
    'attrs: for attr in attrs {
        match &attr.meta {
            Meta::List(l) => if l.path.is_ident("file") || l.path.is_ident("dir") {
                seen_file_dir = Some(l.span());

                // Assert we haven't seen `#[this]` yet
                if let Some(old) = res.this {
                    Diagnostic::spanned(l.path.span(), Level::Error, "Field cannot be both '#[file]'/'#[dir]' and '#[this]".into()).span_note(old, "Conflicting attribute given here".into()).emit();
                    continue 'attrs;
                }

                // Parse the arguments of this tag
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
                        Diagnostic::spanned(l.tokens.span(), Level::Error, "Cannot parse arguments for file/dir-attribute".into()).span_error(err.span(), err.to_string()).emit();
                        continue 'attrs;
                    },
                };

                // Examine the found meta
                for arg in args {
                    match arg {
                        Meta::NameValue(nv) => if nv.path.is_ident("path") {
                            // It's the path identifier!

                            // Assert it has not been flattened yet
                            if let Some(old) = res.flatten {
                                Diagnostic::spanned(nv.path.span(), Level::Error, format!("Field cannot be both '#[{}](path)' and '#[{}](flatten)'", l.path.get_ident().unwrap(), l.path.get_ident().unwrap())).span_note(old, "Conflicting attribute given here".into()).emit();
                                continue 'attrs;
                            }

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
                                Diagnostic::spanned(nv.path.span(), Level::Warning, format!("Duplicate '#[{}(path)]' attribute", l.path.get_ident().unwrap())).span_note(old.1, "Previous occurrence is given here".into()).emit();
                            }
                            res.path = Some((value.into(), nv.value.span()));

                        } else {
                            Diagnostic::spanned(l.path.span(), Level::Error, format!("Unknown attribute{}", if let Some(i) = nv.path.get_ident() { format!(" '{}'", i.to_string()) } else { String::new() })).emit();
                            continue 'attrs;
                        },

                        // Parse the 'flatten' thing
                        Meta::Path(p) => if p.is_ident("flatten") {
                            // Assert not path has been parsed yet
                            if let Some(old) = res.path.take() {
                                Diagnostic::spanned(p.span(), Level::Error, format!("Field cannot be both '#[{}](path)' and '#[{}](flatten)'", l.path.get_ident().unwrap(), l.path.get_ident().unwrap())).span_note(old.1, "Conflicting attribute given here".into()).emit();
                                continue 'attrs;
                            }

                            // Mark this as flattened
                            if let Some(old) = res.flatten {
                                Diagnostic::spanned(p.span(), Level::Warning, format!("Duplicate '#[{}(flatten)]' attribute", l.path.get_ident().unwrap())).span_note(old, "Previous occurrence is given here".into()).emit();
                            }
                            res.flatten = Some(p.span());

                        } else {
                            Diagnostic::spanned(p.span(), Level::Error, format!("Unknown attribute{}", if let Some(i) = p.get_ident() { format!(" '{}'", i.to_string()) } else { String::new() })).emit();
                            continue 'attrs;
                        },

                        // The rest we have nothing for yet
                        Meta::List(l) => {
                            Diagnostic::spanned(l.path.span(), Level::Error, format!("Unknown attribute{}", if let Some(i) = l.path.get_ident() { format!(" '{}'", i.to_string()) } else { String::new() })).emit();
                            continue 'attrs;
                        },
                    }
                }
            },

            // Look for this
            Meta::Path(p) => if p.is_ident("this") {
                // Assert this is not a `#[file]` or `#[dir]`
                if let Some(old) = seen_file_dir {
                    Diagnostic::spanned(p.span(), Level::Error, "Field cannot be both '#[file]'/'#[dir]' and '#[this]".into()).span_note(old, "Conflicting attribute given here".into()).emit();
                    continue 'attrs;
                }

                // Simply set ourselves to this
                if let Some(old) = res.this {
                    Diagnostic::spanned(p.span(), Level::Warning, "Duplicate '#[this]' attribute".into()).span_note(old, "Previous occurrence is given here".into()).emit();
                }
                res.this = Some(p.span());
            },

            // Ignore the rest
            Meta::NameValue(_) => {},
        }
    }

    // Done
    Ok(res)
}





/***** HELPER STRUCTS *****/
/// Defines everything we might learn from toplevel attributes.
#[derive(Clone, Debug)]
struct DirectoryAttributes {
    /// The database of extensions for this directory.
    exts : HashMap<String, String>,
}
impl DirectoryAttributes {
    /// Constructor for the DirectoryAttributes that initializes it to empty (nothing parsed).
    /// 
    /// # Returns
    /// A new DirectoryAttributes struct with everything empty.
    #[inline]
    fn empty() -> Self {
        Self {
            exts : EXTENSIONS.iter().map(|e| (format!("_{e}"), format!(".{e}"))).collect(),
        }
    }
}



/// Defines everything we want to know of each field in a Directory.
#[derive(Clone)]
struct DirectoryField {
    /// Defines the identifier of the field.
    name : Ident,
    /// Defines the type of this field.
    ty   : Type,

    /// Defines the mode of the field.
    mode : FieldMode,
}

#[derive(Clone, Debug, EnumDebug)]
enum FieldMode {
    /// It's a path, as per usual
    Path(PathBuf),
    /// It's flattened.
    Flatten,
}


/// Defines everything we want to know of the `#[this]`-field.
#[derive(Clone)]
struct ThisField {
    /// Defines the identifier of the field.
    name : Ident,
}


/// Defines everything we might learn from field attributes.
#[derive(Clone, Debug)]
struct FieldAttributes {
    /// If true, then this is the `#[this]` field.
    this : Option<Span>,

    /// Whether the field is flattened or not.
    flatten : Option<Span>,
    /// An override for the default derived path.
    path    : Option<(PathBuf, Span)>,
}
impl FieldAttributes {
    /// Constructor for the FieldAttributes that initializes it to empty (nothing parsed).
    /// 
    /// # Returns
    /// A new FieldAttributes struct with everything empty.
    #[inline]
    fn empty() -> Self {
        Self {
            this : None,

            flatten : None,
            path    : None,
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
pub fn derive(ident: Ident, data: Data, attrs: Vec<Attribute>, generics: Generics, _vis: Visibility) -> Result<TokenStream, Error<'static>> {
    // First: let's extract this as a struct
    let data: DataStruct = match data {
        Data::Struct(s) => s,
        Data::Enum(e)   => { return Err(Error::new(e.enum_token.span, ErrorKind::NotAStruct)); },
        Data::Union(u)  => { return Err(Error::new(u.union_token.span, ErrorKind::NotAStruct)); },
    };

    // Next, we can collect any main struct attributes
    let dir_attrs: DirectoryAttributes = parse_toplevel_attrs(attrs)?;

    // Time to dive into the struct's fields and get the information we need
    let mut seen_this: Option<Span> = None;
    let mut this: Option<ThisField> = None;
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

        // Switch on whether this is a `#[this]`-field or not
        if let Some(span) = attrs.this {
            // Assert it's the only one of its kind
            if let Some(old) = seen_this {
                Diagnostic::spanned(name.span(), Level::Error, "Cannot have multiple `#[this]` fields in a struct".into()).span_note(old, "Previous occurrence given here".into()).emit();
                continue;
            }
            seen_this = Some(span);

            // Now set it
            this = Some(ThisField {
                name,
            });

        } else if attrs.flatten.is_some() {
            // Mark it as a flattened thing
            fields.push(DirectoryField {
                name,
                ty : field.ty,

                mode : FieldMode::Flatten,
            });

        } else {
            // Deduce a default path if not given
            let path: PathBuf =  attrs.path.map(|(p, _)| p).unwrap_or_else(|| {
                // See if the name ends in a particular suffix
                let name: String = name.to_string();
                let name_len: usize = name.len();
                for (suffix, ext) in &dir_attrs.exts {
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

                mode : FieldMode::Path(path),
            });
        }
    }

    // Change the this into initialization
    let this_init: Option<_> = this.as_ref().map(|t| {
        let ThisField { name } = t;
        quote! {
            #name : _base,
        }
    });
    let this_exists: Option<_> = this.map(|t| {
        let ThisField { name } = t;
        quote! {
            exists &= <PathBuf as ::directories::DirectoryExt>::exists(&self.#name);
        }
    });

    // Change the fields into field initializations
    let mut fields_init: Vec<_> = Vec::with_capacity(fields.len());
    let mut fields_exists: Vec<_> = Vec::with_capacity(fields.len());
    for field in fields {
        let DirectoryField { name, ty, mode } = field;

        // We can already deduce the exists
        fields_exists.push(quote! {
            exists &= <#ty as ::directories::DirectoryExt>::exists(&self.#name);
        });

        // Match on what to do for the instantiation
        match mode {
            FieldMode::Path(path) => {
                // Preprocess the path
                let spath: String = path.display().to_string();

                // Generate the instantiation, which may differ based on whether the path is absolute or not
                if path.is_absolute() {
                    fields_init.push(quote! {
                        #name : <#ty as ::directories::Directory>::try_init(#spath)?,
                    });
                } else {
                    fields_init.push(quote! {
                        #name : <#ty as ::directories::Directory>::try_init(_base.join(#spath))?,
                    });
                }
            },

            FieldMode::Flatten => {
                // Generate the instantiation, which just clones base
                fields_init.push(quote! {
                    #name : <#ty as ::directories::Directory>::try_init(_base.clone())?,
                });
            }
        }
    }

    // Get the generics of this struct for writing the new tokenstream
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Now return the impls we need
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::directories::Directory for #ident #ty_generics #where_clause {
            type Error = ::directories::Error;

            fn try_init(base: impl Into<::std::path::PathBuf>) -> Result<Self, Self::Error> {
                let _base: ::std::path::PathBuf = base.into();
                Ok(Self {
                    // Populate the normal fields
                    #(#fields_init)*
                    // If a 'this' exists, then populate it (which can consume the '_base' now)
                    #this_init
                })
            }
        }

        #[automatically_derived]
        impl #impl_generics ::directories::DirectoryExt for #ident #ty_generics #where_clause {
            fn exists(&self) -> bool {
                let mut exists: bool = true;
                #this_exists
                #(#fields_exists)*
                exists
            }
        }
    }.into())
}
