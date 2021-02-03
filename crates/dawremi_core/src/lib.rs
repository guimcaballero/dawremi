#![feature(box_syntax)]
#![feature(arbitrary_enum_discriminant)]

pub mod effects;
pub mod helpers;
pub mod looper;
pub mod notes;
pub mod player;
pub mod record;
pub mod song;
pub mod sound_files;
pub mod synth;

pub mod prelude {
    pub use std::collections::HashMap;

    pub use super::effects::*;
    pub use super::helpers::*;
    pub use super::looper::*;
    pub use super::notes::*;
    pub use super::song::*;
    pub use super::sound_files::{enums::*, io::*};
    pub use super::synth::*;
}
