//! Workout history parsing and validation
//!
//! This module handles PWF history documents (completed workout exports).

mod parser;
mod types;
mod validator;

pub use parser::parse;
pub use types::*;
pub use validator::{validate, ValidationResult};
