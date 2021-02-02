use super::*;

simple_instrument!(Bell);
impl SynthInstrument for Bell {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: self.seconds(0.01),
            decay: self.seconds(0.7),
            release: self.seconds(0.1),

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    fn note(&mut self) -> f64 {
        self.sample += 1;
        let a_lfo = 0.001;
        let f_lfo = 5.0;

        let base_note = self.note.up_an_octave();

        // Make the base note disappear faster than the rest
        let attack = self.get_params().attack;
        let base_note_vol_multiplier = if self.sample > attack {
            attack as f64 / self.sample as f64
        } else {
            1.
        };

        let freq: Frequency = base_note.into();
        let mut result = base_note_vol_multiplier
            * (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin();

        // Add higher notes
        let note = base_note.up_an_octave();
        let freq: Frequency = note.into();
        result += 0.5 * (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin();

        let note = note.up_an_octave();
        let freq: Frequency = note.into();
        result +=
            0.125 * (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin();

        let note = note.up_an_octave();
        let freq: Frequency = note.into();
        if self.sample > attack {
            result += 0.0125
                * (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin();
        }

        result
    }
}