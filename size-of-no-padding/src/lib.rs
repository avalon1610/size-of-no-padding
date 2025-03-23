//! # Size Of No Padding
//! This crate provides two derive proc-macro.
//!
//! ## `SizeOfNoPadding`
//! > Only support struct can be marked as `Copy`
//!
//! Create a shadow struct which as same as original struct except it's marked `#[repr(packed)]`. call `size_of_no_padding()` method on struct will invoke `std::mem::size_of()` function on shadow one in compile time.
//!
//! ### Example
//! ```rust
//! use size_of_no_padding::SizeOfNoPadding;
//! use size_of_no_padding::SizeOf;
//!
//! #[derive(SizeOfNoPadding)]
//! struct Abc {
//!     a: u8,
//!     b: u32,
//!     c: u8,
//! }
//!
//! assert_eq!(8, std::mem::size_of::<Abc>());
//! assert_eq!(6, Abc::size_of_no_padding());
//!
//! ```
//!
//! ### `SizeOfNoPaddingAny`
//! > Support struct whose non-Copy fields impl [`SizeOfAny`] trait
//!
//! Calculate the size for every field, add up all the size. call `size_of_no_padding(&self)` will give the size of whole struct without padding in runtime.
//!
//! ### Example
//! ```rust
//! use size_of_no_padding::SizeOfNoPaddingAny;
//! use size_of_no_padding::SizeOfAny;
//!
//! #[derive(SizeOfNoPaddingAny)]
//! struct Abc2 {
//!     a: u8,
//!     b: u32,
//!     c: u8,
//!     d: Vec<u16>,
//!     e: Abc3,
//! }
//!
//! #[derive(SizeOfNoPaddingAny)]
//! struct Abc3([u32; 4], u16);
//!
//! let abc = Abc2 {
//!     a: 1,
//!     b: 2,
//!     c: 3,
//!     d: vec![4, 5, 6],
//!     e: Abc3([7, 8, 9, 10], 11),
//! };
//!
//! assert_eq!(std::mem::size_of::<Abc2>(), 56);
//! assert_eq!(abc.size_of_no_padding_any(), 30);
//! ```
//!
//! ## License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
pub use size_of_no_padding_derive::{SizeOfNoPadding, SizeOfNoPaddingAny};
extern crate self as size_of_no_padding;

/// A trait for types that can calculate their size without padding.
/// Struct can derive this trait by using `#[derive(SizeOfNoPaddingAny)]`
pub trait SizeOfAny {
    fn size_of_no_padding_any(&self) -> usize;
}

/// A trait for types that can calculate their size without padding.
/// Struct can derive this trait by using `#[derive(SizeOfNoPadding)]`
pub trait SizeOf {
    fn size_of_no_padding() -> usize;
}

/// A trait for Primitive types.
/// This trait is used to implement [`SizeOfAny`] for primitive types.
pub trait Primitive {}

impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for usize {}
impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for isize {}
impl Primitive for f32 {}
impl Primitive for f64 {}
impl Primitive for bool {}
impl Primitive for char {}
impl Primitive for () {}

impl<T: SizeOfAny> SizeOfAny for [T] {
    fn size_of_no_padding_any(&self) -> usize {
        self.iter().map(|item| item.size_of_no_padding_any()).sum()
    }
}

impl<T: Primitive> SizeOfAny for T {
    fn size_of_no_padding_any(&self) -> usize {
        std::mem::size_of::<T>()
    }
}

#[cfg(test)]
mod test {
    use crate::{Primitive, SizeOf, SizeOfAny};
    use size_of_no_padding_derive::{SizeOfNoPadding, SizeOfNoPaddingAny};

    #[derive(SizeOfNoPadding)]
    #[allow(dead_code)]
    struct Abc<T> {
        a: u8,
        b: u32,
        c: u8,
        d: T,
    }

    #[test]
    fn test_size_of_abc() {
        let size = std::mem::size_of::<Abc<u32>>();
        assert_eq!(size, 12);
        let size_no_padding = Abc::<u32>::size_of_no_padding();
        assert_eq!(size_no_padding, 10);
    }

    #[derive(SizeOfNoPaddingAny)]
    struct Abc2<const T: usize, K: Primitive> {
        a: K,
        b: u32,
        c: [u8; T],
        d: Vec<Abc3>,
        e: Abc3,
    }

    #[derive(SizeOfNoPaddingAny, Clone, Copy)]
    struct Abc3([u32; 4], u16);

    #[test]
    fn test_size_of_abc2() {
        assert_eq!(std::mem::size_of::<Abc3>(), 20);

        let abc3 = Abc3([7, 8, 9, 10], 11);
        let size = abc3.size_of_no_padding_any();
        assert_eq!(size, 18);

        let abc2 = Abc2 {
            a: 1u8,
            b: 2,
            c: [3, 4, 5, 6],
            d: vec![abc3, abc3],
            e: abc3,
        };

        let size = abc2.size_of_no_padding_any();
        assert_eq!(size, 63);
    }
}
