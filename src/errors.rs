#![allow(unused_imports)] // For rustdoc to work.

//! # Errors
//!
//! This module contains errors that may occur during the execution of the program.
//!
//! ```
//! use tuit::draw::TerminalDrawTarget;
//! use tuit::prelude::Terminal;
//! use tuit::terminal::ConstantSize;
//!
//! use tuit::errors::Error;
//!
//! # struct MagicGPU;
//!
//! # impl TerminalDrawTarget for MagicGPU {
//! #   fn render(&mut self, terminal: impl Terminal) -> tuit::Result<()> {
//! #        todo!()
//! #    }
//! # }
//!
//! let mut my_gpu = MagicGPU;
//!
//! let my_terminal: ConstantSize<20, 20> = ConstantSize::new();
//!
//! let my_render = my_gpu.render(my_terminal);
//!
//! match my_render {
//!    Ok(_) => { /* Happy path! */ }
//!    Err(Error::Generic(anyhow_error)) => { /* Depressing path :( */ }
//!    Err(_) => { /* Miserable path ;( */ }
//! }
//! ```

use thiserror::Error;


use crate::terminal::Terminal; // Unused in code, used in docs.
use crate::terminal::Widget; // Unused in code, used in docs.

/// This enum contains errors that may occur at runtime
///
/// ```
///
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// For generic I/O related errors.
    #[error("Encountered an I/O error.")]
    Io,
    /// This error is for when an implementor of TerminalDrawTarget fails for whatever reason
    #[error("Failed to render terminal screen.")]
    RenderError,
    /// This error is for when an out-of-bounds index occurs within any [`Terminal`]/[`Widget`] method.
    /// It does not include x,y co-ordinates, just the index into the character
    /// buffer slice.
    #[error("Attempted to access a character that was out of bounds at index {0}")]
    OutOfBoundsCharacter(usize),
    /// This error is for when an out-of-bounds index occurs within any [`Terminal`]/[`Widget`] method.
    /// It includes the x,y coordinates used to index into the slice.
    #[error("Attempted to access a character co-ordinate that was out of bounds at: (x: {0}, y: {0})")]
    OutOfBoundsCoordinate(usize, usize),
    /// For when an error case is not covered by Tuit. If you are forced to use this, and think
    /// that your use-case for the error is in fact general enough, please feel free to submit a PR!
    #[error(transparent)]
    Generic(#[from] anyhow::Error),
    /// A slightly more specific version of [`Error::Generic`] that specifies
    /// the error occurred within an object that was drawing to the terminal.
    #[error("{0}")]
    GenericDrawError(#[source] anyhow::Error),
    /// A slightly more specific version of [`Error::Generic`] that specifies
    /// the error occurred within an object that was updating.
    #[error("{0}")]
    GenericUpdateError(#[source] anyhow::Error),
    /// This error does not exist for use by [`tuit`], it exists for usage by the users of the library.
    ///
    /// It is better to return an [`Error::Todo`] than to panic using the `todo!()` macro when you use a widget
    /// that is not fully implemented.
    #[error("This area has not been implemented!")]
    Todo
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(_value: std::io::Error) -> Self {
        Self::Io
    }
}