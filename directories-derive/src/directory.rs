//  DIRECTORY.rs
//    by Lut99
// 
//  Created:
//    21 Apr 2023, 20:17:21
//  Last edited:
//    22 Apr 2023, 11:16:02
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the derive macro for the [`directories::Directory`]
//!   trait.
// 

use std::path::PathBuf;

use proc_macro_error::{Diagnostic, Level};
use syn::{Attribute, Expr, ExprLit, Field, Lit, Meta, Token};
use syn::__private::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;

use enum_debug::EnumDebug;

pub(crate) use crate::errors::DirError as Error;


/***** AUXILLARY STRUCTS *****/
/// Defines what we need to know about each field in a struct.
pub(crate) struct DirectoryField {
    /// The physical field we are referencing
    pub field : Field,
    /// The kind-specific other stuff
    pub kind  : DirectoryFieldKind,
}

/// Defines what we need to know about various types of struct fields.
#[derive(EnumDebug)]
pub(crate) enum DirectoryFieldKind {
    /// It's our own base path.
    This(NestedThis),
    /// It's a nested directory.
    Directory(NestedDirectory),
    /// It's a nested file.
    File(NestedFile),
}
impl DirectoryFieldKind {
    /// Returns if this DirectoryField is a `Self::This`.
    #[inline]
    pub fn is_this(&self) -> bool { matches!(self, Self::This(_)) }
    /// Provides access to this DirectoryField as if it is a `Self::This`.
    /// 
    /// # Returns
    /// A reference to the internal [`NestedThis`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::This`.
    #[inline]
    pub fn this(&self) -> &NestedThis { if let Self::This(this) = self { this } else { panic!("Cannot unwrap Self::{} as a Self::This", self.variant()); } }
    /// Provides mutable access to this DirectoryField as if it is a `Self::This`.
    /// 
    /// # Returns
    /// A mutable reference to the internal [`NestedThis`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::This`.
    #[inline]
    pub fn this_mut(&mut self) -> &mut NestedThis { if let Self::This(this) = self { this } else { panic!("Cannot unwrap Self::{} as a Self::This", self.variant()); } }
    /// Consumes this DirectoryField as if it is a `Self::This`.
    /// 
    /// # Returns
    /// The internal [`NestedThis`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::This`.
    #[inline]
    pub fn into_this(self) -> NestedThis { if let Self::This(this) = self { this } else { panic!("Cannot unwrap Self::{} as a Self::This", self.variant()); } }

    /// Returns if this DirectoryField is a [`NestedDirectory`].
    #[inline]
    pub fn is_dir(&self) -> bool { matches!(self, Self::Directory(_)) }
    /// Provides access to this DirectoryField as if it is a `Self::Directory`.
    /// 
    /// # Returns
    /// A reference to the internal [`NestedDirectory`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::Directory`.
    #[inline]
    pub fn dir(&self) -> &NestedDirectory { if let Self::Directory(dir) = self { dir } else { panic!("Cannot unwrap Self::{} as a Self::Directory", self.variant()); } }
    /// Provides mutable access to this DirectoryField as if it is a `Self::Directory`.
    /// 
    /// # Returns
    /// A mutable reference to the internal [`NestedDirectory`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::Directory`.
    #[inline]
    pub fn dir_mut(&mut self) -> &mut NestedDirectory { if let Self::Directory(dir) = self { dir } else { panic!("Cannot unwrap Self::{} as a Self::Directory", self.variant()); } }
    /// Consumes this DirectoryField as if it is a `Self::Directory`.
    /// 
    /// # Returns
    /// The internal [`NestedDirectory`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::Directory`.
    #[inline]
    pub fn into_dir(self) -> NestedDirectory { if let Self::Directory(dir) = self { dir } else { panic!("Cannot unwrap Self::{} as a Self::Directory", self.variant()); } }

    /// Returns if this DirectoryField is a [`NestedFile`].
    #[inline]
    pub fn is_file(&self) -> bool { matches!(self, Self::File(_)) }
    /// Provides access to this DirectoryField as if it is a `Self::File`.
    /// 
    /// # Returns
    /// A reference to the internal [`NestedFile`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::File`.
    #[inline]
    pub fn file(&self) -> &NestedFile { if let Self::File(file) = self { file } else { panic!("Cannot unwrap Self::{} as a Self::File", self.variant()); } }
    /// Provides mutable access to this DirectoryField as if it is a `Self::File`.
    /// 
    /// # Returns
    /// A mutable reference to the internal [`NestedFile`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::File`.
    #[inline]
    pub fn file_mut(&mut self) -> &mut NestedFile { if let Self::File(file) = self { file } else { panic!("Cannot unwrap Self::{} as a Self::File", self.variant()); } }
    /// Consumes this DirectoryField as if it is a `Self::File`.
    /// 
    /// # Returns
    /// The internal [`NestedFile`] struct.
    /// 
    /// # Panics
    /// This function panics if we are not a `Self::File`.
    #[inline]
    pub fn into_file(self) -> NestedFile { if let Self::File(file) = self { file } else { panic!("Cannot unwrap Self::{} as a Self::File", self.variant()); } }
}

