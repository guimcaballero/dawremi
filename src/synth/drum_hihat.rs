use super::*;

simple_instrument!(DrumHiHat);
impl SynthInstrument for DrumHiHat {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: self.seconds(0.01),
            decay: self.seconds(0.05),
            release: 0,

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
            .expect("Note passed to Drum Kick should be able to be decreased by 3 octaves");
        let freq: Frequency = base_note.into();

        let square =
            if (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin() > 0. {
                1.
            } else {
                -1.
            };

        0.1 * square + 0.9 * rand::thread_rng().gen_range(-1., 1.)
    }
}
