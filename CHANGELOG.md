# Changelog

All notable changes to Statekit are documented in this file.

## [Unreleased]

### Documentation 

- Updated specification to include Transitions domain model responsibilities.
- Updated README to include new API for inspecting transitions.

## [0.2.0]

### Added

- Added `MachineBuilder::try_allow` for fallible transition construction.
-  Added validation that rejects state names with leading or trailing Unicode whitespace.
- Added `StateError::AmbiguousStateName`.
- Added `Machine::sources` for iterating states with outgoing transitions.
- Added `Machine::states` for iterating over all unique states in the machine.
- Added `Machine::targets_from` as a clearer transition-target query.

### Changed

- State names are validated when transitions are added rather than during `build`.
- Internal machine storage now uses validated state-name domain types.
- `MachineBuilder::allow` now validates transitions immediately and panics on invalid input.

### Deprecated

- Deprecated `MachineBuilder::allow` in favor of `MachineBuilder::try_allow`.
- Deprecated `Machine::targets` in favor of `Machine::targets_from`.

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