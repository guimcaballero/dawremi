use super::*;

simple_instrument!(DrumKick);
impl SynthInstrument for DrumKick {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: self.seconds(0.01),
            decay: self.seconds(0.15),
            release: 0,

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    fn note(&mut self) -> f64 {
        self.sample += 1;
        let a_lfo = 1.;
        let f_lfo = 1.;

        let freq: Frequency = self.note.into();

        0.99 * (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin()
            + 0.01 * rand::thread_rng().gen_range(-1., 1.)
    }
}