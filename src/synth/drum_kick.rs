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
    fn default_asdr(sample_rate: u32) -> Asdr {
        let sr = sample_rate as f64;
        Asdr {
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
        asdr: Asdr,
    ) -> Vec<Frame> {
        let vec: Vec<Frame> = (0..length)
            .enumerate()
            .map(|(idx, sample)| {
                let a_lfo = self.lfo_amplitude.value(idx);
                let f_lfo = self.lfo_frequency.value(idx);

                let time = TAU * (sample as f64 / sample_rate as f64);

                let result = 0.99
                    * (frequency * time + a_lfo * frequency * (f_lfo * time).sin()).sin()
                    + 0.01 * rand::thread_rng().gen_range(-1., 1.);

                Frame::mono(result)
            })
            .collect();
        vec.multiply(&asdr.generate(length).into_frames())
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
            Asdr {
                attack: 100,
                ..DrumKick::default_asdr(sample_rate)
            },
        );
        assert_eq!(1000, vec.len());
    }
}