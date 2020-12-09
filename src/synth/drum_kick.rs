use super::*;

simple_instrument!(DrumKick);
impl SynthInstrument for DrumKick {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: 0.01,
            decay: 0.15,
            release: 0.,

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    fn note(&mut self) -> f64 {
        self.sample += 1;
        let a_lfo = 1.;
        let f_lfo = 1.;

        let base_note = self
            .note
            .down_an_octave()
            .expect("Note passed to Drum Kick should be able to be decreased by 3 octaves")
            .down_an_octave()
            .expect("Note passed to Drum Kick should be able to be decreased by 3 octaves")
            .down_an_octave()
            .expect("Note passed to Drum Kick should be able to be decreased by 3 octaves");
        let freq: Frequency = base_note.into();

        0.99 * (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin()
            + 0.01 * rand::thread_rng().gen_range(-1., 1.)
    }
}
