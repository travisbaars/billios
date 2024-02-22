//! Types

use super::calculations::*;

#[derive(Debug, Clone, Copy)]
pub enum SandUsedChoice {
  Value(f64),
  Constructor(SandUsed),
}

#[derive(Debug, Clone, Copy)]
pub enum WetDensityChoice {
  Value(f64),
  Constructor(WetDensity),
}

#[derive(Debug, Clone, Copy)]
pub enum MoistureContentChoice {
  Value(f64),
  Constructor(MoistureContent),
}

#[derive(Debug, Clone, Copy)]
pub enum DryDensityChoice {
  Value(f64),
  Constructor(DryDensity),
}

#[derive(Debug, Clone, Copy)]
pub enum RockCorrectionChoice {
  Value(f64),
  Constructor(RockCorrection),
}