use core::f64::consts::PI;

pub fn sine_one_period(length: usize) -> Vec<f64> {
    let w = 2.0 * PI / length as f64;
    (0..length).map(|i| f64::sin((i as f64) * w)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_sine() {
        let noise = sine_one_period(10);

        assert_eq!(10, noise.len());
    }

    #[test]
    fn check_middlepoint_values_in_sine() {
        let noise = sine_one_period(10);
        assert!(noise[5].abs() < 0.0001);

        let noise = sine_one_period(100);
        assert!(noise[50].abs() < 0.0001);
    }
}
