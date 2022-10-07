use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::cmp::{PartialEq, PartialOrd, Ordering};

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

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
    fn ne(&self, other:&Self) -> bool {
        self.re != other.re && self.im != other.im
    }
}

impl PartialOrd for Complex {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering>{
        if self.re < other.re && self.im < other.im {
            Some(Ordering::Less)
        }
        else if self.re > other.re && self.im > other.im {
            Some(Ordering::Greater)
        }
        else if self.re <= other.re && self.im <= other.im {
            None
        }
        else if self.re >= other.re && self.im >= other.im {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }
    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }
    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), None)
    }
    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Equal))
    }
}
