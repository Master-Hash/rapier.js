//! # Rapier
//! Fast and deterministic WASM physics engine.

#![allow(non_snake_case)] // JS uses camelCase, so we will follow its convention for the generated bindings.
                          // #![deny(missing_docs)]

extern crate nalgebra as na;
#[cfg(feature = "dim2")]
extern crate rapier2d as rapier;
#[cfg(feature = "dim3")]
extern crate rapier3d as rapier;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub mod dynamics;
pub mod geometry;
pub mod math;
pub mod pipeline;
pub mod utils;
