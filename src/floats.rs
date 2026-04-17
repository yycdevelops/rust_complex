use num_traits::{Float, FloatConst, Pow};

use crate::Complex;

pub trait ComplexFloats<T> {
    fn square(&self) -> T;
    fn magnitude(&self) -> T;
    fn sqrt(&self) -> Self;
    fn atan(&self) -> Self;
    fn atan2(&self) -> T;
    fn cos(&self) -> Self;
    fn sin(&self) -> Self;
    fn asin(&self) -> Self;
    fn log(&self) -> Self;
    fn norm(&self) -> T;
    fn pow(&self, n: T) -> Self
    where
        T: Pow<T, Output = T>;
}

impl<T: Float + FloatConst> ComplexFloats<T> for Complex<T> {
    fn square(&self) -> T {
        self.real() * self.real() + self.imag() * self.imag()
    }

    fn magnitude(&self) -> T {
        T::sqrt(self.square())
    }

    fn sqrt(&self) -> Self {
        let real: T = self.real();
        let imag: T = self.imag();

        let magnitude = self.magnitude();
        let real_part = T::sqrt((magnitude + real) / T::from(2.0).unwrap());
        let mut imag_part = T::sqrt((magnitude - real) / T::from(2.0).unwrap());

        if imag < T::zero() {
            imag_part = -imag_part;
        }

        Self::new(real_part, imag_part)
    }

    fn atan(&self) -> Self {
        Complex::new(T::from(1.0).unwrap(), T::from(1.0).unwrap())
    }

    fn atan2(&self) -> T {
        T::atan2(self.imag(), self.real())
    }

    fn cos(&self) -> Self {
        Self::new(
            T::cos(self.real()) * T::cosh(self.imag()),
            -T::sin(self.real()) * T::sinh(self.imag()),
        )
    }

    fn sin(&self) -> Self {
        Self::new(
            T::sin(self.real()) * T::cosh(self.imag()),
            T::cos(self.real()) * T::sinh(self.imag()),
        )
    }

    fn asin(&self) -> Self {
        let sqr_res: Self = (1.0 - (*self * *self)).sqrt();
        let lz = Complex::new(T::zero(), T::one()) * *self + sqr_res;
        Complex::new(T::zero(), T::one()) * lz.log()
    }

    #[allow(clippy::all)]
    fn log(&self) -> Self {
        Complex::new(
            self.magnitude().log(T::from(2.71828).unwrap()),
            self.atan2(),
        )
    }

    fn norm(&self) -> T {
        self.square()
    }

    fn pow(&self, n: T) -> Self
    where
        T: Pow<T, Output = T>,
    {
        let r: T = self.square().sqrt();
        let theta: T = self.atan2();
        let r_n: T = T::pow(r, n);
        let theta_n: T = n * theta;

        Self::new(r_n * T::cos(theta_n), r_n * T::sin(theta_n))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Complex, floats::ComplexFloats};

    #[test]
    fn test_complex_number_initialization_f32() {
        const COMPL: Complex<f64> = Complex::new(1.0, 2.0);

        assert_eq!(COMPL.real, 1.0);
        assert_eq!(COMPL.imag, 2.0);
    }

    #[test]
    fn test_complex_number_initialization_i32() {
        const COMPL: Complex<i32> = Complex::new(1, 2);

        assert_eq!(COMPL.real, 1);
        assert_eq!(COMPL.imag, 2);
    }

    #[test]
    fn test_complex_number_default() {
        let compl: Complex<f32> = Complex::default();

        assert_eq!(compl.real, 0.0);
        assert_eq!(compl.imag, 0.0);
    }

    #[test]
    fn test_complex_number_real() {
        let mut compl: Complex<f32> = Complex::default();
        assert_eq!(compl.real(), 0.0);
        compl.real = 1.0;
        assert_eq!(compl.real(), 1.0);
    }

    #[test]
    fn test_complex_sqrt() {
        let complex1: Complex<f64> = Complex::new(2.0, 3.0).sqrt();
        assert_eq!(
            complex1,
            Complex::new(1.6741492280355401, 0.895977476129838)
        )
    }

    #[test]
    fn test_complex_atan2() {
        let atan = Complex::new(2.0, 3.0).atan2();
        assert_eq!(atan, 0.982793723247329);
    }

    #[test]
    fn test_complex_sub() {
        let complex = Complex::new(2.0, 3.0);
        let result = 1.0 - complex;
        assert_eq!(result, Complex::new(-1.0, -3.0))
    }

    #[test]
    fn test_complex_log() {
        let complex = Complex::new(2.0, 3.0);
        let result = complex.log();
        assert_eq!(result, Complex::new(1.282475541391427, 0.982793723247329));
    }

    #[test]
    fn test_cos() {
        let complex: Complex<f32> = Complex::new(2.0, 3.0);
        let result = complex.cos();

        assert_eq!(
            result,
            Complex {
                real: -4.1896257,
                imag: -9.109227
            }
        );
    }

    #[test]
    fn test_sin() {
        let complex: Complex<f32> = Complex::new(2.0, 3.0);
        let result = complex.sin();
        assert_eq!(
            result,
            Complex {
                real: 9.154499,
                imag: -4.168907
            }
        );
    }
    #[test]
    fn test_asin() {
        let complex = Complex::new(2.0, 3.0);

        let result = complex.asin();

        assert_eq!(
            result,
            Complex::new(-0.570652784321099, -1.9833883640481096)
        );
    }

    #[test]
    fn test_complex_with_positive_numbers() {
        let complex = Complex::new(100.0, 240.0).sqrt();
        assert_eq!(complex, Complex::new(13.416407864998739, 8.94427190999916));
    }

    #[test]
    fn test_complex_with_negative_numbers() {
        let complex = Complex::new(1.0, -3.0).sqrt();
        assert_eq!(
            complex,
            Complex::new(1.442615274452683, -1.03977826005557050)
        );
    }

    #[test]
    fn test_complex_with_negative_zero() {
        let complex = Complex::new(0.0, -3.4510).sqrt();
        assert_eq!(
            complex,
            Complex::new(1.3135828866120325, -1.3135828866120325)
        );
    }

    #[test]
    fn test_complex_pow_2() {
        let complex = Complex::new(2.0, 3.0).pow(2.0);
        assert_eq!(
            complex,
            Complex::new(-4.999999999999999, 11.999999999999998)
        );
    }

    #[test]
    fn test_complex_pow_neg_2() {
        let complex = Complex::new(2.0, -3.0).pow(2.0);
        assert_eq!(
            complex,
            Complex::new(-4.999999999999999, -11.999999999999998)
        );
    }
}
