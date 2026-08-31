# Changelog

All notable changes to Statekit are documented in this file.

## [Unreleased]

### Added

- Added `Transition` for storing the source and target transitions.
- Added `Machine::transitions` for iterating over all transitions in the machine.

### Changed

- Changed internal storage changes for transitions.
- Changed return type of `Machine::targets_from` from `Option<Iterator<&str>>` to `Iterator<&str>`. This makes the iterator act more in line with rust best practices.
- Changed `StateError::AmbiguousStateName` to include field name `state`.
- Changed StateError displayed errors to quote values with a double quote (\") instead of a back tick (\`)
- Changed `MachineBuilder` to no longer implement unnecessary trait: `Default`.

### Documentation 

- Updated specification to include updated domain responsibilities.
- Updated README to include new API changes.

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
