use super::*;

pub struct DrumKick {
    lfo_amplitude: Automation<f64>,
    lfo_frequency: Automation<f64>,
}
impl Default for DrumKick {
    fn default() -> Self {
        Self {
            lfo_amplitude: Automation::Const(0.001),
            lfo_frequency: Automation::Const(7.0),
        }
    }
}

impl Instrument for DrumKick {
    fn default_adsr(sample_rate: u32) -> Adsr {
        let sr = sample_rate as f64;
        Adsr {
            attack: (sr * 0.01) as usize,
            decay: (sr * 0.15) as usize,
            release: 0,

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
        (0..length)
            .enumerate()
            .map(|(idx, sample)| {
                let a_lfo = self.lfo_amplitude.value(idx);
                let f_lfo = self.lfo_frequency.value(idx);

                let time = TAU * (sample as f64 / sample_rate as f64);

                let result = 0.99_f64.mul_add(
                    frequency
                        .mul_add(time, a_lfo * frequency * (f_lfo * time).sin())
                        .sin(),
                    0.01 * rand::thread_rng().gen_range(-1., 1.),
                );

                Frame::mono(result)
            })
            .collect::<Vec<Frame>>()
            .envelope(&adsr)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_generate_from_kick() {
        let sample_rate = 44_100;
        let vec = DrumKick::default().generate(
            1000,
            100.,
            sample_rate,
            Adsr {
                attack: 100,
                ..DrumKick::default_adsr(sample_rate)
            },
        );
        assert_eq!(1000, vec.len());
    }
}
