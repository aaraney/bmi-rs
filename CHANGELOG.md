# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Unsafe `get_value_mut_ptr` method to `bmi_rs::Bmi` trait; analog of bmi-c's `get_value_ptr`.
  Default returns `Err(BmiNotImplementedError)`. [#14](https://github.com/aaraney/bmi-rs/pull/14)

### Changed

- bmi-c ffi `get_value_ptr` wrapper calls `bmi_rs::Bmi::get_value_mut_ptr` instead of always returning `BMI_FAILURE`. [#14](https://github.com/aaraney/bmi-rs/pull/14)

### Deprecated

### Removed

### Fixed

### Security

## [v0.0.1-alpha.0] - 2025-11-07

### Added

- Initial release!
- `bmi-rs`: Crate for exposing numerical models over the CSDMS Basic Model Interface (BMI).
- `bmi-rs-sys`: Bindgen bindings for the CSDMS Basic Model Interface (BMI) C spec.

[unreleased]: https://github.com/aaraney/bmi-rs/releases/tag/v0.0.1-alpha.0...HEAD
[v0.0.1-alpha.0]: https://github.com/aaraney/bmi-rs/releases/tag/v0.0.1-alpha.0
