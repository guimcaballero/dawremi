use dasp::{signal, Sample, Signal};

pub trait Song {
    fn play(&self, sample_rate: f64) -> Audio;
}

pub type Audio = Box<dyn Iterator<Item = f32> + Send>;

pub struct Test;
impl Song for Test {
    fn play(&self, sample_rate: f64) -> Audio {
        let one_sec = sample_rate as usize;

        // Create a signal chain to play back 1 second of each oscillator at A4.
        let hz0 = signal::rate(sample_rate).const_hz(0.0);
        let hz220 = signal::rate(sample_rate).const_hz(220.0);
        let hz440 = signal::rate(sample_rate).const_hz(440.0);

        // Create two signals that we'll merge
        let synth1 = hz440.clone().sine().take(one_sec * 2);
        let synth2 = hz0
            .sine()
            .take(one_sec)
            .chain(hz220.sine().take(one_sec * 2));
        let signal1 = signal::from_iter(synth1);
        let signal2 = signal::from_iter(synth2);

        // Merge the signals into one
        let synth = signal1
            .zip_map(signal2, |a, b| (a + b) / 2.)
            .map(|s| s.to_sample::<f32>() * 0.2)
            .take(one_sec * 3);

        Box::new(synth)
    }
}

fn saw(sample_rate: f64, duration: usize) -> Vec<f64> {
    let hz = signal::rate(sample_rate).const_hz(440.0);
    hz.clone().saw().take(duration).collect()
}
