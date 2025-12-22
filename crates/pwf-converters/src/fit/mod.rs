//! FIT file format converter

pub mod mappings;
pub mod parser;
pub mod types;

// Re-export main conversion function
pub use parser::fit_to_pwf;
