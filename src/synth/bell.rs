use super::*;

instrument!(Bell,);
impl SynthInstrument for Bell {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: 0.01,
            decay: 0.3,
            release: 0.,

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    fn note(&mut self) -> f64 {
        let freq: Frequency = self.note.into();
        self.sample += 1;
        let a_lfo = 0.001;
        let f_lfo = 5.0;

        let square_1 =
            (freq.0 * 2. * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin();
        let square_2 = (freq.0 * 3. * self.time()).sin();
        let square_3 = (freq.0 * 4. * self.time()).sin();

        1. * square_1 + 0.5 * square_2 + 0.25 * square_3
    }
}
