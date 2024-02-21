//! Calculations
use crate::domain::traits::Rounding;
use crate::field_test::types::{DryDensityChoice, MoistureContentChoice, WetDensityChoice, RockCorrectionChoice};
use super::constants::*;
// use super::utilities::Rounding;

/// Sand used calculation
///
/// # Example
///
/// ```
/// use billios::field_test::SandUsed;
///
/// let sand_used = SandUsed::new(14.65, 8.75, None);
/// assert_eq!(2.31, sand_used.calculate());
/// ```
///
#[derive(Debug, Clone, Copy)]
pub struct SandUsed {
  cone_pre_test: f64,
  cone_post_test: f64,
  sand_in_cone: Option<f64>,
}

impl Rounding for SandUsed {}

impl SandUsed {
  /// Create a new instance of `SandUsed`.
  ///
  /// # Arguments
  ///
  /// - `cone_pre_test` - A float that corresponds to the measurement: **Cone Pre Test**.
  ///
  /// - `cone_post_test` - A float that corresponds to the measurement: **Cone Post Test**.
  ///
  /// - `sand_in_cone` - An `Option<f64>` that when `None` defaults to the constant value `SAND_IN_CONE`. This can be modified as needed by setting a `Some()` value.
  ///
  pub fn new(cone_pre_test: f64, cone_post_test: f64, sand_in_cone: Option<f64>) -> Self {
    Self { cone_pre_test, cone_post_test, sand_in_cone }
  }

  /// Calculate the Sand Used value.
  pub fn calculate(&self) -> f64 {
    let sand_in_cone = self.get_sand_in_cone();

    let result = self.cone_pre_test - (self.cone_post_test + sand_in_cone);

    self.round_n(result, 2).expect("Error rounding the result.")
  }

  /// Getter for `cone_pre_test`
  pub fn get_cone_pre_test(&self) -> f64 {
    self.cone_pre_test
  }

  /// Getter for `cone_post_test`
  pub fn get_cone_post_test(&self) -> f64 {
    self.cone_post_test
  }

  /// Getter for `sand_in_cone`
  ///
  /// If no value is provided, the default is set to the `SAND_IN_CONE` constant.
  fn get_sand_in_cone(&self) -> f64 {
    match self.sand_in_cone {
      Some(v) => v,
      None => SAND_IN_CONE,
    }
  }
}

/// Wet Density calculation
///
/// # Example
///
/// ```
/// use billios::field_test::WetDensity;
///
/// let wet_density = WetDensity::new(4.65, 2.31, None);
/// assert_eq!(177.1429, wet_density.calculate());
/// ```
///
#[derive(Debug, Clone, Copy)]
pub struct WetDensity {
  soil: f64,
  sand_used: f64,
  sand_density: Option<f64>,
}

impl Rounding for WetDensity {}

impl WetDensity {
  /// Create a new instance of `WetDensity`.
  ///
  /// # Arguments
  ///
  /// - `soil` - A float that corresponds to the measurement: **Soil**.
  ///
  /// - `sand_used` - A float that corresponds to the measurement: **Sand Used**
  ///
  /// - `sand_density` - An `Option<f64>` that when `None` defaults to the constant value `SAND_DENSITY`. This can be modified as needed by setting a `Some()` value.
  ///
  pub fn new(soil: f64, sand_used: f64, sand_density: Option<f64>) -> Self {
    Self { soil, sand_used, sand_density }
  }

  /// Calculate the Wet Density value.
  pub fn calculate(&self) -> f64 {
    let sand_density = self.get_sand_density();

    let result = (self.soil / self.sand_used) * sand_density;

    self.round_n(result, 4).expect("Error rounding the result.")
  }

  /// Getter for `soil`
  pub fn get_soil(&self) -> f64 {
    self.soil
  }

  /// Getter for `sand_used`
  pub fn get_sand_used(&self) -> f64 {
    self.sand_used
  }

  /// Getter for `sand_density`
  pub fn get_sand_density(&self) -> f64 {
    match self.sand_density {
        Some(v) => v,
        None => SAND_DENSITY,
    }
  }
}

/// Moisture Content calculation
///
/// # Example
///
/// ```
/// use billios::field_test::MoistureContent;
///
/// let moisture_content = MoistureContent::new(1600., 1575., 1400.);
/// assert_eq!(0.14285714, moisture_content.calculate());
/// ```
///
#[derive(Debug, Clone, Copy)]
pub struct MoistureContent {
  wet_weight: f64,
  dry_weight: f64,
  tare_pan: f64,
}

impl Rounding for MoistureContent {}

