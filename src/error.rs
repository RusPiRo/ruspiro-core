/*************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **************************************************************************************************/
//! # Basic Error trait
//! This is more or less the same as found in Rust std library: [Error](https://doc.rust-lang.org/std/error/trait.Error.html)
//! but made available in ``no_std`` environment where an allocator is in place, which is the case for
//! the RusPiRo family.

extern crate alloc;
use alloc::boxed::Box;
use core::fmt::{Debug, Display};

/// The type that shall be used as ``Error`` type when returning a [``Result``]. This allows to use
/// one return type while the function might use different error types when using the ``?`` operator
/// and mapping all error types to a generic one would be quite cumbersome
pub type BoxError = Box<dyn Error + Send>; // + Send + Sync needed ?

/// The generic Error trait. All actual errors implementing this trait also need to implement ``Debug``
/// and ``Display`` to provide human readable text of the error.
pub trait Error: Debug + Display + Send {
    /// the underlaying source of this error, if any. This allows to "stack" errors while keeping
    /// track to it's root cause
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}

impl<'a, E: Error + 'a + Send> From<E> for Box<dyn Error + 'a + Send> {
    /// Conviniently convert an [``Error``] into a boxed ``dyn Error``. This allows simple usage of
    /// ``.into()`` calls when reterning an ``Error`` type.
    fn from(orig: E) -> Box<dyn Error + 'a + Send> {
        Box::new(orig)
    }
}