/// Defines what metadata we like to know about a nested this in a Directory struct.
pub(crate) struct NestedThis {
    /// The name of the variable.
    pub name : String,
}

/// Defines what metadata we like to know about a nested directory in a Directory struct.
pub(crate) struct NestedDirectory {
    /// Defines the name of the directory.
    pub name : String,
}

/// Defines what metadata we like to know about a nested file in a Directory struct.
pub(crate) struct NestedFile {}





/***** LIBRARY *****/
/// Parses the attribute(s) of a field in a directory structs.
/// 
/// # Arguments
/// - `field`: The field itself, which we may use for analysis.
/// - `attrs`: The list of attributes to parse.
/// 
/// # Returns
/// A [`DirectoryFieldKind`] struct with the information we parsed.
/// 
/// # Errors
/// This function may error if we failed to parse the attributes for some reason.
pub(crate) fn parse_entry_attributes(field: &Field) -> Result<DirectoryFieldKind, Diagnostic> {
    for a in &field.attrs {
        match &a.meta {
            Meta::List(l) => {
                // Match on which of our attributes we are parsing
                if l.path.is_ident("this") {
                    // Assert the field has an identifier
                    if field.ident.is_none() { return Err(Error::ThisWithoutIdentifier{ span: l.span() }.into()); }

                    // It's our own thing, so quickly return this.
                    return Ok(DirectoryFieldKind::This(NestedThis {
                        name : field.ident.as_ref().map(|i| i.to_string()).unwrap_or_else(String::new),
                    }));

                } else if l.path.is_ident("dir") {
                    let mut name: Option<(String, Span)> = None;

                    // Parse whatever the user wrote in there
                    let nested = match l.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                        Ok(nested) => nested,
                        Err(err)   => { return Err(Error::IllegalAttributeSyntax { _span: l.tokens.span(), path: l.path.get_ident().unwrap().to_string(), err }.into()); },
                    };

                    // Now match what they wrote
                    for nested_a in nested {
                        match nested_a {
                            Meta::List(l) => {
                                return Err(Error::UnknownDirAttribute{ span: l.span() }.into());
                            },

                            Meta::NameValue(nv) => {
                                // Match on the key first
                                if nv.path.is_ident("path") {
                                    // Attempt to read the expression as a string literal
                                    if let Expr::Lit(ExprLit{ lit: Lit::Str(s), .. }) = &nv.value {
                                        // Emit a warning if we have already set it before
                                        if let Some((_, span)) = &name {
                                            Diagnostic::spanned(nv.span(), Level::Warning, "The `path` attribute is set twice".into())
                                                .span_note(*span, "Previous assignment here".into())
                                                .emit();
                                        }

                                        // Set it
                                        name = Some((s.value(), nv.span()));
                                    } else {
                                        return Err(Error::IllegalDirPathValue{ span: nv.value.span() }.into());
                                    }
                                } else {
                                    return Err(Error::UnknownDirNameValue{ span: nv.path.span(), path: nv.path.get_ident().map(|i| i.to_string()) }.into());
                                }
                            },
    
                            Meta::Path(p) => {
                                // Match on the identifier used
                                if p.is_ident("any") {
                                    /* TODO */
                                } else if p.is_ident("optional") {
                                    /* TODO */
                                } else {
                                    return Err(Error::UnknownDirAttribute{ span: p.span() }.into());
                                }
                            },
                        }
                    }

                    // Resolve the name to a default name if not set already
                    let name: String = match name {
                        Some((name, _)) => name,
                        None            => "".into(),
                    };
                
                    // OK, return the attributes
                    return Ok(DirectoryFieldKind::Directory(NestedDirectory {
                        name,
                    }));

                } /* Ignore otherwise */
            },

            Meta::NameValue(nv) => {

            },

            Meta::Path(p) => {

            },
        }
    }

    // Otherwise we can't know what type this field is
    Err(Error::UntypedField { span: field.span(), name : field.ident.as_ref().map(|i| i.to_string()).unwrap_or("<anonymous>".into()) }.into())
}
