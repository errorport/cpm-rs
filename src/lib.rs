#![doc = include_str!("../README.md")]

#[macro_use]
extern crate log;

/// Module for CustomTask type.
pub mod customtask;
/// Module for Path type.
pub mod path;
/// Module for Scheduler type.
pub mod scheduler;

/// Input file parser.
pub mod input_parser;

mod tests;

pub use crate::customtask::CustomTask;
pub use crate::path::Path;
pub use crate::scheduler::Scheduler;

