#[derive(Clone, Copy, Default, Debug)]
pub struct Adsr {
    pub attack: usize,
    pub decay: usize,
    pub release: usize,

    pub attack_amplitude: f64,
    pub sustain_amplitude: f64,
}

impl Adsr {
    /// Generates the envelope
    /// Length of envelope is guaranteed to be `length`
    /// This means that the release is included inside length
    pub fn generate(&self, length: usize) -> Vec<f64> {
        let attack_sustain_diff = self.attack_amplitude - self.sustain_amplitude;
        let samples_release_diff = length.checked_sub(self.release).unwrap_or(length);

        (0..length)
            .map(|i| {
                let volume = if i < self.attack {
                    (i as f64 / self.attack as f64) * self.attack_amplitude
                } else if i < self.attack + self.decay {
                    self.attack_amplitude
                        - ((i - self.attack) as f64 / self.decay as f64) * attack_sustain_diff
                } else {
                    self.sustain_amplitude
                };

                let release_multiplier = if i >= samples_release_diff {
                    (length - (i + 1)) as f64 / ((self.release - 1) as f64)
                } else {
                    1.
                };

                volume * release_multiplier
            })
            .collect::<Vec<f64>>()
    }

    /// Generates the envelope with release as an added to the envelope
    /// Total length of the generated envelope will be `length` + release
    /// This is poorly explained, I hope to edit it later when I know how to phrase it
    pub fn generate_with_release(&self, length: usize) -> Vec<f64> {
        self.generate(length + self.release)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let a = Adsr {
            attack: 10,
            decay: 30,
            release: 30,
            attack_amplitude: 1.,
            sustain_amplitude: 0.7,
        };

        let vec = a.generate(120);

        assert_eq!(120, vec.len());
        assert!(vec[0] < 0.000001);
        assert!(vec[119] < 0.000001);
    }

    #[test]
    fn values_longer_than_length() {
        let a = Adsr {
            attack: 10000,
            decay: 10000,
            release: 100000,
            attack_amplitude: 1.0,
            sustain_amplitude: 0.0,
        };

        let vec = a.generate(100);

        assert_eq!(100, vec.len());
    }
}
