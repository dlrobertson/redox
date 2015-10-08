//! # The Redox Library
//!
//! The Redox Library contains a collection of commonly used low-level software
//! constructs to be used on top of the base operating system, including graphics 
//! support and windowing, a basic filesystem, audio support, a simple console
//! with shell style functions, an event system, and environment argument support.

#![crate_name="redox"]
#![crate_type="rlib"]
#![feature(alloc)]
#![feature(asm)]
#![feature(box_syntax)]
#![feature(collections)]
#![feature(core_slice_ext)]
#![feature(core_str_ext)]
#![feature(lang_items)]
#![feature(vec_push_all)]
#![feature(no_std)]
#![no_std]

// Yep I'm evil (this is a good idea!)
#![warn(missing_docs)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate collections;

pub use alloc::boxed::Box;

pub use collections::*;
pub use collections::string::ToString;

pub use common::random::*;
pub use common::time::*;

pub use externs::*;

pub use syscall::call::*;

pub use audio::wav::*;
pub use console::*;
pub use env::*;
pub use event::*;
pub use fs::file::*;
pub use graphics::bmp::*;
pub use orbital::*;

/// A module for common functionalities.
/// Primary functionality provided by std.
#[path="../../src/common/src/"]
mod common {
    pub mod debug;
    pub mod random;
    pub mod time;
}

/// A module for necessary C and assembly constructs
#[path="../../src/externs.rs"]
pub mod externs;

/// A module for system calls
#[path="../../src/syscall/src"]
mod syscall {
    /// Calls
    pub mod call;
    /// Common
    pub mod common;
}

/// A module for audio
mod audio {
    pub mod wav;
}

/// A module for console functionality
#[macro_use]
pub mod console;
/// A module for commands and enviroment
pub mod env;
/// A module for events
pub mod event;
/// A module for the filesystem
#[path="fs/lib.rs"]
mod fs;
/// Graphics support
mod graphics {
    pub mod bmp;
}
/// A module for window support
pub mod orbital;

/// A module for shell based functions
pub mod ion;

/* Extensions for String { */
/// Parse the string to a integer using a given radix
pub trait ToNum {
    fn to_num_radix(&self, radix: usize) -> usize;
    fn to_num_radix_signed(&self, radix: usize) -> isize;
    fn to_num(&self) -> usize;
    fn to_num_signed(&self) -> isize;
}

impl ToNum for String {
    fn to_num_radix(&self, radix: usize) -> usize {
        if radix == 0 {
            return 0;
        }

        let mut num = 0;
        for c in self.chars() {
            let digit;
            if c >= '0' && c <= '9' {
                digit = c as usize - '0' as usize
            } else if c >= 'A' && c <= 'Z' {
                digit = c as usize - 'A' as usize + 10
            } else if c >= 'a' && c <= 'z' {
                digit = c as usize - 'a' as usize + 10
            } else {
                break;
            }

            if digit >= radix {
                break;
            }

            num *= radix;
            num += digit;
        }

        num
    }

    /// Parse the string as a signed integer using a given radix
    fn to_num_radix_signed(&self, radix: usize) -> isize {
        if self.starts_with('-') {
            -(self[1 .. self.len()].to_string().to_num_radix(radix) as isize)
        } else {
            self.to_num_radix(radix) as isize
        }
    }

    /// Parse it as a unsigned integer in base 10
    fn to_num(&self) -> usize {
        self.to_num_radix(10)
    }

    /// Parse it as a signed integer in base 10
    fn to_num_signed(&self) -> isize {
        self.to_num_radix_signed(10)
    }
}
/* } Extensions for String */
