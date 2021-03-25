use super::*;

simple_instrument!(DrumSnare);
impl SynthInstrument for DrumSnare {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: 0,
            decay: self.seconds(0.2),
            release: 0,

            attack_amplitude: 0.5,
            sustain_amplitude: 0.,
        }
    }

    fn frame(&mut self) -> Frame {
        let a_lfo = 1.;
        let f_lfo = 0.5;

        let result = 0.5
            * (self.frequency.0 * self.time()
                + a_lfo * self.frequency.0 * (f_lfo * self.time()).sin())
            .sin()
            + 0.5 * rand::thread_rng().gen_range(-0.8, 0.8);

        Frame::mono(result)
    }
}
