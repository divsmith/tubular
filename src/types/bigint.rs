use num_bigint::BigInt;
use num_traits::{Zero, Signed, ToPrimitive};
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct TubularBigInt(pub BigInt);

impl TubularBigInt {
    pub fn new(value: i64) -> Self {
        TubularBigInt(BigInt::from(value))
    }

    pub fn zero() -> Self {
        TubularBigInt(BigInt::from(0))
    }

    pub fn one() -> Self {
        TubularBigInt(BigInt::from(1))
    }

    pub fn from_bigint(value: BigInt) -> Self {
        TubularBigInt(value)
    }

    pub fn into_bigint(self) -> BigInt {
        self.0
    }

    pub fn as_bigint(&self) -> &BigInt {
        &self.0
    }

    pub fn as_bigint_mut(&mut self) -> &mut BigInt {
        &mut self.0
    }

    pub fn increment(&mut self) -> &mut Self {
        self.0 += 1;
        self
    }

    pub fn decrement(&mut self) -> &mut Self {
        self.0 -= 1;
        self
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn is_positive(&self) -> bool {
        self.0 > BigInt::from(0)
    }

    pub fn is_negative(&self) -> bool {
        self.0 < BigInt::from(0)
    }

    pub fn abs(&self) -> Self {
        TubularBigInt(self.0.abs())
    }

    pub fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    pub fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }

    pub fn to_char(&self) -> Option<char> {
        self.to_i64().and_then(|n| std::char::from_u32(n as u32))
    }

    pub fn from_char(c: char) -> Self {
        TubularBigInt(BigInt::from(c as u32 as i64))
    }

    pub fn safe_div(&self, other: &Self) -> Self {
        if other.is_zero() {
            TubularBigInt::zero()
        } else {
            TubularBigInt(&self.0 / &other.0)
        }
    }

    pub fn safe_mod(&self, other: &Self) -> Self {
        if other.is_zero() {
            TubularBigInt::zero()
        } else {
            TubularBigInt(&self.0 % &other.0)
        }
    }
}

impl Default for TubularBigInt {
    fn default() -> Self {
        TubularBigInt::zero()
    }
}

impl From<i64> for TubularBigInt {
    fn from(value: i64) -> Self {
        TubularBigInt::new(value)
    }
}

impl From<BigInt> for TubularBigInt {
    fn from(value: BigInt) -> Self {
        TubularBigInt::from_bigint(value)
    }
}

impl PartialEq for TubularBigInt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for TubularBigInt {}

impl PartialOrd for TubularBigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for TubularBigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Display for TubularBigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for TubularBigInt {
    type Output = TubularBigInt;

    fn add(self, other: Self) -> Self::Output {
        TubularBigInt(self.0 + other.0)
    }
}

impl Sub for TubularBigInt {
    type Output = TubularBigInt;

    fn sub(self, other: Self) -> Self::Output {
        TubularBigInt(self.0 - other.0)
    }
}

impl Mul for TubularBigInt {
    type Output = TubularBigInt;

    fn mul(self, other: Self) -> Self::Output {
        TubularBigInt(self.0 * other.0)
    }
}

impl Div for TubularBigInt {
    type Output = TubularBigInt;

    fn div(self, other: Self) -> Self::Output {
        self.safe_div(&other)
    }
}

impl Rem for TubularBigInt {
    type Output = TubularBigInt;

    fn rem(self, other: Self) -> Self::Output {
        self.safe_mod(&other)
    }
}