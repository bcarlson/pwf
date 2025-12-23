//! TCX (Training Center XML) format converter

pub mod exporter;
pub mod mappings;
pub mod parser;

// Re-export main conversion functions
pub use exporter::pwf_to_tcx;
pub use parser::tcx_to_pwf;
