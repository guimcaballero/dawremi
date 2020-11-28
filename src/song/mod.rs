use dasp::{
    signal::{self, ConstHz},
    Sample, Signal,
};

pub trait Song: HasSampleRate {
    fn play(&self) -> Audio;

    /// Returns a ConstHz with this song's sample rate
    fn hz(&self, freq: f64) -> ConstHz {
        signal::rate(self.get_sample_rate()).const_hz(freq)
    }
    /// Returns the number of samples that should be taken to pass x seconds
    fn seconds(&self, x: f64) -> usize {
        (self.get_sample_rate() * x) as usize
    }
}
pub trait HasSampleRate {
    /// Sets sample rate for the Song
    fn set_sample_rate(&mut self, sample_rate: f64);
    /// Should only be called after setting sample_rate
    fn get_sample_rate(&self) -> f64;
}

pub type Audio = Box<dyn Iterator<Item = f32> + Send>;

macro_rules! song {
    ($name:ident, $( $id:ident : $type:ty ),*) => {
        #[derive(Default)]
        pub struct $name {
            sample_rate: Option<f64>,
            $( $id: $type )*
        }

        impl HasSampleRate for $name {
            fn set_sample_rate(&mut self, sample_rate: f64) {
                self.sample_rate = Some(sample_rate);
            }
            fn get_sample_rate(&self) -> f64 {
                self.sample_rate.unwrap()
            }
        }
    };
}

// fn saw(sample_rate: f64, duration: usize) -> Vec<f64> {
//     let hz = signal::rate(sample_rate).const_hz(440.0);
//     hz.clone().saw().take(duration).collect()
// }

pub mod test;
