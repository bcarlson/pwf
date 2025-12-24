//! WebAssembly bindings for PWF (Portable Workout Format)
//!
//! This crate provides WASM bindings for the PWF validation and conversion functionality,
//! allowing PWF to be used in web browsers without a backend server.

mod conversion;
mod validation;
mod utils;

pub use conversion::*;
pub use validation::*;
pub use utils::*;

use wasm_bindgen::prelude::*;

/// Initialize the WASM module.
///
/// This function sets up panic hooks for better error messages in the browser console
/// and configures the global allocator for smaller binary size.
#[wasm_bindgen(start)]
pub fn init() {
    // Set up better panic messages in the browser console
    console_error_panic_hook::set_once();
}

/// Use wee_alloc as the global allocator for smaller binary size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
