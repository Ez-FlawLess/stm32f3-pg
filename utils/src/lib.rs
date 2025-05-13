#![cfg_attr(not(test), no_std)]

pub use paste::paste as utils_paste;

pub mod gpio;
pub mod vector_table;
pub mod register;
