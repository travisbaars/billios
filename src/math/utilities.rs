//! Utilities

/// Collection of functions related to getting a power of a number.
pub struct GetNumPower;

impl GetNumPower {
  /// Simple function for getting the power of a number given a base.
  ///
  /// Base defaults to base 10.
  pub fn power_n(power: u32, base: Option<u32>) -> u32 {
    let base = match base { Some(v) => v, None => 10 };

    base.pow(power)
  }

  /// Get the power of a number base 10.
  pub fn power_10(power: u32) -> u32 {
    let base: u32 = 10;

    base.pow(power)
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_power() {
    assert_eq!(100, GetNumPower::power_n(2, Some(10)));
    assert_eq!(100, GetNumPower::power_n(2, None));
  }

  #[test]
  fn test_get_power_10() {
    assert_eq!(100, GetNumPower::power_10(2));
    assert_eq!(1000, GetNumPower::power_10(3));
  }
}