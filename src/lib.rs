#![cfg_attr(feature="nightly", feature(test))]

pub mod convert;

#[cfg(feature="nightly")]
extern crate test;

#[cfg(test)]
mod tests;