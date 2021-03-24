//! `Frame` keeps a sample for both the left and the right channel, and provides some helper functions

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub left: f64,
    pub right: f64,
}
impl Frame {
    pub fn new(left: f64, right: f64) -> Self {
        Self { left, right }
    }
    pub fn mono(val: f64) -> Self {
        Self {
            left: val,
            right: val,
        }
    }

    pub fn map<F: FnMut(f64) -> f64>(mut self, mut fun: F) -> Self {
        self.left = fun(self.left);
        self.right = fun(self.right);
        self
    }
    pub fn map_left_right<F: FnMut(f64) -> f64, G: FnMut(f64) -> f64>(
        mut self,
        mut fun_left: F,
        mut fun_right: G,
    ) -> Self {
        self.left = fun_left(self.left);
        self.right = fun_right(self.right);
        self
    }

    pub fn max(&self) -> f64 {
        f64::max(self.left, self.right)
    }
    pub fn min(&self) -> f64 {
        f64::min(self.left, self.right)
    }
    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Self::new(self.left.clamp(min, max), self.right.clamp(min, max))
    }

    pub fn balance(mut self, balance: f64) -> Self {
        if balance < 0. {
            self.right *= 1. + balance;
        } else {
            self.left *= 1. - balance;
        }
        self
    }

    pub fn to_mono(&self) -> f64 {
        (self.left + self.right) / 2.
    }
}

// TODO Some combinations are missing

impl Add for Frame {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.left + rhs.left, self.right + rhs.right)
    }
}

impl Add for &Frame {
    type Output = Frame;

    fn add(self, rhs: Self) -> Self::Output {
        Frame::new(self.left + rhs.left, self.right + rhs.right)
    }
}

impl Add<f64> for Frame {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.left + rhs, self.right + rhs)
    }
}

impl AddAssign for Frame {
    fn add_assign(&mut self, rhs: Self) {
        self.left += rhs.left;
        self.right += rhs.right;
    }
}

impl AddAssign<&Frame> for Frame {
    fn add_assign(&mut self, rhs: &Frame) {
        self.left += rhs.left;
        self.right += rhs.right;
    }
}

impl AddAssign<f64> for Frame {
    fn add_assign(&mut self, rhs: f64) {
        self.left += rhs;
        self.right += rhs;
    }
}

impl Sub for Frame {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.left - rhs.left, self.right - rhs.right)
    }
}

impl Sub<f64> for Frame {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::new(self.left - rhs, self.right - rhs)
    }
}

impl SubAssign for Frame {
    fn sub_assign(&mut self, rhs: Self) {
        self.left -= rhs.left;
        self.right -= rhs.right;
    }
}

impl SubAssign<f64> for Frame {
    fn sub_assign(&mut self, rhs: f64) {
        self.left -= rhs;
        self.right -= rhs;
    }
}

impl SubAssign<&Frame> for Frame {
    fn sub_assign(&mut self, rhs: &Frame) {
        self.left -= rhs.left;
        self.right -= rhs.right;
    }
}

impl Mul for Frame {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.left * rhs.left, self.right * rhs.right)
    }
}

impl Mul for &Frame {
    type Output = Frame;

    fn mul(self, rhs: Self) -> Self::Output {
        Frame::new(self.left * rhs.left, self.right * rhs.right)
    }
}

impl Mul<f64> for Frame {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.left * rhs, self.right * rhs)
    }
}

impl Mul<f64> for &Frame {
    type Output = Frame;

    fn mul(self, rhs: f64) -> Self::Output {
        Frame::new(self.left * rhs, self.right * rhs)
    }
}

impl Mul<&f64> for &Frame {
    type Output = Frame;

    fn mul(self, rhs: &f64) -> Self::Output {
        Frame::new(self.left * rhs, self.right * rhs)
    }
}

impl MulAssign for Frame {
    fn mul_assign(&mut self, rhs: Frame) {
        self.left *= rhs.left;
        self.right *= rhs.right;
    }
}

impl MulAssign<f64> for Frame {
    fn mul_assign(&mut self, rhs: f64) {
        self.left *= rhs;
        self.right *= rhs;
    }
}

impl MulAssign<&Frame> for Frame {
    fn mul_assign(&mut self, rhs: &Frame) {
        self.left *= rhs.left;
        self.right *= rhs.right;
    }
}

impl Div for Frame {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.left / rhs.left, self.right / rhs.right)
    }
}

impl Div<f64> for Frame {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.left / rhs, self.right / rhs)
    }
}

impl Div<f64> for &Frame {
    type Output = Frame;

    fn div(self, rhs: f64) -> Self::Output {
        Frame::new(self.left / rhs, self.right / rhs)
    }
}

impl DivAssign<f64> for Frame {
    fn div_assign(&mut self, rhs: f64) {
        self.left /= rhs;
        self.right /= rhs;
    }
}

impl Neg for Frame {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.left, -self.right)
    }
}

pub enum Side {
    Left,
    Right,
}

impl Index<Side> for Frame {
    type Output = f64;

    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }
}

impl IndexMut<Side> for Frame {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }
}

impl Index<usize> for Frame {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.left,
            1 => &self.right,
            _ => panic!(
                "Out of range exception. Attempted to access value at index {} in Frame",
                index
            ),
        }
    }
}

impl IndexMut<usize> for Frame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.left,
            1 => &mut self.right,
            _ => panic!(
                "Out of range exception. Attempted to access value at index {} in Frame",
                index
            ),
        }
    }
}

pub trait IntoFrames {
    fn into_frames(&self) -> Vec<Frame>;
}
impl IntoFrames for Vec<f64> {
    fn into_frames(&self) -> Vec<Frame> {
        self.iter().map(|val| Frame::mono(*val)).collect()
    }
}

pub trait GetSidesExtension {
    /// Returns the left side of the list
    fn left(&self) -> Vec<f64>;
    /// Returns the right side of the list
    fn right(&self) -> Vec<f64>;

    /// Returns both sides
    fn split_sides(&self) -> (Vec<f64>, Vec<f64>) {
        (self.left(), self.right())
    }
}
impl GetSidesExtension for Vec<Frame> {
    fn left(&self) -> Vec<f64> {
        self.iter().map(|frame| frame.left).collect()
    }
    fn right(&self) -> Vec<f64> {
        self.iter().map(|frame| frame.right).collect()
    }
}
impl GetSidesExtension for &[Frame] {
    fn left(&self) -> Vec<f64> {
        self.iter().map(|frame| frame.left).collect()
    }
    fn right(&self) -> Vec<f64> {
        self.iter().map(|frame| frame.right).collect()
    }
}

pub fn join_left_and_right_channels(left: Vec<f64>, right: Vec<f64>) -> Vec<Frame> {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| Frame::new(*l, *r))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_balance() {
        let frame = Frame::mono(1.);

        assert_eq!(Frame::new(1., 0.), frame.balance(-1.));
        assert_eq!(Frame::new(1., 0.5), frame.balance(-0.5));
        assert_eq!(Frame::new(1., 1.,), frame.balance(0.));
        assert_eq!(Frame::new(0.5, 1.), frame.balance(0.5));
        assert_eq!(Frame::new(0., 1.), frame.balance(1.));
    }
}
