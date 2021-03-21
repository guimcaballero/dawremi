use crate::frame::*;
use crate::helpers::*;

pub trait EffectExtension {
    fn effect(self, effect: &impl Effect) -> Vec<Frame>;
}
impl EffectExtension for Vec<Frame> {
    fn effect(self, effect: &impl Effect) -> Vec<Frame> {
        effect.run(self)
    }
}

pub trait Effect {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame>;
}

impl Effect for Box<dyn Effect> {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        (**self).run(input)
    }
}

pub struct EffectBundle(pub Vec<Box<dyn Effect>>);
impl Effect for EffectBundle {
    fn run(&self, mut input: Vec<Frame>) -> Vec<Frame> {
        for effect in &self.0 {
            input = input.effect(effect);
        }
        input
    }
}

#[derive(Debug, Clone)]
pub enum Automation<T> {
    Const(T),
    Vec(Vec<T>),
}
impl<T: Default + Clone> Automation<T> {
    pub fn value(&self, idx: usize) -> T {
        match self {
            Self::Const(val) => val.clone(),
            Self::Vec(vec) => {
                if let Some(val) = vec.get(idx) {
                    val.clone()
                } else {
                    T::default()
                }
            }
        }
    }
}

mod volume;
pub use volume::Volume;
mod flanger;
pub use flanger::Flanger;
mod bass_boost;
pub use bass_boost::BassBoost;
mod multitap_reverb;
pub use multitap_reverb::MultitapReverb;
mod slow_convolution;
pub use slow_convolution::SlowConvolution;
mod fft_convolution;
pub use fft_convolution::Convolution;
mod pitch_shift;
pub use pitch_shift::PitchShift;
mod autotune;
pub use autotune::Autotune;
mod filter;
pub use filter::{Filter, FilterMode};
mod delay;
pub use delay::Delay;
mod stretch;
pub use stretch::Stretch;
mod balance;
pub use balance::Balance;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn effect_bundle_works() {
        let effect_bundle = EffectBundle(vec![Box::new(Volume {
            mult: Automation::Const(0.5),
        })]);

        let res = vec![0., 1., 0., 1.].into_frames().effect(&effect_bundle);
        assert_eq!(4, res.len());
    }

    #[test]
    fn effect_bundle_works_empty() {
        let effect_bundle = EffectBundle(vec![]);

        let res = vec![0., 1., 0., 1.].into_frames().effect(&effect_bundle);
        assert_eq!(4, res.len());
    }
}
