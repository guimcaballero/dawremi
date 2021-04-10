//! Contains various effects to modify a list of samples

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
pub enum Automation<T: Default + Clone> {
    /// Always returns the same value
    Const(T),
    /// Will return each value of the vec in order, and after that will return T::default()
    Vec(Vec<T>),
    /// Will return each value of the vec in order, looping back to the beginning once it's done
    Loop(Vec<T>),
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
            Self::Loop(vec) => vec[idx % vec.len()].clone(),
        }
    }
}

impl<T: Default + Clone> Default for Automation<T> {
    fn default() -> Self {
        Automation::Const(T::default())
    }
}

impl<T: Default + Clone> IntoIterator for Automation<T> {
    type Item = T;
    type IntoIter = AutomationIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        AutomationIter(self, 0)
    }
}

/// Infinite Iterator for the provided automation
pub struct AutomationIter<T: Default + Clone>(Automation<T>, usize);
impl<T: Default + Clone> Iterator for AutomationIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0.value(self.1);
        self.1 += 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
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
pub use autotune::{Autotune, AutotuneConfig};
mod filter;
pub use filter::{Filter, FilterMode};
mod delay;
pub use delay::Delay;
mod stretch;
pub use stretch::Stretch;
mod balance;
pub use balance::Balance;
mod bit_crusher;
pub use bit_crusher::{BitCrusher, BitCrusherMode};

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

    #[test]
    fn const_automation() {
        let a = Automation::Const(4.);

        let vec: Vec<f64> = (0..10).map(|idx| a.value(idx)).collect();
        assert_eq!(vec![4.; 10], vec);
    }

    #[test]
    fn vec_automation() {
        let a = Automation::Vec(vec![0., 1., 2., 3., 4.]);

        let vec: Vec<f64> = (0..10).map(|idx| a.value(idx)).collect();
        assert_eq!(vec![0., 1., 2., 3., 4., 0., 0., 0., 0., 0.], vec);
    }

    #[test]
    fn loop_automation() {
        let a = Automation::Loop(vec![0., 1., 2., 3., 4.]);

        let vec: Vec<f64> = (0..10).map(|idx| a.value(idx)).collect();
        assert_eq!(vec![0., 1., 2., 3., 4., 0., 1., 2., 3., 4.,], vec);
    }

    #[test]
    fn automation_into_iter() {
        let a = Automation::Loop(vec![0.0f64, 1., 2., 3., 4.]);
        let iter = a.into_iter();

        let vec: Vec<f64> = iter.take(20).collect();
        assert_eq!(20, vec.len());
    }

    #[test]
    fn automation_for_loop() {
        let a = Automation::Loop(vec![0.0f64, 1., 2., 3., 4.]);

        let mut i = 0.;
        for val in a {
            i += val;

            if i > 10. {
                break;
            }
        }

        assert!(i > 10.);
    }
}
