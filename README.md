# billios - A Soil Library

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/travisbaars/billios/integrate.yml?branch=main&logo=github)
![Crates.io Version](https://img.shields.io/crates/v/billios)
![docs.rs](https://img.shields.io/docsrs/billios)

This crate is a collection of soil related data structures and formulae.

## Installation

Standard functionality, no additional dependencies are required:

```toml
[dependencies]
billios = "0.1.1"
```

Alternatively, you can install using the `cargo add` command:

```bash
cargo add billios
```

## Code Examples

### Example - Sand Used

Using this formula only requires a couple lines of code.

Simply call the `new()` method with the desired values, then call `calculate()` to retrieve the result.

#### Use with constant `sand_in_cone` value:

```rust
use billios::field_test::*;

// This constructor  takes 3 arguments:
//
// cone_pre_test, cone_post_test, sand_in_cone
//
// The first two are straight forward; two float values.
// For the third we use the `None` option. The reason for this
// is becouse the `sand_in_cone` argument is a constant (mostly).
// In general this value does not change often, so we don't have to worry
// about setting it. Instead we just use `None`. If for some reason you
// needed to change the value, you simple pass in a `Some(<f64>).
let sand_used = SandUsed::new(14.65, 8.75, None);

// In order to get the calculated value we call the `calculate()` method:
let result = sand_used.calculate();

assert_eq!(2.31, result);
```

#### Use with a custom `sand_in_cone` value:

```rust
use billios::field_test::SandUsed;

let sand_used = SandUsed::new(14.65, 8.75, Some(3.59));
let result = sand_used.calculate();

assert_eq!(2.31, result);
```

### Example - Compaction

Using this formula can a little more complicated than the last example. However it is still pretty straghtforward.

#### Use with pre-existing `dry_density` and `moisture_content` values:

```rust
use billios::field_test::*;

let wet_density = WetDensityChoice::Value(177.1429);
let moisture_content = MoistureContentChoice::Value(0.1428571);
let dry_density = DryDensity::new(wet_density, moisture_content);

let result = dry_density.calculate();

assert_eq!(155., result);
```

#### Use with `dry_density` and `moisture_content` constructors:

It is not necessary to calculate each of the values.

```rust
use billios::field_test::*;

let wd = WetDensity::new(4.65, 2.31, None);
let wet_density = WetDensityChoice::constructor(wd);
let mc = MoistureContent::new(1600., 1575., 1400.);
let moisture_content = MoistureContentChoice::constructor(mc);

let dry_density = DryDensity::new(wet_density, moisture_content);
let result = dry_density.calculate();

assert_eq!(155., result);
```

## Todo

- [x] Implement GitHub Action workflow
- [x] Create first deployment to crates.io
- [ ] Improve `README.md`
  - [x] Add the build/crates.io/docs badges
- [ ] Complete documentation
  - [x] Need basic docs atleast for crates.io
  - [ ] `lib.rs` needs attention
- [ ] Add `Setter()` methods to calculation structs
- [ ] Possibly change choice enums (ex. `SandUsedChoices`) to `SandUsedOption`
- [x] Rework the public API. Namely how things like `domain::types` are accessed
- [ ] More calculations?
- [ ] Rename `utilities.rs`?
- [x] Rename `calculations.rs` to `field_test.rs`?