use std::ops::{Add, AddAssign, DivAssign};

use gcd::Gcd;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyadicRationalNumber {
    numerator: i32,
    denominator: i32,
}

impl DyadicRationalNumber {
    pub fn numerator(&self) -> i32 {
        self.numerator
    }

    pub fn denominator(&self) -> i32 {
        self.denominator
    }

    pub fn denominator_exponent(&self) -> i32 {
        self.denominator().trailing_zeros() as i32
    }

    fn normalized(&self) -> Self {
        let d = Gcd::gcd(
            self.numerator().abs() as u32,
            self.denominator().abs() as u32,
        );

        DyadicRationalNumber {
            numerator: self.numerator / (d as i32),
            denominator: self.denominator / (d as i32),
        }
    }

    fn normalize(&mut self) {
        let d = Gcd::gcd(
            self.numerator().abs() as u32,
            self.denominator().abs() as u32,
        );

        self.numerator.div_assign(d as i32);
        self.denominator.div_assign(d as i32);
    }

    pub fn rational(numerator: i32, denominator: i32) -> Option<Self> {
        if denominator == 0 {
            return None;
        }
        if numerator == 0 {
            return Some(DyadicRationalNumber {
                numerator: 0,
                denominator: 1,
            });
        }

        let sign = numerator.signum() * denominator.signum();

        // FIXME: Check if fraction is dyadic
        Some(
            DyadicRationalNumber {
                numerator: numerator.abs() * sign,
                denominator: denominator.abs(),
            }
            .normalized(),
        )
    }

    /// Convert to intger if it's an integer
    pub fn to_integer(&self) -> Option<i32> {
        if self.denominator == 1 {
            Some(self.numerator)
        } else {
            None
        }
    }
}

impl From<i32> for DyadicRationalNumber {
    fn from(value: i32) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }
}

impl Add for DyadicRationalNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        DyadicRationalNumber {
            numerator: self.numerator() * rhs.denominator + self.denominator * rhs.numerator,
            denominator: self.denominator() * rhs.denominator(),
        }
        .normalized()
    }
}

impl AddAssign for DyadicRationalNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.numerator = self.numerator() * rhs.denominator + self.denominator * rhs.numerator;
        self.denominator = self.denominator() * rhs.denominator();
        self.normalize();
    }
}

#[test]
fn denominator_exponent_works() {
    assert_eq!(
        DyadicRationalNumber::rational(52, 1)
            .unwrap()
            .denominator_exponent(),
        0
    );
    assert_eq!(
        DyadicRationalNumber::rational(1, 8)
            .unwrap()
            .denominator_exponent(),
        3
    );
}
