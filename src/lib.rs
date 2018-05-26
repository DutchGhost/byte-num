#![cfg_attr(feature="nightly", feature(test))]
#![cfg_attr(feature="simd", feature(stdsimd))]

//! This crates offers functions to convert from (unsigned for now) integers to bytes, and from bytes to integers.
//! The goal of this crate is to do it's operations fast. This also means that weird corner cases, are not really handled.
//! If such a corner case can occur, this is noted in the # Safety section, and shows the corner case.

pub mod convert;

#[cfg(feature="nightly")]
extern crate test;

#[cfg(feature="simd")]
pub mod convert_simd;

#[cfg(test)]
mod tests;