impl MoistureContent {
  /// Create new instance of MoistureContent
  ///
  /// # Arguments
  ///
  /// - `wet_weight` - A float that corresponds to the measurement: **Wet Weight**.
  ///
  /// - `dry_weight` - A float that corresponds to the measurement: **Dry Weight**
  ///
  /// - `tare_pan` - A float that corresponds to the measurement: **Tare, Pan **
  ///
  pub fn new(wet_weight: f64, dry_weight: f64, tare_pan: f64) -> Self {
    Self { wet_weight, dry_weight, tare_pan }
  }

  // Calculate the Moisture Content value.
  pub fn calculate(&self) -> f64 {
    let result = (self.wet_weight - self.dry_weight) / (self.dry_weight - self.tare_pan);

    self.round_n(result, 8).expect("Error rounding the result.")
  }

  /// Getter for `wet_weight`
  pub fn get_wet_weight(&self) -> f64 {
    self.wet_weight
  }

  /// Getter for `dry_weight`
  pub fn get_dry_weight(&self) -> f64 {
    self.dry_weight
  }

  /// Getter for `tare_pan`
  pub fn get_tare_pan(&self) -> f64 {
    self.tare_pan
  }
}

/// Dry Density calculation
///
/// # Example 1
///
/// An example using `WetDensityChoice::Value()` and `MoistureContentChoice::Value()`,
///
/// ```
/// use billios::field_test::DryDensity;
/// use billios::field_test::types::{WetDensityChoice, MoistureContentChoice};
///
/// let dry_density = DryDensity::new(WetDensityChoice::Value(177.1429), MoistureContentChoice::Value(0.1428571));
///
/// assert_eq!(155., dry_density.calculate());
/// ```
///
#[derive(Debug, Clone, Copy)]
pub struct DryDensity {
  wet_density: WetDensityChoice,
  moisture_content: MoistureContentChoice,
}

impl Rounding for DryDensity {}

impl DryDensity {
  /// Create a new instance of DryDensity
  ///
  /// # Arguments
  ///
  /// - `wet_density` - A `WetDensityChoice` for the measurement: **Wet Density**. This offers the ability to use either a value (float), or pass an existing `WetDensity::new()` constructor.
  ///
  /// - `moisture_content` - A `MoistureContentChoice` for the measurement: **Moisture Content**. This offers the ability to use either a value (float), or pass an existing `MoistureContent::new()` constructor.
  ///
  pub fn new(wet_density: WetDensityChoice, moisture_content: MoistureContentChoice) -> Self {
    Self { wet_density, moisture_content }
  }

  /// Calculate Dry Density value.
  pub fn calculate(&self) -> f64 {
    let wet_density = self.get_wet_density();
    let moisture_content = self.get_moisture_content();

    let result = wet_density / (1. + moisture_content);

    self.round_n(result, 0).expect("Error rounding the result.")
  }

  /// Getter for `wet_density`
  pub fn get_wet_density(&self) -> f64 {
    match self.wet_density {
      WetDensityChoice::Value(v) => v,
      WetDensityChoice::Constructor(v) => v.calculate(),
    }
  }

  /// Getter for `wet_density`
  pub fn get_moisture_content(&self) -> f64 {
    match self.moisture_content {
      MoistureContentChoice::Value(v) => v,
      MoistureContentChoice::Constructor(v) => v.calculate(),
    }
  }
}

/// Percent Compaction calculation
///
/// # Example 1
///
/// An example using `DryDensityChoice::Value()`.
///
/// ```
/// use billios::field_test::Compaction;
/// use billios::field_test::types::DryDensityChoice;
///
/// let compaction = Compaction::new(DryDensityChoice::Value(155.), 135.6);
/// assert_eq!(114.3, compaction.calculate());
///
/// ```
///
/// # Example 2
///
/// An example using `DryDensityChoice::Constructor()`.
///
/// ```
/// use billios::field_test::{DryDensity, Compaction};
/// use billios::field_test::types::{DryDensityChoice, WetDensityChoice, MoistureContentChoice};
///
/// let lab_max = 135.6;
/// let dry_density = DryDensity::new(WetDensityChoice::Value(177.1429), MoistureContentChoice::Value(0.1428571));
///
/// let compaction = Compaction::new(DryDensityChoice::Constructor(dry_density), lab_max);
/// assert_eq!(114.3, compaction.calculate());
///
/// ```
///
#[derive(Debug, Clone, Copy)]
pub struct Compaction {
  dry_density: DryDensityChoice,
  lab_max: f64,
}

