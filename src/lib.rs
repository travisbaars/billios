//! billios - A library for soil calculations
//!
//! This crate is a collection of soil related data structures and formulae.
//!
//! ## Installation
//!
//! Standard functionality, no aditional dependencies are requeried:
//!
//! ```toml
//! [dependencies]
//! billios = "0.1.0"
//! ```
//!
//! Alternatively, this crate can be installed using the `cargo add` command:
//!
//! ```bash
//! cargo add billios
//! ```
//!
//! ## Code Examples
//!
//! ### Example - Sand Used
//!
//! Using this formula only requeres a couple lines of code.
//!
//! Simply call the `new()` method with the desired values, then call `calculate()` to retrieve the result.
//!
//! #### Use with constant `sand_in_cone` values:
//!
//! ```
//! use billios::math::calculations::*;
//!
//! let sand_used = SandUsed::new(14.65, 8.75, None);
//! let result = sand_used.calculate();
//!
//! assert_eq!(2.31, result);
//! ```

pub mod domain;

pub mod math;