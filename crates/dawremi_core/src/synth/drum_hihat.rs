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

    fn frame(&mut self) -> Frame {
        let a_lfo = 1.;
        let f_lfo = 1.;

        let square = if (self.frequency.0 * self.time()
            + a_lfo * self.frequency.0 * (f_lfo * self.time()).sin())
        .sin()
            > 0.
        {
            1.
        } else {
            -1.
        };

        let result = 0.1 * square + 0.9 * rand::thread_rng().gen_range(-1., 1.);

        Frame::mono(result)
    }
}
