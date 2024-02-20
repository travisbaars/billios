//! Traits
//!
use crate::math::utilities::GetNumPower;

/// A trait for adding the ability to round to (n) decimal places.
pub trait Rounding {
  /// Round a floating point number to the number (n) decimal points.
  fn round_n(&self, number: f64, n: u32) -> Result<f64, ()> {
    let power: f64 = GetNumPower::power_10(n).into();

    let result = (number * power).round() / power;

    Ok(result)
  }
}