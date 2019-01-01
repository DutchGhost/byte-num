#![cfg_attr(feature = "nightly", feature(test))]

//! This crate provides functions to convert from and into bytes, in base 10.
//! The functions are based on the fastware talks of Andrei Alexandrescu ([Talk](https://www.youtube.com/watch?v=o4-CwDo2zpg)).
//!
//! To convert from bytes, to integers, use the [`from_ascii`] module.
//!
//! To convert from integers, to bytes, use the [`into_ascii`] module.
mod constants;
pub mod error;
pub mod from_ascii;
pub mod into_ascii;