impl Rounding for Compaction {}

impl Compaction {
  /// Create new instance of Compaction
  pub fn new(dry_density: DryDensityChoice, lab_max: f64) -> Self {
    Self { dry_density, lab_max }
  }

  /// Calculate the Compaction value
  pub fn calculate(&self) -> f64 {
    let dry_density = self.get_dry_density();

    let result = (dry_density / self.lab_max) * 100.;

    self.round_n(result, 1).expect("Error rounding the result.")
  }

  /// Getter for `dry_density`.
  pub fn get_dry_density(&self) -> f64 {
    match self.dry_density {
      DryDensityChoice::Value(v) => v,
      DryDensityChoice::Constructor(v) => v.calculate(),
    }
  }

  /// Getter for `lab_max`.
  pub fn get_lab_max(&self) -> f64 {
    self.lab_max
  }
}

/// Rock Correction (percent oversize) calculation
///
/// # Example
///
/// ```
/// use billios::field_test::RockCorrection;
///
/// let left_on_sieve_weight = 100.;
/// let pre_sieve_rock_correction = 500.;
///
/// let rock_correction = RockCorrection::new(left_on_sieve_weight, pre_sieve_rock_correction);
///
/// assert_eq!(0.2, rock_correction.calculate());
///
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RockCorrection {
  left_on_sieve_weight: f64,
  pre_sieve_rock_correction: f64,
}

impl Rounding for RockCorrection {}

impl RockCorrection {
  /// Create new instance of RockCorrection.
  ///
  /// # Arguments
  ///
  /// `left_on_sieve_weight` - A float that corresponds to the measurement: **Left on Sieve Weight**.
  ///
  /// `pre_sieve_rock_correction` - A float that corresponds to the measurement: **Pre Sieve Rock Correction**.
  ///
  pub fn new(left_on_sieve_weight: f64, pre_sieve_rock_correction: f64) -> Self {
    Self { left_on_sieve_weight, pre_sieve_rock_correction }
  }

  /// Calculate the Rock Correction value.
  pub fn calculate(&self) -> f64 {
    let result = self.left_on_sieve_weight / self.pre_sieve_rock_correction;

    self.round_n(result, 1).expect("Error rounding the result.")
  }

  /// Getter for `left_on_sieve_weight`.
  pub fn get_left_on_sieve_weight(&self) -> f64 {
    self.left_on_sieve_weight
  }

  /// Getter for `pre_sieve_rock_correction`.
  pub fn get_pre_sieve_rock_correction(&self) -> f64 {
    self.pre_sieve_rock_correction
  }
}

/// Lab Max Correction calculation
///
/// # Example 1
///
/// Example using a `RockCorrectionChoice::Value()`.
///
/// ```
/// use billios::field_test::LabMaxCorrection;
/// use billios::field_test::types::RockCorrectionChoice;
///
/// let lab_max = 135.6;
/// let rock_correction = 0.2;
/// let lab_max_correction = LabMaxCorrection::new(RockCorrectionChoice::Value(rock_correction), lab_max, None);
///
/// assert_eq!(139.7, lab_max_correction.calculate());
///
/// ```
///
/// # Example 2
///
/// Example using a `RockCorrectionChoice::Constructor()`.
///
/// ```
/// use billios::field_test::{LabMaxCorrection, RockCorrection};
/// use billios::field_test::types::RockCorrectionChoice;
///
/// let lab_max = 135.6;
/// let left_on_sieve_weight = 100.;
/// let pre_sieve_rock_correction = 500.;
/// let rock_correction = RockCorrection::new(left_on_sieve_weight, pre_sieve_rock_correction);
///
/// let lab_max_correction = LabMaxCorrection::new(RockCorrectionChoice::Constructor(rock_correction), lab_max, None);
///
/// assert_eq!(139.7, lab_max_correction.calculate());
///
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LabMaxCorrection {
  rock_correction: RockCorrectionChoice,
  lab_max: f64,
  specific_gravity: Option<f64>,
}

impl Rounding for LabMaxCorrection {}

impl LabMaxCorrection {
  /// Create new instance of LabMaxCorrection.
  ///
  /// # Arguments
  ///
  /// `rock_correction` - A `RockCorrectionChoice` for the measurement: **Rock Correction**. This offers the ability to use either a value (float), or pass an existing constructor (`RockCorrection::new()`).
  ///
  /// `lab_max` - A float corresponding to the measurement: **Lab Max**.
  ///
  /// `specific_gravity` - An `Option<f64>` that when `None` defaults to the constant value `SPECIFIC_GRAVITY`. This can be modified as needed by setting a `Some()` value.
  ///
  pub fn new(rock_correction: RockCorrectionChoice, lab_max: f64, specific_gravity: Option<f64>) -> Self {
    Self { rock_correction, lab_max, specific_gravity }
  }

