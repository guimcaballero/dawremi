use crate::frame::Frame;
pub use fundsp::prelude;
use fundsp::prelude::hacker::*;

pub trait AudioNodeExtension {
    fn take_mono(&mut self, samples: usize) -> Vec<f64>;
    fn take_stereo(&mut self, samples: usize) -> Vec<Frame>;
}
impl<X> AudioNodeExtension for An<X>
where
    X: AudioNode<Sample = f64>,
{
    fn take_mono(&mut self, samples: usize) -> Vec<f64> {
        self.reset(None);
        (0..samples).map(|_| self.get_mono()).collect()
    }

    fn take_stereo(&mut self, samples: usize) -> Vec<Frame> {
        self.reset(None);
        (0..samples)
            .map(|_| {
                let f = self.get_stereo();
                Frame::new(f.0, f.1)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_take_stereo() {
        let mut c = lfo(|t| {
            xerp11(50.0, 5000.0, snoise(0, t)) * lerp11(0.5, 1.0, ewave_hz(sigmoid(1.0), 8.0, t))
        }) >> triangle();

        c.reset(Some(44_100.0));
        let vec = c.take_stereo(100);

        assert_eq!(100, vec.len());
    }
}
