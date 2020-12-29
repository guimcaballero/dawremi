use super::*;
use crate::helpers::interpolation::interpolate;
use dasp::signal;

instrument!(Plucked, noise_length: usize, noise: Vec<f64>);

pub enum InitialBurstType {
    Random,
    Triangle(usize, usize),
    DoubleTriangle,
    Sine,
}
impl InitialBurstType {
    fn noise(&self, length: usize) -> Vec<f64> {
        match self {
            InitialBurstType::Random => signal::noise(42069).take_samples(length),
            InitialBurstType::DoubleTriangle => {
                interpolate(vec![(length / 4, 1.), (length * 3 / 4, -1.), (length, 0.)])
            }
            InitialBurstType::Triangle(a, b) => {
                let top = (length * a) / b;
                interpolate(vec![(top, 1.), (length, 0.)])
            }
            InitialBurstType::Sine => signal::rate(length as f64)
                .const_hz(1.0)
                .sine()
                .take_samples(length),
        }
    }
}

impl Plucked {
    #[allow(dead_code)]
    pub fn new(burst: InitialBurstType, note: Note, sample_rate: f64) -> Self {
        let freq: Frequency = note.into();
        let noise_length = (sample_rate / freq.0) as usize;
        let noise = burst.noise(noise_length);
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
            // We basically want the initial random noise to not be very noticeable
            attack: self.noise_length * 3,

            decay: self.seconds(0.7),
            release: self.seconds(0.1),

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    fn note(&mut self) -> f64 {
        self.sample += 1;

        let prev = self.noise[(self.sample - 1) % self.noise_length];
        let result = self.noise[self.sample % self.noise_length];
        let next = self.noise[(self.sample + 1) % self.noise_length];
        self.noise[self.sample % self.noise_length] = (result + next + prev) * 0.996 / 3.;

        result
    }
}
