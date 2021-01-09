use crate::helpers::*;

pub trait EffectExtension {
    fn effect(self, effect: &impl Effect) -> Vec<f64>;
}
impl EffectExtension for Vec<f64> {
    fn effect(self, effect: &impl Effect) -> Vec<f64> {
        effect.run(self)
    }
}

pub trait Effect {
    fn run(&self, input: Vec<f64>) -> Vec<f64>;
}

impl Effect for Box<dyn Effect> {
    fn run(&self, input: Vec<f64>) -> Vec<f64> {
        (**self).run(input)
    }
}

pub struct EffectBundle(pub Vec<Box<dyn Effect>>);
impl Effect for EffectBundle {
    fn run(&self, mut input: Vec<f64>) -> Vec<f64> {
        for effect in &self.0 {
            input = input.effect(effect);
        }
        input
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn effect_bundle_works() {
        let effect_bundle = EffectBundle(vec![box Volume { mult: 0.5 }]);

        let res = vec![0., 1., 0., 1.].effect(&effect_bundle);
        assert_eq!(4, res.len());
    }

    #[test]
    fn effect_bundle_works_empty() {
        let effect_bundle = EffectBundle(vec![]);

        let res = vec![0., 1., 0., 1.].effect(&effect_bundle);
        assert_eq!(4, res.len());
    }
}
