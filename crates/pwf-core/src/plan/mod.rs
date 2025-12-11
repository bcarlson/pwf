//! Plan parsing and validation
//!
//! This module handles PWF plan documents (workout templates).

mod parser;
mod types;
mod validator;

pub use parser::parse;
pub use types::*;
pub use validator::{validate, ValidationResult};
