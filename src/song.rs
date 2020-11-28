use dasp::{signal, Sample, Signal};

pub trait Song {
    fn play(&self, sample_rate: f64, one_sec: usize) -> Audio;
}

pub type Audio = Box<dyn Iterator<Item = f32> + Send>;

pub struct Test;
impl Song for Test {
    fn play(&self, sample_rate: f64, one_sec: usize) -> Audio {
        // Create a signal chain to play back 1 second of each oscillator at A4.
        let hz = signal::rate(sample_rate).const_hz(440.0);
        let synth = hz
            .clone()
            .sine()
            .take(one_sec)
            .chain(saw(sample_rate, one_sec))
            .chain(hz.clone().square().take(one_sec))
            .chain(hz.clone().noise_simplex().take(one_sec))
            .chain(signal::noise(0).take(one_sec))
            .map(|s| s.to_sample::<f32>() * 0.2);
        Box::new(synth)
    }
}

fn saw(sample_rate: f64, one_sec: usize) -> Vec<f64> {
    let hz = signal::rate(sample_rate).const_hz(440.0);
    hz.clone().saw().take(one_sec).collect()
}
