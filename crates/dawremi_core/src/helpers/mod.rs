use dasp::signal;

mod extensions;
pub use extensions::*;

pub mod interpolation;

mod macros;
pub use macros::*;

pub mod delay_line;

pub fn silence() -> signal::Equilibrium<f64> {
    signal::equilibrium()
}
