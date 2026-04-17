pub mod floats;

use std::ops;

use num_traits::{Float, FloatConst, MulAdd};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex<T> {
    real: T,
    imag: T,
}

impl<T: Float + FloatConst> Default for Complex<T> {
    fn default() -> Self {
        Self {
            real: T::zero(),
            imag: T::zero(),
        }
    }
}

impl<T> Complex<T> {
    const fn new(real: T, imag: T) -> Self {
        Self { real, imag }
    }

    fn real(self) -> T {
        self.real
    }

    fn imag(self) -> T {
        self.imag
    }
}

fn add_multi<T>(x: T, y: T, z: T) -> T
where
    T: MulAdd<T, T, Output = T>,
{
    x.mul_add(y, z)
}

impl<T> ops::Add for Complex<T>
where
    T: ops::Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.real() + rhs.real(), self.imag() + rhs.imag())
    }
}

impl<T> ops::Mul for Complex<T>
where
    T: ops::Mul<Output = T> + Copy + ops::Sub<Output = T> + ops::Add<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let re = self.real() * rhs.real() - self.imag() * rhs.imag();
        let im = self.real() * rhs.imag() + self.imag() * rhs.real();
        Self::Output::new(re, im)
    }
}

impl<T> ops::Sub for Complex<T>
where
    T: ops::Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let re: T = self.real() - rhs.real();
        let im: T = self.imag() - rhs.imag();
        Self::Output::new(re, im)
    }
}

impl<T> ops::Sub<Complex<T>> for f32
where
    T: Float + ops::Sub<Output = T> + Copy,
{
    type Output = Complex<T>;

    fn sub(self, rhs: Complex<T>) -> Self::Output {
        let real: T = rhs.real();
        let imag: T = rhs.imag();

        let rhs_1: T = T::from(self).unwrap();
        Complex::new(rhs_1 - real, T::zero() - imag)
    }
}

impl<T> ops::Div for Complex<T>
where
    T: ops::Mul<Output = T>
        + Copy
        + ops::Sub<Output = T>
        + ops::Add<Output = T>
        + ops::Div<Output = T>
        + num_traits::MulAdd<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let re = add_multi(self.real(), rhs.real(), self.imag() * rhs.imag())
            / add_multi(rhs.real(), rhs.real(), rhs.imag() * rhs.imag());
        let im = (self.imag() * rhs.real() - self.real() * rhs.imag())
            / add_multi(rhs.real(), rhs.real(), rhs.imag() * rhs.imag());
        Self::Output::new(re, im)
    }
}

#[cfg(test)]
mod test {
    use crate::{Complex};

    #[test]
    fn test_complex_add() {
        let complex1 = Complex::new(1.0, 2.0);
        let complex2 = Complex::new(2.0, 3.0);
        let result = complex1 + complex2;
        assert_eq!(result, Complex::new(3.0, 5.0));
    }

    #[test]
    fn test_complex_mul() {
        let complex1 = Complex::new(1.0, 2.0);
        let complex2 = Complex::new(3.0, 4.0);

        let result = complex1 * complex2;

        assert_eq!(result, Complex::new(-5.0, 10.0));
    }

    #[test]
    fn test_complex_sub() {
        let complex1 = Complex::new(1.0, 2.0);
        let complex2 = Complex::new(3.0, 2.0);

        let result = complex1 - complex2;

        assert_eq!(result, Complex::new(-2.0, 0.0));
    }

    #[test]
    fn test_complex_div() {
        let complex1 = Complex::new(1.0, 2.0);
        let complex2 = Complex::new(2.0, 3.0);

        let result = complex1 / complex2;

        assert_eq!(result, Complex::new(0.6153846153846154, 0.07692307692307693));
    }
}
