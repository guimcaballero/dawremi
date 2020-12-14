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

mod flanger;
pub use flanger::Flanger;
mod bass_boost;
pub use bass_boost::BassBoost;
mod multitap_reverb;
pub use multitap_reverb::MultitapReverb;
