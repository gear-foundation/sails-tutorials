#![no_std]

#[cfg(target_arch = "wasm32")]
pub use simple_ft::wasm::*;
