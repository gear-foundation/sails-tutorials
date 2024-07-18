#![no_std]

#[cfg(target_arch = "wasm32")]
pub use hello_world::wasm::*;
