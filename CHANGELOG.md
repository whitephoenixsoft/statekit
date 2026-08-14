# Changelog

All notable changes to Statekit are documented in this file.

## [Unreleased]

## [0.2.0]

### Added

- Added `MachineBuilder::try_allow` for fallible transition construction.
- Added validation for leading and trailing whitespace in state names.
- Added `StateError::AmbiguousStateName`.
- Added `Machine::sources` for iterating states with outgoing transitions.
- Added `Machine::states` for iterating in unique states contained within both sources and targets.

### Changed

- State names are validated when transitions are added rather than during `build`.
- Internal machine storage now uses validated state-name domain types.
- Added `Machine::targets` to replace `Machine::targets_from`.

### Deprecated

- Deprecated `MachineBuilder::allow` in favor of `MachineBuilder::try_allow`.
- Deprecated `MachineBuilder::targets` in favor of `MachineBuilder::targets_from`.

### Documentation

- Added the Statekit specification.
- Added a migration guide.

## [0.1.0]

### Added

- Initial Statekit release.
- Immutable state-machine definitions.
- Builder-based machine construction.
- Transition validation.
- State and transition inspection.