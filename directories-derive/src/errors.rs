//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    22 Apr 2023, 10:02:20
//  Last edited:
//    22 Apr 2023, 11:17:11
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines errors used in the proc macro's.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use proc_macro_error::{Diagnostic, Level};
use syn::__private::Span;


/***** LIBRARY *****/
/// Defines errors that occur in the [`crate::Directory`] procedural macro.
#[derive(Debug)]
pub(crate) enum DirError {
    /// A field was missing an attribute to denote what it is.
    UntypedField{ span: Span, name: String },

    /// Got a `#[this]` thing without an identifier.
    ThisWithoutIdentifier{ span: Span },

    /// Failed to parse the internal meta of an attribute, as attributes.
    IllegalAttributeSyntax{ _span: Span, path: String, err: syn::parse::Error },
    /// Unknown attribute was given to the `dir` attribute.
    UnknownDirAttribute{ span: Span },
    /// A given `name = value` pair was not known to the `dir` attribute.
    UnknownDirNameValue{ span: Span, path: Option<String> },
    /// The `path = ...` attribute was not given a string literal
    IllegalDirPathValue{ span: Span },
}
impl Display for DirError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use DirError::*;
        match self {
            UntypedField{ name, .. } => write!(f, "Field `{name}` as no `#[this]`, `#[dir]` or `#[file]` attribute set"),

            ThisWithoutIdentifier{ .. } => write!(f, "Field marked as `#[this]`, but no identifier found"),

            IllegalAttributeSyntax{ path, .. } => write!(f, "Illegal syntax for arguments to attribute `{path}`"),
            UnknownDirAttribute{ .. }          => write!(f, "Unknown attribute for `dir`"),
            UnknownDirNameValue{ path, .. }    => write!(f, "Unknown option{} for attribute `dir`", if let Some(path) = path { format!(" `{path}`") } else { String::new() }),
            IllegalDirPathValue{ .. }          => write!(f, "Expected string literal for attribute `path`"),
        }
    }
}
impl Error for DirError {}
impl From<DirError> for Diagnostic {
    fn from(value: DirError) -> Self {
        use DirError::*;
        match &value {
            UntypedField{ span, .. } => Diagnostic::spanned(*span, Level::Error, value.to_string()),

            ThisWithoutIdentifier{ span, .. } => Diagnostic::spanned(*span, Level::Error, value.to_string()),

            IllegalAttributeSyntax{ err, .. } => Diagnostic::spanned(err.span(), Level::Error, err.to_string()),
            UnknownDirAttribute{ span, .. }   => Diagnostic::spanned(*span, Level::Error, value.to_string()),
            UnknownDirNameValue{ span, .. }   => Diagnostic::spanned(*span, Level::Error, value.to_string()),
            IllegalDirPathValue{ span, .. }   => Diagnostic::spanned(*span, Level::Error, value.to_string()),
        }
    }
}
