// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SaturatingU16 {
	value: u16
}

impl Add<SaturatingU16> for SaturatingU16 {
	type Output = SaturatingU16;
	fn add(self, value:SaturatingU16) -> Self::Output {
		SaturatingU16 {
			value: self.value.saturating_add(value.value)
		}
	}
}
impl Add<u16> for SaturatingU16 {
	type Output = SaturatingU16;
	fn add(self, value:u16) -> Self::Output {
		SaturatingU16 {
			value: self.value.saturating_add(value)
		}
	}
}
impl Add<&u16> for SaturatingU16 {
	type Output = SaturatingU16;
	fn add(self, value:&u16) -> Self::Output {
		SaturatingU16 {
			value: self.value.saturating_add(*value)
		}
	}
}
impl PartialEq<u16> for SaturatingU16 {
	fn eq(&self, val:&u16) -> bool {
		self.value==*val
	}
}
impl From<u16> for SaturatingU16 {
	fn from(val:u16) -> SaturatingU16 {
		SaturatingU16 {
			value: val
		}
	}
}
impl From<&u16> for SaturatingU16 {
	fn from(val:&u16) -> SaturatingU16 {
		SaturatingU16 {
			value: *val
		}
	}
}
impl From<u8> for SaturatingU16 {
	fn from(val:u8) -> SaturatingU16 {
		SaturatingU16 {
			value: val.into()
		}
	}
}
impl From<&u8> for SaturatingU16 {
	fn from(val:&u8) -> SaturatingU16 {
		SaturatingU16 {
			value: (*val).into()
		}
	}
}