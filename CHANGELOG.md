# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## `x.y.z` (YYYY-MM-DD) [CURRENT | YANKED]

### Added (for new features)
### Changed (for changes in existing functionality)
### Deprecated (for soon-to-be removed features)
### Removed (for now removed features)
### Fixed (for any bug fixes)
### Security
-->

## `0.2.0` (2024-02-22) [CURRENT]

### Added

- Add new `field_test` module which holds all field test related calculations and constants

### Changed

- Move `calculations.rs` from `math` module into the new `field_test` module
- Move `constants.rs` from `domain` module into the new `field_test` module
- Move `types.rs` from `domain` module into the new `field_test` module
- Update paths in `calculations.rs` to reflect moved files
- Update `README.md` Todo list

### Removed

- Remove unused modules from `math/mod.rs`
- Remove unused modules from `domain/mod.rs`



## `0.1.1` (2024-02-20)

### Added

- Add `CHANGELOG.md`
- Add status badges to `README.md`
- Add reusable `test.yml` workflow
- Add `deploy.yml` workflow
- Add project details to `Cargo.toml`

### Changed

- Changed version number for release

### Removed

- Remove `resources` directory

## `0.1.0` (2024-02-20)

### Added

- Add initial public API.
