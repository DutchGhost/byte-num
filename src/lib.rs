#![cfg_attr(feature = "nightly", feature(test))]

//! This crate provides function to convert from and into bytes, in base 10.
//! The functions are based on the fastware talks of Andrei Alexandrescu. (https://www.youtube.com/watch?v=o4-CwDo2zpg)
pub mod convert;

pub mod from_ascii;

pub mod into_ascii;

#[cfg(feature = "nightly")]
extern crate test;

#[cfg(test)]
pub mod tests;
