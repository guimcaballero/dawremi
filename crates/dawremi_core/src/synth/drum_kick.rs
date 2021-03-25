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

    fn frame(&mut self) -> Frame {
        let a_lfo = 1.;
        let f_lfo = 1.;

        let time = TAU * self.time();

        let result = 0.99
            * (self.frequency.0 * time + a_lfo * self.frequency.0 * (f_lfo * time).sin()).sin()
            + 0.01 * rand::thread_rng().gen_range(-1., 1.);

        Frame::mono(result)
    }
}
