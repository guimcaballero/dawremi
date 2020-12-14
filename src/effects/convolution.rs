use super::*;

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
    fn run(&self, input: Vec<f64>) -> Vec<f64> {
        let sound_len = self.sound.len();
        let input_len = input.len();
        let len = sound_len + input_len;

        (0..len)
            .map(|out_index| {
                let mut sound_index = if out_index < sound_len {
                    sound_len - out_index - 1
                } else {
                    0
                };
                let mut input_index = if out_index > sound_len {
                    out_index - sound_len
                } else {
                    0
                };

                let mut output = 0.;
                while input_index < input_len && sound_index < sound_len {
                    output += input[input_index] * self.sound[sound_index];

                    sound_index += 1;
                    input_index += 1;
                }

                output
            })
            .collect()
    }
}
