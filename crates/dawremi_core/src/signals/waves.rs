use core::f64::consts::PI;

/// Generates a full period of a sine wave that lasts `length`
pub fn sine_one_period(length: usize) -> Vec<f64> {
    let w = 2.0 * PI / length as f64;
    (0..length).map(|i| f64::sin((i as f64) * w)).collect()
}

/// Generates a sine wave of frequency
pub fn sine(length: usize, frequency: f64) -> Vec<f64> {
    let w = 2.0 * PI * frequency;
    (0..length).map(|i| f64::sin((i as f64) * w)).collect()
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
        let noise = sine(10, 0.1);

        assert_eq!(10, noise.len());
    }

    #[test]
    fn check_middlepoint_values_in_sine() {
        // We take an extra one, so we can check the last value too

        let noise = sine(11, 0.1);
        assert!(noise[0].abs() < 0.0001);
        assert!(noise[4].abs() > 0.0001);
        assert!(noise[5].abs() < 0.0001);
        assert!(noise[6].abs() > 0.0001);
        assert!(noise[10].abs() < 0.0001);

        let noise = sine(101, 0.01);
        assert!(noise[0].abs() < 0.0001);
        assert!(noise[49].abs() > 0.0001);
        assert!(noise[50].abs() < 0.0001);
        assert!(noise[51].abs() > 0.0001);
        assert!(noise[100].abs() < 0.0001);
    }
}
