use super::*;

// From: https://blog.demofox.org/2015/03/17/diy-synth-multitap-reverb/

pub struct MultitapReverb {
    taps: Vec<(usize, f64)>,
    max_buffer: usize,
}
impl MultitapReverb {
    pub fn new(sample_rate: f64) -> Self {
        fn seconds(x: f64, sample_rate: f64) -> usize {
            (sample_rate * x) as usize
        }

        let taps = vec![
            (seconds(0.079, sample_rate), 0.0562),
            (seconds(0.130, sample_rate), 0.0707),
            (seconds(0.230, sample_rate), 0.1778),
            (seconds(0.340, sample_rate), 0.0707),
            (seconds(0.470, sample_rate), 0.1412),
            (seconds(0.532, sample_rate), 0.0891),
            (seconds(0.662, sample_rate), 0.2238),
        ];

        Self::new_with_taps(taps)
    }

    pub fn new_with_taps(taps: Vec<(usize, f64)>) -> Self {
        Self {
            max_buffer: taps
                .iter()
                .map(|(l, _)| l)
                .max()
                .expect("There should be a max")
                + 10,
            taps,
        }
    }
}

impl Effect for MultitapReverb {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let (left, right) = input.split_sides();

        join_left_and_right_channels(run(self, left), run(self, right))
    }
}

fn run(tap: &MultitapReverb, input: Vec<f64>) -> Vec<f64> {
    let mut reverb_buffer = vec![0.; tap.max_buffer + 1];

    input
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let mut output = *val;
            for (time, amplitude) in &tap.taps {
                if i < *time {
                    continue;
                }
                if let Some(rev) = reverb_buffer.get((i - time) % tap.max_buffer) {
                    output += amplitude * rev;
                }
            }

            reverb_buffer[i % tap.max_buffer] = output;
            output
        })
        .collect()
}

// Some tests because I'm not confident that this implementation works correctly
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_with_0s() {
        let smth = vec![Frame::default(); 100].effect(&MultitapReverb::new(10.));

        assert_eq!(smth, vec![Frame::default(); 100])
    }

    #[test]
    fn it_works() {
        let taps = vec![
            (1, 0.0562),
            (2, 0.0707),
            (3, 0.1778),
            (4, 0.0707),
            (5, 0.1412),
            (6, 0.0891),
            (7, 0.2238),
        ];

        let smth = vec![Frame::mono(0.5); 100].effect(&MultitapReverb {
            taps,
            max_buffer: 8,
        });

        let first = 0.5;
        assert_eq!(smth[0].left, first);
        let second = 0.5 + first * 0.0562;
        assert_eq!(smth[1].left, second);
        let third = 0.5 + second * 0.0562 + first * 0.0707;
        assert_eq!(smth[2].left, third);
        let fourth = 0.5 + third * 0.0562 + second * 0.0707 + first * 0.1778;
        assert_eq!(smth[3].left, fourth);
    }
}
