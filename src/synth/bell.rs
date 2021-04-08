use super::*;

pub struct Bell;

impl Instrument for Bell {
    fn default_asdr(sample_rate: u32) -> Asdr {
        let sr = sample_rate as f64;
        Asdr {
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
        asdr: Asdr,
    ) -> Vec<Frame> {
        let vec: Vec<Frame> = (0..length)
            .map(|sample| {
                let a_lfo = 0.001;
                let f_lfo = 5.0;

                // Make the base note disappear faster than the rest
                let attack = asdr.attack;
                let base_note_vol_multiplier = if sample > attack {
                    attack as f64 / sample as f64
                } else {
                    1.
                };

                let time = TAU * (sample as f64 / sample_rate as f64);

                let mut result = base_note_vol_multiplier
                    * (frequency * time + a_lfo * frequency * (f_lfo * time).sin()).sin();

                // Add higher notes
                let freq = frequency * 2.;
                result += 0.5 * (freq * time + a_lfo * freq * (f_lfo * time).sin()).sin();

                let freq = frequency * 2.;
                result += 0.125 * (freq * time + a_lfo * freq * (f_lfo * time).sin()).sin();

                let freq = frequency * 2.;
                if sample > attack {
                    result += 0.0125 * (freq * time + a_lfo * freq * (f_lfo * time).sin()).sin();
                }

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
    fn can_generate_from_bell() {
        let sample_rate = 44_100;
        let vec = Bell.generate(
            1000,
            100.,
            sample_rate,
            Asdr {
                attack: 100,
                ..Bell::default_asdr(sample_rate)
            },
        );
        assert_eq!(1000, vec.len());
    }
}
