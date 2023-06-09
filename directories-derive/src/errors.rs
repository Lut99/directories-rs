//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    23 Apr 2023, 10:57:00
//  Last edited:
//    23 Apr 2023, 11:16:00
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the errors that we use in the derivation crate.
// 

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};

use proc_macro_error::{Diagnostic, Level};
use syn::spanned::Spanned;


/***** AUXILLARY *****/
/// Additional [`Error`]-like trait that we can use for [`Diagnostic`] interoperability.
pub trait DiagnosticError: Debug + Display + Error {
    
}





/***** LIBRARY *****/
/// Defines the error type we return when deriving dictionaries.
pub struct DirectoryError<'s> {
    /// The span that relates this error to the source text.
    span : Box<dyn 's + Spanned>,
    /// The specific kind of error (containing more specific information).
    kind : DirectoryErrorKind,
}
impl<'s> DirectoryError<'s> {
    /// Constructor for the DirectoryError that conveniently wraps any [`Spanned`] type.
    /// 
    /// # Arguments
    /// - `span`: The [`syn::spanned::Spanned`]-type that we wrap to relate this error to the source text.
    /// - `kind`: The specific error kind that we want to emit.
    /// 
    /// # Returns
    /// A new DirectoryError instance.
    #[inline]
    pub fn new(span: impl 's + Spanned, kind: DirectoryErrorKind) -> Self {
        Self {
            span : Box::new(span),
            kind,
        }
    }

    /// Calls [`Diagnostic::abort()`] on the [`Diagnostic`] struct generated by ourselves.
    /// 
    /// Equivalent to calling `Diagnostic::from(self).abort()`.
    /// 
    /// # Returns
    /// Never, since [`Diagnostic::abort()`] returns the process.
    #[inline]
    pub fn abort(self) -> ! { Diagnostic::from(self).abort() }
}

impl<'s> Debug for DirectoryError<'s> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{:?}", self.kind)
    }
}
impl<'s> Display for DirectoryError<'s> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.kind)
    }
}
impl<'s> Error for DirectoryError<'s> {}

impl<'s> From<DirectoryError<'s>> for Diagnostic {
    #[inline]
    fn from(value: DirectoryError) -> Self { Diagnostic::spanned(value.span.span(), Level::Error, value.kind.to_string()) }
}



/// Defines the possible types of errors we may return when deriving dictionaries.
#[derive(Debug)]
pub enum DirectoryErrorKind {
    /// Attempted to derive the [`directories::Directory`] trait on a non-struct container.
    NotAStruct,
}
impl Display for DirectoryErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use DirectoryErrorKind::*;
        match self {
            NotAStruct => write!(f, "Cannot derive `Directory` on non-struct data types"),
        }
    }
}
