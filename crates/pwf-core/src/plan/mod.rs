//! Plan parsing and validation
//!
//! This module handles PWF plan documents (workout templates).

mod parser;
pub mod resolver;
mod types;
mod validator;

pub use parser::parse;
pub use resolver::{resolve_exercise, ResolvedExercise};
pub use types::*;
pub use validator::{validate, ValidationResult};
