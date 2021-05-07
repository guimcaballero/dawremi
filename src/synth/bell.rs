use super::*;

pub struct Bell {
    lfo_amplitude: Automation<f64>,
    lfo_frequency: Automation<f64>,
}
impl Default for Bell {
    fn default() -> Self {
        Self {
            lfo_amplitude: Automation::Const(0.001),
            lfo_frequency: Automation::Const(7.0),
        }
    }
}

impl Instrument for Bell {
    fn default_adsr(sample_rate: u32) -> Adsr {
        let sr = sample_rate as f64;
        Adsr {
            attack: (sr * 0.01) as usize,
            decay: (sr * 0.7) as usize,
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
        (0..length)
            .enumerate()
            .map(|(idx, sample)| {
                let a_lfo = self.lfo_amplitude.value(idx);
                let f_lfo = self.lfo_frequency.value(idx);

                // Make the base note disappear faster than the rest
                let attack = adsr.attack;
                let base_note_vol_multiplier = if sample > attack {
                    attack as f64 / sample as f64
                } else {
                    1.
                };

                let time = TAU * (sample as f64 / sample_rate as f64);

                let mut result = base_note_vol_multiplier
                    * (frequency.mul_add(time, a_lfo * frequency * (f_lfo * time).sin())).sin();

                // Add higher notes
                let freq = frequency * 2.;
                result += 0.5 * (freq.mul_add(time, a_lfo * freq * (f_lfo * time).sin())).sin();

                let freq = frequency * 3.;
                result += 0.125 * (freq.mul_add(time, a_lfo * freq * (f_lfo * time).sin())).sin();

                let freq = frequency * 4.;
                if sample > attack {
                    result +=
                        0.0125 * (freq.mul_add(time, a_lfo * freq * (f_lfo * time).sin())).sin();
                }

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
    fn can_generate_from_bell() {
        let sample_rate = 44_100;
        let vec = Bell::default().generate(
            1000,
            100.,
            sample_rate,
            Adsr {
                attack: 100,
                ..Bell::default_adsr(sample_rate)
            },
        );
        assert_eq!(1000, vec.len());
    }
}
