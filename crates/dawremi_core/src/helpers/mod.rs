#![macro_use]

use dasp::signal;

mod extensions;
pub use extensions::*;

pub mod interpolation;

#[macro_use]
mod macros;
pub use macros::*;

pub fn silence() -> signal::Equilibrium<f64> {
    signal::equilibrium()
}
