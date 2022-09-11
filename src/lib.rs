use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

type CValue = f64;

pub struct Complex {
    pub re: CValue,
    pub im: CValue,
}

impl Complex {
    pub fn new(re: CValue, im: CValue) -> Self {
        Self { re, im }
    }
}

trait Inv {
    fn inv(self) -> Complex;
}

impl Inv for Complex {
    fn inv(self) -> Complex {
        let den: CValue = self.re.powf(2.0) + self.im.powf(2.0);

        Complex {
            re: self.re / den,
            im: -self.im / den,
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im < 0.0 {
            write!(
                f,
                "Complex({}{}i)\n     Re({}{}i) = {}\n     Im({}{}i) = {}",
                self.re, self.im, self.re, self.im, self.re, self.re, self.im, self.im
            )
        } else {
            write!(
                f,
                "Complex({}+{}i)\n     Re({}+{}i) = {}\n     Im({}+{}i) = {}",
                self.re, self.im, self.re, self.im, self.re, self.re, self.im, self.im
            )
        }
    }
}
