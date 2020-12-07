use super::*;

instrument!(DrumSnare,);
impl SynthInstrument for DrumSnare {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: 0.,
            decay: 0.2,
            release: 0.,

            attack_amplitude: 0.5,
            sustain_amplitude: 0.,
        }
    }

    fn note(&mut self) -> f64 {
        self.sample += 1;
        let a_lfo = 1.;
        let f_lfo = 0.5;

        let base_note = self
            .note
            .down_an_octave()
            .expect("Note passed to Drum Kick should be able to be decreased by 3 octaves")
            .down_an_octave()
            .expect("Note passed to Drum Kick should be able to be decreased by 3 octaves");
        let freq: Frequency = base_note.into();

        0.3 * (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin()
            + 0.7 * rand::thread_rng().gen_range(-1., 1.)
    }
}
