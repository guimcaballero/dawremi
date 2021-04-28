use super::*;

pub struct Harmonica {
    lfo_amplitude: Automation<f64>,
    lfo_frequency: Automation<f64>,
}
impl Default for Harmonica {
    fn default() -> Self {
        Self {
            lfo_amplitude: Automation::Const(0.001),
            lfo_frequency: Automation::Const(7.0),
        }
    }
}

impl Instrument for Harmonica {
    fn default_adsr(sample_rate: u32) -> Adsr {
        let sr = sample_rate as f64;
        Adsr {
            attack: (sr * 0.1) as usize,
            decay: (sr * 0.1) as usize,
            release: (sr * 0.2) as usize,

            attack_amplitude: 1.0,
            sustain_amplitude: 0.8,
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

                let square_1 =
                    if (frequency * time + a_lfo * frequency * (f_lfo * time).sin()).sin() > 0. {
                        1.
                    } else {
                        -1.
                    };
                let square_2 = if (frequency * 1.5 * time).sin() > 0. {
                    1.
                } else {
                    -1.
                };
                let square_3 = if (frequency * 2.0 * time).sin() > 0. {
                    1.
                } else {
                    -1.
                };

                let result = 0.02 * square_1
                    + 0.5 * square_2
                    + 0.15 * square_3
                    + 0.01 * rand::thread_rng().gen_range(-1., 1.);

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
    fn can_generate_from_harmonica() {
        let sample_rate = 44_100;
        let vec = Harmonica::default().generate(
            1000,
            100.,
            sample_rate,
            Adsr {
                attack: 100,
                ..Harmonica::default_adsr(sample_rate)
            },
        );
        assert_eq!(1000, vec.len());
    }
}
