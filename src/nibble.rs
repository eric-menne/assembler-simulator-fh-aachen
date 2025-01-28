use core::fmt::Display;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::ops::{Add, Sub};

/// A 4-bit integer, representing a nibble in a simulated processor.
///
/// This type simulates a real 4-bit addition with carry bit, allowing for
/// accurate arithmetic operations on 4-bit integers.
///
/// # Example
/// ```
/// use asim::Nibble;
///
/// let a = Nibble::from(5);
/// let b = Nibble::from(2);
/// let result = a + b;
/// println!("Result: {}", result);
/// ```
#[derive(Clone, Copy, Default)]
pub struct Nibble(u8);

impl Nibble {
    #[inline]
    pub fn get_value(&self) -> u8 {
        self.0 & 0b00001111
    }

    #[inline]
    pub fn has_carry(&self) -> bool {
        (self.0 & 0b00010000) != 0
    }

    #[inline]
    pub fn has_negative(&self) -> bool {
        (self.0 & 0b00001000) != 0
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 & 0b00001111 == 0
    }

    #[inline]
    #[allow(unused)]
    pub fn as_unsigned(&self) -> i32 {
        self.get_value() as i32
    }

    #[allow(unused)]
    pub fn as_signed(&self) -> i32 {
        if self.has_negative() {
            return -((self.arithmetic_complement()) as i32);
        }
        self.get_value() as i32
    }

    fn arithmetic_complement(&self) -> u8 {
        ((!self.get_value()) & 0b00001111) + 1
    }
}

impl Debug for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nibble [{:#010b}]", self.0)
    }
}

impl Display for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_value())
    }
}

impl From<usize> for Nibble {
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<u8> for Nibble {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<i32> for Nibble {
    fn from(value: i32) -> Self {
        Self(value as u8)
    }
}

impl Into<usize> for Nibble {
    fn into(self) -> usize {
        (self.get_value()) as usize
    }
}

impl Into<u8> for Nibble {
    fn into(self) -> u8 {
        self.get_value()
    }
}

impl Into<i32> for Nibble {
    fn into(self) -> i32 {
        (self.get_value()) as i32
    }
}

impl Add for Nibble {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.get_value() + rhs.get_value())
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Nibble {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.get_value() + rhs.arithmetic_complement())
    }
}

impl PartialEq for Nibble {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl PartialEq<i32> for Nibble {
    fn eq(&self, other: &i32) -> bool {
        self.get_value() as i32 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::Nibble;

    #[test]
    fn test_nibble_addition() {
        let a = Nibble::from(5);
        let b = Nibble::from(2);

        let result = a + b;

        assert!(result.as_signed() == 7, "{:?}", result);
        assert!(result.as_unsigned() == 7, "{:?}", result);

        assert!(result.has_carry() == false, "{:?}", result);
        assert!(result.has_negative() == false, "{:?}", result);
        assert!(result.is_zero() == false, "{:?}", result);
    }

    #[test]
    fn test_nibble_addition_with_overflow() {
        let a = Nibble::from(7);
        let b = Nibble::from(1);

        let result = a + b;

        assert!(result.as_unsigned() == 8, "{:?}", result);
        assert!(result.as_signed() == -8, "{:?}", result);

        assert!(result.has_carry() == false, "{:?}", result);
        assert!(result.has_negative() == true, "{:?}", result);
        assert!(result.is_zero() == false, "{:?}", result);
    }

    #[test]
    fn test_nibble_subtraction() {
        let a = Nibble::from(5);
        let b = Nibble::from(2);

        let result = a - b;

        assert!(result.as_unsigned() == 3, "{:?}", result);
        assert!(result.as_signed() == 3, "{:?}", result);

        assert!(result.has_carry() == true, "{:?}", result);
        assert!(result.has_negative() == false, "{:?}", result);
        assert!(result.is_zero() == false, "{:?}", result);
    }

    #[test]
    fn test_nibble_subtraction_with_overflow() {
        let a = Nibble::from(2);
        let b = Nibble::from(4);

        let result = a - b;

        assert!(result.as_unsigned() == 14, "{:?}", result);
        assert!(result.as_signed() == -2, "{:?}", result);

        assert!(result.has_carry() == false, "{:?}", result);
        assert!(result.has_negative() == true, "{:?}", result);
        assert!(result.is_zero() == false, "{:?}", result);
    }
}
