use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;

pub fn detect(signal: Vec<f64>, sample_rate: f64) -> f64 {
    const POWER_THRESHOLD: f64 = 5.0;
    const CLARITY_THRESHOLD: f64 = 0.7;

    let size = signal.len();
    let padding = size / 2;

    let mut detector = McLeodDetector::new(size, padding);

    detector
        .get_pitch(
            &signal,
            sample_rate as usize,
            POWER_THRESHOLD,
            CLARITY_THRESHOLD,
        )
        .expect("Couldn't get pitch from audio")
        .frequency
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::effects::Automation;
    use crate::signals::waves::sine;

    #[test]
    fn pitch_detection() {
        for i in 10..100 {
            let freq = i as f64 * 10.;
            let wave = sine(2048, Automation::Const(freq), 48000.);
            assert!((detect(wave, 48000.) - freq).abs() < 1.0);
        }
    }

    #[test]
    fn pitch_detection_in_different_sample_rates() {
        let wave = sine(2048, Automation::Const(440.), 48000.);
        assert!((detect(wave, 48000.) - 440.).abs() < 1.0);

        let wave = sine(2048, Automation::Const(440.), 44100.);
        assert!((detect(wave, 44100.) - 440.).abs() < 3.0);
    }

    #[test]
    fn pitch_detection_fails_if_sample_rates_dont_match_up() {
        let wave = sine(2048, Automation::Const(440.), 48000.);
        assert!((detect(wave, 44100.) - 440.).abs() > 10.);

        let wave = sine(2048, Automation::Const(440.), 44100.);
        assert!((detect(wave, 48000.) - 440.).abs() > 10.);
    }
}
