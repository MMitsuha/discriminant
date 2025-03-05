//! # Discriminant
//!
//! Convert a enum (with or without field) with `#[repr(...)]` to corresponding type. Compatible with `#![no_std]`.
//!
//! ## Example
//!
//! ```
//! use discriminant::Discriminant;
//! #[derive(Discriminant)]
//! #[repr(i16)]
//! enum Test {
//!     One = 1,
//!     Two = 2,
//!     Four = 4,
//! }
//!
//! fn test() {
//!     assert_eq!(Test::One.discriminant(), 1);
//!     assert_eq!(Test::Two.discriminant(), 2);
//!     assert_eq!(Test::Four.discriminant(), 4);
//! }
//! ```

// Re-export macros
extern crate discriminant_derive;
pub use discriminant_derive::Discriminant;

/// Trait to constraint the type of macro
pub trait Discriminant<T>
where
    T: num_traits::ToPrimitive,
{
    fn discriminant(&self) -> T;
}
