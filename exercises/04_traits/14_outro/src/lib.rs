// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value. done 
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`. done
//   It should support addition with a right-hand side of type. done 
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`. 
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SaturatingU16 {
    val: u16
}

impl From<u16> for SaturatingU16 {
    fn from(val: u16) -> Self {
        SaturatingU16 { val }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(val: u8) -> Self {
        SaturatingU16 { val: val.into() }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(val: &u16) -> Self {
        SaturatingU16 { val: *val }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(val: &u8) -> Self {
        SaturatingU16 { val: (*val).into() }
    }
}


impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { val: self.val.saturating_add(rhs.val) }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self { val: self.val.saturating_add((*rhs).val) }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self {
        Self { val: self.val.saturating_add(rhs) }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &u16) -> Self {
        Self { val: self.val.saturating_add(*rhs) }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.val == *other
    }
}

#[test]
fn test_saturating_u16() {
    let a: SaturatingU16 = (&10u8).into();
    let b: SaturatingU16 = 5u8.into();
    let c: SaturatingU16 = u16::MAX.into();
    let d: SaturatingU16 = (&1u16).into();
    let e = &c;

    assert_eq!(a + b, SaturatingU16::from(15u16));
    assert_eq!(a + c, SaturatingU16::from(u16::MAX));
    assert_eq!(a + d, SaturatingU16::from(11u16));
    assert_eq!(a + a, 20u16);
    assert_eq!(a + 5u16, 15u16);
    assert_eq!(a + e, SaturatingU16::from(u16::MAX));
}