  /// Calculate the Lab Max Correction value.
  pub fn calculate(&self) -> f64 {
    let rock_correction = self.get_rock_correction();
    let specific_gravity = self.get_specific_gravity();

    let result = (1. - 0.05 * rock_correction) / (rock_correction / (62.4 * specific_gravity) + (1. - rock_correction) / self.lab_max);

    self.round_n(result, 1).expect("Error rounding the result.")
  }

  /// Getter for `rock_correction` value.
  pub fn get_rock_correction(&self) -> f64 {
    match self.rock_correction {
      RockCorrectionChoice::Value(v) => v,
      RockCorrectionChoice::Constructor(c) => c.calculate(),
    }
  }

  /// Getter for `lab_max` value.
  pub fn get_lab_max(&self) -> f64 {
    self.lab_max
  }

  /// Getter for `specific_gravity`.
  ///
  /// If no value is provided, the default is set to the `SPECIFIC_GRAVITY` constant.
  pub fn get_specific_gravity(&self) -> f64 {
    match self.specific_gravity {
      Some(v) => v,
      None => SPECIFIC_GRAVITY,
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sand_used_new() {
    // Test with Some() value.
    let sand_used_some = SandUsed::new(10., 15., Some(20.));

    assert_eq!(10., sand_used_some.get_cone_pre_test());
    assert_eq!(15., sand_used_some.get_cone_post_test());
    assert_eq!(20., sand_used_some.get_sand_in_cone());

    // Test with None, (default), value.
    let sand_used_none = SandUsed::new(10., 15., None);

    assert_eq!(10., sand_used_none.get_cone_pre_test());
    assert_eq!(15., sand_used_none.get_cone_post_test());
    assert_eq!(SAND_IN_CONE, sand_used_none.get_sand_in_cone());
  }

  #[test]
  fn test_sand_used_calculate() {
    let setup = Setup::new();

    // Test with a Some() value.
    let sand_used_some = SandUsed::new(setup.cone_pre_test, setup.cone_post_test, Some(3.59));
    assert_eq!(2.31, sand_used_some.calculate());

    // Test with a None, (default), value.
    let sand_used_none = SandUsed::new(setup.cone_pre_test, setup.cone_post_test, None);
    assert_eq!(2.31, sand_used_none.calculate());
  }

  #[test]
  fn test_wet_density_new() {
    let some = WetDensity::new(10., 15., Some(20.));

    assert_eq!(10., some.get_soil());
    assert_eq!(15., some.get_sand_used());
    assert_eq!(20., some.get_sand_density());

    let none = WetDensity::new(10., 15., None);

    assert_eq!(10., none.get_soil());
    assert_eq!(15., some.get_sand_used());
    assert_eq!(20., some.get_sand_density());
  }

  #[test]
  fn test_wet_density_calculate() {
    let setup = Setup::new();

    let some = WetDensity::new(setup.soil, 2.31, Some(88.));
    assert_eq!(177.1429, some.calculate());

    let none = WetDensity::new(setup.soil, 2.31, None);
    assert_eq!(177.1429, none.calculate());
  }

  #[test]
  fn test_moisture_content_new() {
    let new = MoistureContent::new(10., 15., 20.);

    assert_eq!(10., new.get_wet_weight());
    assert_eq!(15., new.get_dry_weight());
    assert_eq!(20., new.get_tare_pan());
  }

  #[test]
  fn test_moisture_content_calculate() {
    let setup = Setup::new();

    let calc = MoistureContent::new(setup.wet_weight, setup.dry_weight, setup.tare_pan);
    assert_eq!(0.14285714, calc.calculate());
  }

  #[test]
  fn test_dry_density_new() {
    let setup = Setup::new();

    let value = DryDensity::new(WetDensityChoice::Value(10.), MoistureContentChoice::Value(15.));

    assert_eq!(10., value.get_wet_density());
    assert_eq!(15., value.get_moisture_content());

    let wet_density = WetDensity::new(setup.soil, 2.31, None);
    let moisture_content = MoistureContent::new(setup.wet_weight, setup.dry_weight, setup.tare_pan);

    let constructor = DryDensity::new(WetDensityChoice::Constructor(wet_density), MoistureContentChoice::Constructor(moisture_content));

    assert_eq!(177.1429, constructor.get_wet_density());
    assert_eq!(0.14285714, constructor.get_moisture_content());
  }

  #[test]
  fn test_dry_density_calculate() {
    let setup = Setup::new();

    let value = DryDensity::new(WetDensityChoice::Value(177.1429), MoistureContentChoice::Value(0.1428571));
    assert_eq!(155., value.calculate());

    let wet_density = WetDensity::new(setup.soil, 2.31, None);
    let moisture_content = MoistureContent::new(setup.wet_weight, setup.dry_weight, setup.tare_pan);

    let constructor = DryDensity::new(WetDensityChoice::Constructor(wet_density), MoistureContentChoice::Constructor(moisture_content));
    assert_eq!(155., constructor.calculate());
  }

  #[test]
  fn test_compaction_new() {
    let value = Compaction::new(DryDensityChoice::Value(10.), 15.);
    assert_eq!(10., value.get_dry_density());
    assert_eq!(15., value.get_lab_max());

    let dry_density = DryDensity::new(WetDensityChoice::Value(177.1429), MoistureContentChoice::Value(0.1428571));

    let constructor = Compaction::new(DryDensityChoice::Constructor(dry_density), 15.);
    assert_eq!(155., constructor.get_dry_density());
    assert_eq!(15., constructor.get_lab_max());
  }

  #[test]
  fn test_compaction_calculate() {
    let setup = Setup::new();

    let value = Compaction::new(DryDensityChoice::Value(155.), setup.lab_max);
    assert_eq!(114.3, value.calculate());

    let dry_density = DryDensity::new(WetDensityChoice::Value(177.1429), MoistureContentChoice::Value(0.1428571));
    let constructor = Compaction::new(DryDensityChoice::Constructor(dry_density), setup.lab_max);

    assert_eq!(114.3, constructor.calculate());
  }

  #[test]
  fn test_rock_correction_new() {
    let new = RockCorrection::new(10., 15.);

    assert_eq!(10., new.get_left_on_sieve_weight());
    assert_eq!(15., new.get_pre_sieve_rock_correction());
  }

  #[test]
  fn test_rock_correction_calculate() {
    let setup = Setup::new();

    let rc = RockCorrection::new(setup.left_on_sieve_weight, setup.pre_sieve_rock_correction);

    assert_eq!(0.2, rc.calculate());
  }

  #[test]
  fn test_lab_max_correction_new() {
    let setup = Setup::new();

    let value = LabMaxCorrection::new(RockCorrectionChoice::Value(10.), 15., Some(20.));

    assert_eq!(10., value.get_rock_correction());
    assert_eq!(15., value.get_lab_max());
    assert_eq!(20., value.get_specific_gravity());

    let rock_correction = RockCorrection::new(setup.left_on_sieve_weight, setup.pre_sieve_rock_correction);
    let constructor = LabMaxCorrection::new(RockCorrectionChoice::Constructor(rock_correction), 15., Some(20.));

    assert_eq!(0.2, constructor.get_rock_correction());
    assert_eq!(15., constructor.get_lab_max());
    assert_eq!(20., constructor.get_specific_gravity());

    let none = LabMaxCorrection::new(RockCorrectionChoice::Value(10.), 15., None);

    assert_eq!(2.7, none.get_specific_gravity());
  }

  #[test]
  fn test_lab_max_correction_calculate() {
    let setup = Setup::new();

    let value = LabMaxCorrection::new(RockCorrectionChoice::Value(0.2), setup.lab_max, None);

    assert_eq!(139.7, value.calculate());

    let rock_correction = RockCorrection::new(setup.left_on_sieve_weight, setup.pre_sieve_rock_correction);
    let constructor = LabMaxCorrection::new(RockCorrectionChoice::Constructor(rock_correction), setup.lab_max, None);

     assert_eq!(139.7, constructor.calculate());
  }

  struct Setup {
    lab_max: f64,
    _lab_moisture: f64,
    cone_pre_test: f64,
    cone_post_test: f64,
    soil: f64,
    tare_pan: f64,
    wet_weight: f64,
    dry_weight: f64,
    pre_sieve_rock_correction: f64,
    left_on_sieve_weight: f64,
  }

  impl Setup {
    fn new() -> Self {
      Self {
        lab_max: 135.6,
        _lab_moisture: 9.8,
        cone_pre_test: 14.65,
        cone_post_test: 8.75,
        soil: 4.65,
        tare_pan: 1400.,
        wet_weight: 1600.,
        dry_weight: 1575.,
        pre_sieve_rock_correction: 500.,
        left_on_sieve_weight: 100.,
      }
    }
  }
}