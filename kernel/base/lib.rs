// TODO Copyright Header

//! # The Reenix base util stuff.

#![crate_name="base"]
#![crate_type="rlib"]
#![no_std]
#![allow(missing_doc)]
#![feature(asm, macro_rules, globs, concat_idents,lang_items, trace_macros, phase)]


#[phase(plugin, link)] extern crate core;
extern crate libc;

//pub use errno::*;

// NOTE Needs to go first so everything else can get the macro's defined in it.
mod bitflags;
mod macros;
mod gdb;

pub mod errno;

pub mod io;
pub mod debug;
pub mod kernel;
pub mod from_str;

pub mod describe {
    use core::fmt;
    use core::prelude::*;
    pub trait Describeable {
        fn describe(&self, &mut fmt::Formatter) -> fmt::Result;
    }
    pub struct Describer<T: Describeable>(T);
    impl<T: Describeable> Describeable for Describer<T> {
        fn describe(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let &Describer(ref x) = self;
            try!(write!(f, "Describe("));
            try!(x.describe(f));
            write!(f, ")")
        }
    }
    impl<T: Describeable> fmt::Show for Describer<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let &Describer(ref x) = self;
            x.describe(f)
        }
    }
}

pub fn init_stage1() { debug::setup(); }
pub fn init_stage2() {}

// NOTE Needed for the #[deriving] stuff to work. Because that makes sense.
mod std {
    pub use core::cmp;
    pub use core::fmt;
    pub use core::option;
    pub use core::num;
}
// This lets us use the macro's exported from here locally.
mod base {
    pub use super::*;
}
