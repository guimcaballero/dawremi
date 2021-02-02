use super::*;

simple_instrument!(Harmonica);
impl SynthInstrument for Harmonica {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: self.seconds(0.1),
            decay: self.seconds(0.1),
            release: self.seconds(0.2),

            attack_amplitude: 1.0,
            sustain_amplitude: 0.8,
        }
    }

    fn note(&mut self) -> f64 {
        let freq: Frequency = self.note.into();
        self.sample += 1;
        let a_lfo = 0.001;
        let f_lfo = 7.0;

        let square_1 =
            if (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin() > 0. {
                1.
            } else {
                -1.
            };
        let square_2 = if (freq.0 * 1.5 * self.time()).sin() > 0. {
            1.
        } else {
            -1.
        };
        let square_3 = if (freq.0 * 2.0 * self.time()).sin() > 0. {
            1.
        } else {
            -1.
        };

        0.02 * square_1
            + 0.5 * square_2
            + 0.15 * square_3
            + 0.01 * rand::thread_rng().gen_range(-1., 1.)
    }
}
