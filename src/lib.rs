/*************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-core/0.1.0")]
#![no_std]

//! # RusPiRo Core
//! Provide core traits and functions usefull accross the [RusPiRo crates](https://crates.io/search?q=ruspiro).
//! It also re-exports the rust ``core`` and ``alloc`` crates for more convinient usage in dependent
//! RusPiRo crates

/// use and re-export the rust alloc crate to enable easy access when this one is maintained as dependency
extern crate alloc;
/// re-export the rust core crate to enable all those core fetures used from ``ruspiro-core`` once,
/// this is maintained as dependency
pub use core::*;
/// re-export the console crate to ease access to the println!, info! etc. macros
pub use ruspiro_console::*;
/// re-export the pin utils crate
pub use pin_utils;
/// generic error trait definition (similar to the one of std::error::Error)
pub mod error;