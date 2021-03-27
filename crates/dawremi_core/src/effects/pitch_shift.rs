use super::*;

// From https://github.com/nwoeanhinnogaehr/pvoc-plugins/blob/master/src/plugins/pitchshifter.rs

use pvoc::{Bin, PhaseVocoder};

pub struct PitchShift {
    pub sample_rate: f64,
    pub shift: Frame,
}

impl Effect for PitchShift {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let (left, right) = input.split_sides();

        join_left_and_right_channels(
            run(self, left, self.shift.left),
            run(self, right, self.shift.right),
        )
    }
}

fn run(pitch: &PitchShift, input: Vec<f64>, shift: f64) -> Vec<f64> {
    let mut pvoc = PhaseVocoder::new(1, pitch.sample_rate, 256, 4);
    let mut output = vec![0.0; input.len()];

    pvoc.process(
        &[&input],
        &mut [&mut output],
        |channels: usize, bins: usize, input: &[Vec<Bin>], output: &mut [Vec<Bin>]| {
            for i in 0..channels {
                for j in 0..bins / 2 {
                    let index = ((j as f64) * shift) as usize;
                    if index < bins / 2 {
                        output[i][index].freq = input[i][j].freq * shift;
                        output[i][index].amp += input[i][j].amp;
                    }
                }
            }
        },
    );

    output
}
