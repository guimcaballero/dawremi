use super::*;
use dasp::signal;

instrument!(Plucked, noise_length: usize, noise: Vec<f64>);

impl Plucked {
    #[allow(dead_code)]
    pub fn new(note: Note, sample_rate: f64) -> Self {
        let freq: Frequency = note.into();
        let noise_length = (sample_rate / freq.0) as usize;
        let noise = signal::noise(42069).take_samples(noise_length);
        Self {
            note,
            sample_rate,
            sample: 0,
            noise,
            noise_length,
        }
    }
}

impl SynthInstrument for Plucked {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: 0.01,
            decay: 0.7,
            release: 0.1,

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    fn note(&mut self) -> f64 {
        self.sample += 1;

        let result = self.noise[self.sample % self.noise_length];
        let next = self.noise[(self.sample + 1) % self.noise_length];
        self.noise[self.sample % self.noise_length] = (result + next) * 0.996 / 2.;

        result
    }
}
