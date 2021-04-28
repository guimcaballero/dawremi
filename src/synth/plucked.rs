use super::*;

pub struct Plucked(pub InitialBurstType);

impl Default for Plucked {
    fn default() -> Self {
        Self(InitialBurstType::Random)
    }
}

impl Instrument for Plucked {
    fn default_adsr(sample_rate: u32) -> Adsr {
        let sr = sample_rate as f64;
        Adsr {
            attack: (sr * 0.05) as usize,
            decay: (sr * 0.1) as usize,
            release: (sr * 0.1) as usize,

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    fn generate(
        &self,
        length: usize,
        frequency: Frequency,
        sample_rate: u32,
        adsr: Adsr,
    ) -> Vec<Frame> {
        let noise_length = (sample_rate as f64 / frequency) as usize;
        let mut noise = self.0.noise(noise_length);

        (0..length)
            .map(|sample| {
                let prev = noise[(sample.wrapping_sub(1)) % noise_length];
                let result = noise[sample % noise_length];
                let next = noise[(sample + 1) % noise_length];
                noise[sample % noise_length] = (result + next + prev) * 0.996 / 3.;

                Frame::mono(result)
            })
            .collect::<Vec<Frame>>()
            .envelope(&adsr)
    }
}
use crate::signals::interpolation::interpolate;
use crate::signals::noise::noise;
use crate::signals::waves::sine_one_period_of_length;

pub enum InitialBurstType {
    Random,
    Triangle(usize, usize),
    DoubleTriangle,
    Sine,
    Hill,
}
impl InitialBurstType {
    fn noise(&self, length: usize) -> Vec<f64> {
        match self {
            InitialBurstType::Random => noise(length, 3333),
            InitialBurstType::DoubleTriangle => {
                interpolate(vec![(length / 4, 1.), (length * 3 / 4, -1.), (length, 0.)])
            }
            InitialBurstType::Triangle(a, b) => {
                let top = (length * a) / b;
                interpolate(vec![(top, 1.), (length, 0.)])
            }
            InitialBurstType::Sine => sine_one_period_of_length(length),
            InitialBurstType::Hill => interpolate(vec![
                (length * 2 / 8, 0.),
                (length * 3 / 8, 1.),
                (length * 4 / 8, 1.),
                (length * 5 / 8, 0.),
                (length, 0.),
            ]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_generate_from_plucked() {
        let sample_rate = 44_100;
        let vec = Plucked(InitialBurstType::DoubleTriangle).generate(
            1000,
            100.,
            sample_rate,
            Adsr {
                attack: 100,
                ..DrumHiHat::default_adsr(sample_rate)
            },
        );
        assert_eq!(1000, vec.len());
    }
}
