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

    fn frame(&mut self) -> Frame {
        let a_lfo = 0.001;
        let f_lfo = 5.0;

        // Make the base note disappear faster than the rest
        let attack = self.get_params().attack;
        let base_note_vol_multiplier = if self.sample > attack {
            attack as f64 / self.sample as f64
        } else {
            1.
        };

        let time = TAU * self.time();

        let mut result = base_note_vol_multiplier
            * (self.frequency * time + a_lfo * self.frequency * (f_lfo * time).sin()).sin();

        // Add higher notes
        let freq = self.frequency * 2.;
        result += 0.5 * (freq * time + a_lfo * freq * (f_lfo * time).sin()).sin();

        let freq = self.frequency * 2.;
        result += 0.125 * (freq * time + a_lfo * freq * (f_lfo * time).sin()).sin();

        let freq = self.frequency * 2.;
        if self.sample > attack {
            result += 0.0125 * (freq * time + a_lfo * freq * (f_lfo * time).sin()).sin();
        }

        Frame::mono(result)
    }
}
