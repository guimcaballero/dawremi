use core::f64::consts::PI;

use crate::effects::Automation;

/// Generates a full period of a sine wave that lasts `length`
pub fn sine_one_period(length: usize) -> Vec<f64> {
    let w = 2.0 * PI / length as f64;
    (0..length).map(|i| f64::sin((i as f64) * w)).collect()
}

/// Generates a sine wave of frequency
pub fn sine(length: usize, frequency: Automation<f64>, sample_rate: f64) -> Vec<f64> {
    let a = 2.0 * PI / sample_rate;

    (0..length)
        .map(|i| f64::sin((i as f64) * a * frequency.value(i)))
        .collect()
}

pub fn square(length: usize, frequency: Automation<f64>, sample_rate: f64) -> Vec<f64> {
    sine(length, frequency, sample_rate)
        .iter()
        .map(|val| val.signum())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_sine_one_period() {
        let noise = sine_one_period(10);

        assert_eq!(10, noise.len());
    }

    #[test]
    fn check_middlepoint_values_in_sine_one_period() {
        let noise = sine_one_period(10);
        assert!(noise[5].abs() < 0.0001);

        let noise = sine_one_period(100);
        assert!(noise[50].abs() < 0.0001);
    }

    #[test]
    fn can_create_sine() {
        let noise = sine(10, Automation::Const(0.1), 48000.);

        assert_eq!(10, noise.len());
    }
}
