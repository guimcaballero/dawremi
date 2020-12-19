use super::*;

use realfft::{ComplexToReal, RealToComplex};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

// From: https://blog.demofox.org/2015/03/23/diy-synth-convolution-reverb-1d-discrete-convolution-of-audio-samples/

pub struct Convolution {
    sound: Vec<f64>,
}
impl Convolution {
    pub fn new(mut sound: Vec<f64>) -> Self {
        sound.reverse();

        Self { sound }
    }
}

impl Effect for Convolution {
    fn run(&self, mut input: Vec<f64>) -> Vec<f64> {
        let original_len = input.len();

        // Calculate lengths
        let len = {
            let temp = input.len() + self.sound.len();
            if temp % 2 == 0 {
                temp
            } else {
                temp + 1
            }
        };
        let half_len = (len as f64 / 2.) as usize + 1;
        let factor = (half_len as f64).sqrt();

        // Pad the sound to the same size
        let mut sound = self.sound.clone();
        pad_to_len(&mut sound, len);
        pad_to_len(&mut input, len);

        // Get freq domain for input data
        let input_spectrum: Vec<Complex<f64>> = {
            let mut r2c = RealToComplex::<f64>::new(len).unwrap();
            let mut spectrum: Vec<Complex<f64>> = vec![Complex::zero(); half_len];
            // Forward pass of input data
            r2c.process(&mut input, &mut spectrum).unwrap();
            // Normalize data
            spectrum
        };

        // Get freq domain for sound
        let sound_spectrum: Vec<Complex<f64>> = {
            let mut r2c = RealToComplex::<f64>::new(len).unwrap();
            let mut spectrum: Vec<Complex<f64>> = vec![Complex::zero(); half_len];
            // Forward pass of input data
            r2c.process(&mut sound, &mut spectrum).unwrap();
            // Normalize data
            spectrum
        };

        // Multiply the two domains and normalize them
        let comb_spectrum: Vec<Complex<f64>> = input_spectrum
            .iter()
            .zip(sound_spectrum.iter())
            // No idea why, but this 16 keeps the volume good
            .map(|(a, b)| (a * b) / (16. * factor))
            .collect();

        let output: Vec<f64> = {
            let mut outdata: Vec<f64> = vec![0.0; len];
            let mut c2r = ComplexToReal::<f64>::new(len).unwrap();
            // Inverse pass
            c2r.process(&comb_spectrum, &mut outdata).unwrap();
            // Normalize data
            outdata.iter().map(|i| i / factor).collect()
        };

        output.take_samples(original_len)
    }
}

fn pad_to_len(vec: &mut Vec<f64>, len: usize) {
    while vec.len() < len {
        vec.push(0.);
    }
}
