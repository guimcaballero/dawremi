use dasp::signal;

mod extensions;
pub mod interpolation;
mod macros;
pub use extensions::*;
pub use macros::*;

pub fn silence() -> signal::Equilibrium<f64> {
    signal::equilibrium()
}
