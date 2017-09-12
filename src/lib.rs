//! Terminal multiplexer

#![feature(plugin, custom_derive)]
#![plugin(mockers_macros)]

#[cfg(test)] extern crate mockers;

extern crate libc;
extern crate termion;
extern crate vte;

#[cfg(test)]
#[macro_use]
mod test_utils;

pub mod ansi;
pub mod pty;
pub mod tui;
pub mod util;