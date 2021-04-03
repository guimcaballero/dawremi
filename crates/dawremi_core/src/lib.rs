#[macro_use]
pub mod helpers;

pub mod effects;
pub mod frame;
pub mod notes;
pub mod player;
pub mod record;
pub mod select;
pub mod signals;
pub mod song;
pub mod sound_files;
pub mod synth;

pub mod prelude {
    //! The prelude exports almost everything, so it can be imported easily

    pub use std::collections::HashMap;

    pub use super::effects::*;
    pub use super::frame::*;
    pub use super::helpers::*;
    pub use super::notes::{n_tet::*, *};
    pub use super::signals::*;
    pub use super::song::*;
    pub use super::sound_files::{enums::*, io::*, Sound};
    pub use super::synth::*;
}
