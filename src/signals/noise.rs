pub fn noise(length: usize, mut seed: u64) -> Vec<f64> {
    // From: https://github.com/RustAudio/dasp/blob/master/dasp_signal/src/lib.rs

    // A simple one-dimensional noise generator.
    //
    // Credit for the pseudo code from which this was translated goes to Hugo Elias and his
    // excellent primer on perlin noise at
    // http://freespace.virgin.net/hugo.elias/models/m_perlin.htm
    fn noise_1(seed: u64) -> f64 {
        const PRIME_1: u64 = 15_731;
        const PRIME_2: u64 = 789_221;
        const PRIME_3: u64 = 1_376_312_589;
        let x = (seed << 13) ^ seed;
        1.0 - (x
            .wrapping_mul(
                x.wrapping_mul(x)
                    .wrapping_mul(PRIME_1)
                    .wrapping_add(PRIME_2),
            )
            .wrapping_add(PRIME_3)
            & 0x7fffffff) as f64
            / 1_073_741_824.0
    }

    (0..length)
        .map(|_| {
            let noise = noise_1(seed);
            seed += 1;
            noise
        })
        .collect()
}

use crate::notes::Frequency;
use noise::{Fbm, NoiseFn, Seedable};
pub fn fbm(length: usize, frequency: Frequency, seed: u32) -> Vec<f64> {
    let mut fbm = Fbm::new().set_seed(seed);
    fbm.frequency = frequency;

    (0..length)
        .map(|idx| (&fbm).get([idx as f64, 0.]))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_noise() {
        let noise = noise(10, 33);

        assert_eq!(10, noise.len());
    }

    #[test]
    fn noise_is_reproducible() {
        let noise1 = noise(33, 10);
        let noise2 = noise(33, 10);

        let noise3 = noise(69, 100);
        let noise4 = noise(69, 100);

        assert_eq!(noise1, noise2);
        assert_eq!(noise3, noise4);
    }

    #[test]
    fn can_create_fbm() {
        let noise = fbm(10, 33., 1234);

        assert_eq!(10, noise.len());
    }

    #[test]
    fn fbm_is_reproducible() {
        let noise1 = fbm(33, 10., 123);
        let noise2 = fbm(33, 10., 123);

        assert_eq!(noise1, noise2);

        let noise1 = fbm(300, 100., 123);
        let noise2 = fbm(300, 100., 123);

        assert_eq!(noise1, noise2);
    }
}
