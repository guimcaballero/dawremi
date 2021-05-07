use super::*;

// From: https://blog.demofox.org/2015/03/23/diy-synth-convolution-reverb-1d-discrete-convolution-of-audio-samples/

pub struct SlowConvolution {
    sound: Vec<f64>,
}
impl SlowConvolution {
    #[allow(dead_code)]
    pub fn new(mut sound: Vec<Frame>) -> Self {
        sound.reverse();
        Self {
            sound: sound.into_mono(),
        }
    }
}

impl Effect for SlowConvolution {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let (left, right) = input.split_sides();

        join_left_and_right_channels(run(self, left), run(self, right))
    }
}

fn run(slow: &SlowConvolution, input: Vec<f64>) -> Vec<f64> {
    let sound_len = slow.sound.len();
    let input_len = input.len();
    let len = sound_len + input_len;

    println!("Starting convolution: {}", len);
    (0..len)
        .map(|out_index| {
            if out_index % 1000 == 0 {
                println!("Convoluting: {}", out_index);
            }

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
                output += input[input_index] * slow.sound[sound_index];

                sound_index += 1;
                input_index += 1;
            }

            output
        })
        .collect()
}
