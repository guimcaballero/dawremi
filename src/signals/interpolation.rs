/// Basic lerp function
pub fn lerp(a: f64, b: f64, i: f64) -> f64 {
    // a + i * (b - a)
    i.mul_add(b - a, a)
}

/// Returns an n-length vector of values lerping from  a to b
pub fn equidistant_points(a: f64, b: f64, n: usize) -> Vec<f64> {
    (0..n).map(|i| lerp(a, b, i as f64 / n as f64)).collect()
}

/// Interpolates between the provided values at the given indexes
pub fn interpolate(mut vec: Vec<(usize, f64)>) -> Vec<f64> {
    if vec.is_empty() {
        return Vec::new();
    }

    if vec[0].0 != 0 {
        vec.insert(0, (0, 0.));
    }

    let mut res = Vec::new();

    for win in vec.windows(2) {
        res.append(&mut equidistant_points(
            win[0].1,
            win[1].1,
            win[1].0 - win[0].0,
        ));
    }
    res.push(vec.last().unwrap().1);

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interpolation_works_empty() {
        let expected = Vec::<f64>::new();
        let actual = interpolate(Vec::new());

        assert_eq!(expected, actual);
    }

    #[test]
    fn interpolation_works() {
        let expected = vec![0., 0.5, 1., 0.5, 0.];
        let actual = interpolate(vec![(2, 1.), (4, 0.)]);
        assert_eq!(expected, actual);

        let expected = vec![0., 1. / 3., 2. / 3., 1., 0.];
        let actual = interpolate(vec![(3, 1.), (4, 0.)]);
        assert_eq!(expected, actual);
    }
}
