use super::*;

use realfft::RealFftPlanner;
use rustfft::num_complex::Complex;

// From: https://blog.demofox.org/2015/03/23/diy-synth-convolution-reverb-1d-discrete-convolution-of-audio-samples/

pub struct Convolution {
    sound: Vec<Frame>,
}
impl Convolution {
    pub fn new(sound: Vec<Frame>) -> Self {
        Self { sound }
    }
}

impl Effect for Convolution {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let (left, right) = input.split_sides();
        let (sound_left, sound_right) = self.sound.split_sides();

        join_left_and_right_channels(run(left, sound_left), run(right, sound_right))
    }
}

fn run(mut input: Vec<f64>, mut sound: Vec<f64>) -> Vec<f64> {
    let original_len = input.len();

    // Calculate lengths
    let len = {
        let temp = input.len() + sound.len();
        if temp % 2 == 0 {
            temp
        } else {
            temp + 1
        }
    };
    let half_len = (len as f64 / 2.) as usize + 1;
    let factor = (half_len as f64).sqrt();

    // Pad the sound to the same size
    pad_to_len(&mut sound, len);
    pad_to_len(&mut input, len);

    // make a planner
    let mut real_planner = RealFftPlanner::<f64>::new();

    // Get freq domain for input data
    let input_spectrum: Vec<Complex<f64>> = {
        // create a FFT
        let r2c = real_planner.plan_fft_forward(len);
        // make output vector
        let mut spectrum = r2c.make_output_vec();
        // Forward pass of input data
        r2c.process(&mut input, &mut spectrum).unwrap();
        // Normalize data
        spectrum
    };

    // Get freq domain for sound
    let sound_spectrum: Vec<Complex<f64>> = {
        // create a FFT
        let r2c = real_planner.plan_fft_forward(len);
        // make output vector
        let mut spectrum = r2c.make_output_vec();
        // Forward pass of input data
        r2c.process(&mut sound, &mut spectrum).unwrap();
        // Normalize data
        spectrum
    };

    // Multiply the two domains and normalize them
    let mut comb_spectrum: Vec<Complex<f64>> = input_spectrum
        .iter()
        .zip(sound_spectrum.iter())
        // No idea why, but this 16 keeps the volume good
        .map(|(a, b)| (a * b) / (16. * factor))
        .collect();

    let output: Vec<f64> = {
        let c2r = real_planner.plan_fft_inverse(len);
        let mut outdata = c2r.make_output_vec();
        // Inverse pass
        c2r.process(&mut comb_spectrum, &mut outdata).unwrap();
        // Normalize data
        outdata.iter().map(|i| i / factor).collect()
    };

    output.take_samples(original_len)
}

fn pad_to_len(vec: &mut Vec<f64>, len: usize) {
    while vec.len() < len {
        vec.push(0.);
    }
}
