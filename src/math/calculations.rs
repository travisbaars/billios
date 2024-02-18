//! Calculations

/// Sand used calculation
pub struct SandUsed {
  cone_pre_test: f64,
  cone_post_test: f64,
  sand_in_cone: Option<f64>,
}


impl SandUsed {
  /// Create a new instance of SandUsed
  pub fn new(cone_pre_test: f64, cone_post_test: f64, sand_in_cone: Option<f64>) -> Self {
    Self { cone_pre_test, cone_post_test, sand_in_cone }
  }
}

/// Wet Density calculation
pub struct WetDensity {
  soil: f64,
  sand_used: SandUsed,
  sand_density: Option<f64>,
}

impl WetDensity {
  /// Create a new instance of WetDensity
  pub fn new(soil: f64, sand_used: SandUsed, sand_density: Option<f64>) -> Self {
    Self { soil, sand_used, sand_density }
  }
}

/// Moisture Content calculation
pub struct MoistureContent {
  wet_weight: f64,
  dry_weight: f64,
  tare_pan: f64,
}

impl MoistureContent {
  /// Create new instance of MoistureContent
  pub fn new(wet_weight: f64, dry_weight: f64, tare_pan: f64) -> Self {
    Self { wet_weight, dry_weight, tare_pan }
  }
}

/// Dry Density calculation
pub struct DryDensity {
  wet_density: WetDensity,
  moisture_content: MoistureContent,
}

impl DryDensity {
  /// Create a new instance of DryDensity
  pub fn new(wet_density: WetDensity, moisture_content: MoistureContent) -> Self {
    Self { wet_density, moisture_content }
  }
}

/// Percent Compaction calculation
pub struct Compaction {
  dry_density: DryDensity,
  lab_max: f64,
}

impl Compaction {
  /// Create new instance of Compaction
  pub fn new(dry_density: DryDensity, lab_max: f64) -> Self {
    Self { dry_density, lab_max }
  }
}

/// Rock Correction (percent oversize) calculation
pub struct RockCorrection {
  left_on_sieve_weight: f64,
  pre_sieve_rock_correction: f64,
}

impl RockCorrection {
  /// Create new instance of RockCorrection
  pub fn new(left_on_sieve_weight: f64, pre_sieve_rock_correction: f64) -> Self {
    Self { left_on_sieve_weight, pre_sieve_rock_correction }
  }
}

/// Lab Max Correction calculation
pub struct LabMaxCorrection {
  rock_correction: RockCorrection,
  lab_max: f64,
  specific_gravity: Option<f64>,
}

impl LabMaxCorrection {
  /// Create new instance of LabMaxCorrection
  pub fn new(rock_correction: RockCorrection, lab_max: f64, specific_gravity: Option<f64>) -> Self {
    Self { rock_correction, lab_max, specific_gravity }
  }
}