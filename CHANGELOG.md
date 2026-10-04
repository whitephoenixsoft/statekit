# Changelog

All notable changes to Statekit are documented in this file.

## [Unreleased]

## [0.4.0]

### Added

- Added `MachineInstance` for stateful traversal of a `Machine` definition through `Machine::instance()`. Instances maintain independent traversal state and are thread-safe. See [MIGRATION.md](MIGRATION.md) for details.
- Added `StateError::UnknownInitialState` for invalid instance construction.
- Added `Machine::is_terminal(state)` to determine whether a state has no outgoing transitions.
- Added memory benchmarks alongside the existing performance benchmarks. All benchmarks can be run with `cargo bench`.
- Added v0.4 performance and memory benchmark baselines.

### Documentation

- Updated the README and API documentation with `MachineInstance` usage examples.
- Updated benchmark documentation to cover instance operations and memory measurements.
- Added a Project Core document defining Statekit's project intent, invariants, and architectural decisions.
- Added a unit-test outline, mapping project invariants to unit and integration test evidence.
- Updated the property-test outline to map property tests to project invariants.
- Refined the specification to focus on domain requirements and behavioral contracts.

## [0.3.1]

### Added

- Added property-based testing with `proptest`; property tests run with the existing test suite using `cargo test`.
- Added Criterion benchmarks for machine construction and query operations; run them with `cargo bench`.

### Documentation 

- Added a property test outline describing the property-test coverage.
- Added benchmark baseline documents, showing performance for v0.3. 
- Added acknowledgements for ecosystem crates and linked the benchmark documentation from the README.
- Updated `lib.rs` documentation to remove version-specific GitHub documentation links.
- Updated `MIGRATION.md` for clarity.

## [0.3.0]

For breaking changes, [Please see MIGRATION.md](MIGRATION.md) for details.

### Added

- Added the public `Transition` type for representing directed transitions with `source()` and `target()` accessors.
- Added `Machine::transitions()` for iterating over all transitions in the machine.

### Changed

- Refactored internal transition storage around validated `Transition` values.
- Changed `Machine::targets_from()` to return an iterator directly instead of an optional iterator. Sources with no outgoing transitions now produce an empty iterator. This is a breaking change.
- Changed `StateError::AmbiguousStateName` to include the offending state name in a `state` field. This is a breaking change.
- Changed `StateError` display messages to quote state values with double quotes instead of backticks.
- Removed the `Default` and `PartialEq` implementations from `MachineBuilder`. Machine construction should use `Machine::builder()`, and builder equality is no longer part of the public API. This is a breaking change.
- Improved `StateError` display messages, including clearer reporting of whitespace-related state-name errors and double-quoted diagnostic values.

### Documentation

- Updated specification to include updated domain responsibilities.
- Updated README examples and API documentation for transition inspection.

## [0.2.0]

### Added

- Added `MachineBuilder::try_allow` for fallible transition construction.
- Added validation that rejects state names with leading or trailing Unicode whitespace.
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
