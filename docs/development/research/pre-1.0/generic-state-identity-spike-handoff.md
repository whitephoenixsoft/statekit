# Generic State Identity Compatibility Spike

**Status:** Experimental; not a production API decision

**Branch:** `research/generic-state-identity`

**Baseline commit:** `78c1d7490dc64d36cc8801e64fbf833e2b8c85b3`

**Local spike commit:** Unpublished; see the local branch history.

**Remote status:** The branch exists remotely at the baseline SHA. Publishing
was blocked because Git push had no write credentials and GitHub blob and
contents writes returned 403 (`Resource not accessible by integration`).

**Compiler status:** Not run; this environment has no `cargo`, `rustc`, `rustfmt`, or `rust-analyzer`.

## 1. Executive summary

The local research branch adds a separately namespaced generic machine API at
`statekit::generic_identity`. It keeps the existing crate-root `Machine`,
`MachineBuilder`, `MachineInstance`, `Transition`, and `StateError` unchanged.
The new module exercises custom enum and numeric identities, borrowed topology
inspection, immutable shared definitions, independent instances, and typed
structural errors.

This is the **additive generic alternative**, not a replacement of the root
`Machine` with `Machine<S = StateName>`. It uses `Machine<S = String>` inside
the module because `StateName` remains a private validated implementation
type. The string path retains Statekit name validation. Custom identities use
`Eq + Hash`; no string validation is imposed on them. The generic builder and
instance operations use explicitly named methods (`builder_with_identity`,
`try_allow_identity`, and `transition_to_identity`) to avoid overload and
inference collisions with the existing string methods.

The root API is source-compatible by construction in the diff: its public
signatures and implementation files are untouched, and original tests were
not edited, disabled, or deleted. This is not compiler-verified. No tests or
rustdoc examples could be executed because the Rust toolchain is absent.

**Preliminary recommendation:** Keep this branch as an API experiment only.
The additive module is the safer prototype shape if a concrete custom-ID
consumer proves value. Do not promote it until external compilation confirms
the examples and methods, and a consumer demonstrates a meaningful reduction
in conversion or adapter work. This spike does not establish that the root
`Machine` should become generic.

## 2. Research continuity and authority

The previous six reports found that Statekit is a small structural validator
and runtime, that its current immutable `Machine` is useful for configuration
validation and reference-model tests, and that no consumer had yet established
a need for public generic identity. The seventh investigation specifically
identified the broad compatibility surface: builder, transition, instance,
iterators, errors, inference, and equality all participate in a generic
design. It favored the existing string API and suggested a separately named
generic structural API only if a real consumer benefits.

This spike follows the supplied Project Core v0.4 and repository copies of the
specification and roadmap. The roadmap lists generic `Machine<S>` as an
alternative to investigate in v0.5, not an accepted change. v0.6 indexing and
optimization remain separate. No official Project Core, specification, or
roadmap files were changed.

### Evidence labels

- **Verified source fact:** Read from the baseline Statekit or VPE source.
- **Prototype behavior:** Implemented in `src/generic_identity.rs`, but not
  compiler-verified in this environment.
- **Interpretation:** An architectural conclusion from the source and API
  shape.
- **Unverified:** Requires Rust compilation, tests, benchmarks, or another
  consumer.

## 3. Baseline and repository result

The remote research branch was created from the same commit as `main`:

```text
main                         78c1d7490dc64d36cc8801e64fbf833e2b8c85b3
research/generic-state-identity 78c1d7490dc64d36cc8801e64fbf833e2b8c85b3
```

The local clone contains an experimental commit based on that start point. It
has not been pushed, so the remote research branch remains at the baseline.

The baseline checkout was clean. This spike does not change VPE. The original
Statekit implementation and test files are preserved. Changes are limited to
the experimental module export, its implementation and tests, an independent
consumer fixture, and this handoff.

## 4. Verified current Statekit API

At the baseline, `src/lib.rs` re-exports these non-generic public types:

| Type | Baseline behavior |
|---|---|
| `Machine` | Immutable definition backed by `Arc<MachineInner>`; `Debug`, logical `PartialEq`/`Eq`. |
| `MachineBuilder` | Consuming builder; `try_allow(self, from: impl AsRef<str>, to: impl AsRef<str>) -> Result<Self, StateError>`; `build(self) -> Result<Machine, StateError>`. |
| `MachineInstance` | Mutable current-state value plus a shared immutable machine; derived equality. |
| `Transition` | Borrowed public transition record; `source()` and `target()` return `&str`. |
| `StateError` | Non-generic `thiserror` enum; standard error integration and string payloads. |

Baseline `Machine` methods include `builder()`, `instance(&str)`,
`can_transition(&str, &str)`, `validate_transition(&str, &str)`,
`transition_count()`, `contains_state(&str)`, `targets_from(&str)`,
`sources()`, `states()`, `transitions()`, and `is_terminal(&str)`. Public state
and transition enumeration borrows from the machine; iterator ordering is
unspecified. `MachineInstance` exposes `state() -> &str`,
`can_transition_to(&str)`, `transition_to(&str)`, `is_terminal()`, and
`machine()`.

Baseline `StateError` variants are `NoTransitions`, `EmptyState`,
`AmbiguousStateName { state: String }`, `SelfTransition { state: String }`,
`InvalidTransition { from: String, to: String }`, and
`UnknownInitialState { state: String }`.

The baseline uses private `StateName(String)` values to validate construction
names: empty and whitespace-only values fail; leading or trailing Unicode
whitespace fails; names are otherwise exact and case-sensitive. Transition
membership is logically the `(source, target)` pair. Duplicate pairs collapse.
The builder fails on the offending insertion and consumes itself, so the caller
cannot reuse that failed builder. `build()` rejects an empty transition set.
Machine equality compares the logical edge set, not allocation identity or
insertion order. Instance equality follows equal machine definitions plus
equal current states. Failed traversal validates before modifying the current
state.

## 5. Experimental generic API

The module is additive and explicitly marked experimental in rustdoc:

```rust
use statekit::generic_identity::Machine as IdentityMachine;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Phase { Idle, Running, Done }

let machine = IdentityMachine::<Phase>::builder_with_identity()
    .try_allow_identity(Phase::Idle, Phase::Running)?
    .try_allow_identity(Phase::Running, Phase::Done)?
    .build()?;

let mut instance = machine.instance_identity(Phase::Idle)?;
instance.transition_to_identity(Phase::Running)?;
assert_eq!(instance.state_identity(), &Phase::Running);
```

This code is illustrative and **not verified as compiling** here. The same
module's string specialization is designed to retain the familiar validation
flow:

```rust
let machine = IdentityMachine::builder()
    .try_allow("idle", "running")?
    .try_allow("running", "done")?
    .build()?;
```

The stable crate-root version of that example is unchanged and remains the
recommended API for configuration strings.

### Prototype signatures and choices

- `generic_identity::Machine<S = String>` stores an immutable shared
  `HashSet<Transition<S>>`.
- `Machine<String>::builder()` provides string name validation through the
  existing private `StateName` implementation.
- `Machine<S>::builder_with_identity()` plus
  `MachineBuilder<S>::try_allow_identity(S, S)` accepts custom owned IDs.
- `MachineBuilder<S>::build()` rejects an empty edge set.
- Custom machine queries use explicit `_identity` names. `states_identity`,
  `sources_identity`, `targets_from_identity`, and `transitions` borrow from
  stored topology.
- `MachineInstance<S>` stores its current `S` and shares the immutable edge
  set. The custom transition method takes an owned target; it requires `Clone`
  only to return both IDs in a failed-transition error.
- `IdentityError<S>` carries typed custom IDs for self-transition, invalid
  edge, and unknown initial state. It uses `Debug` formatting for those values.
  The `std::error::Error` implementation is conditional on `S: Debug +
  'static`; construction and ordinary queries do not require `Debug`.
- `Eq + Hash` are required to store states in the prototype's `HashSet`.
  The caller's identity must keep its `Eq` and `Hash` behavior stable while
  stored. The prototype does not require `Display`, `Clone`, `Send`, or `Sync`
  globally.

The prototype does not add indexes or claim a performance improvement. It uses
a hash set and scans edges for membership, similar in broad shape to the
baseline storage. The identity values themselves are retained in transitions;
no name-to-ID mapping or lowering is performed.

## 6. API compatibility ledger

All original test/source files are unchanged. The only root source edit adds a
public module declaration; the root exports and method signatures remain the
same. Compilation is unverified, so "compatible by diff" is not a passing
compiler result.

| Existing element or behavior | Classification | Evidence and caveat |
|---|---|---|
| `Machine::builder()` | Unchanged | Root `Machine` and its implementation are untouched. |
| `Machine::builder().try_allow(&str, &str)?.build()?` | Unchanged by source diff | Exercised in the added external fixture; not compiled. |
| `Machine::instance(&str)` and instance transitions | Unchanged | Root implementation remains untouched. |
| `states()`, `sources()`, `targets_from(&str)` | Unchanged | Borrowed root iterator signatures remain untouched. |
| `transitions() -> Iterator<Item=&Transition>` | Unchanged | Root `Transition` remains non-generic. |
| `StateError` variants and payloads | Unchanged | No changes to `src/error.rs`. |
| Root machine equality and instance equality | Unchanged | No edits to baseline type implementations. |
| Existing `tests/*` and in-source tests | Unchanged | No baseline test was edited, deleted, commented out, or disabled. |
| New `generic_identity::Machine<S>` | Additive, unverified | Separately namespaced; its own inference, methods, and error behavior need compilation. |
| `generic_identity::IdentityError<S>` | Additive, unverified | Separate error type; does not alter root `StateError`. |

### Original test inventory

The baseline contains test code in `src/builder.rs`, `src/error.rs`,
`src/inner.rs`, `src/instance.rs`, `src/machine.rs`, `src/state_name.rs`,
`src/transition.rs`, `src/transitions.rs`, `tests/property_tests.rs`, and
`tests/transition_tests.rs`. A static count found 216 existing `#[test]`
attributes across those files. Property macros and doc tests are additional
test forms. All remain unchanged. Seven focused tests and one property test
were added in `tests/generic_identity_tests.rs`; the separate consumer fixture
adds a standalone binary crate. None have been executed.

## 7. Primary proposal versus additive alternative

| Question | Root `Machine<S = StateName>` replacement | Additive module implemented here |
|---|---|---|
| Existing root type path | Generic default type changes its public identity | Root `Machine` stays non-generic. |
| `StateName` visibility | Must become publicly nameable or use another default | Remains private; experimental default is `String`. |
| Existing builder inference | Must be proven for `Machine::builder()` and generic inputs | Root builder stays exactly as before; module has a dedicated string specialization and a separate custom-ID builder name. |
| Error contract | Generic payloads or type-dependent errors affect existing API | Root `StateError` stays unchanged; module adds `IdentityError<S>`. |
| Iterators | `Transition<S>` propagates through public borrowed iterators | Root iterator item remains the existing `&Transition`; generic module yields `&Transition<S>`. |
| Implementation cost | Genericizes all root storage and runtime types | Adds a separate small implementation with duplicated topology/runtime logic. |
| Consumer value | Potentially one unified type name | Explicit distinction is discoverable but requires selecting a module/type. |

The module demonstrates that custom IDs can avoid conversion to strings while
preserving the current root API. Its cost is a parallel implementation. This
does not establish that the generic core can be shared cleanly with the root
types; doing so is the next design question if a second consumer validates the
need. The `Machine<S = StateName>` root proposal itself was not applied, and
Rust inference compatibility was not compiled in this environment.

## 8. Project Core impact

The statuses below evaluate the additive module. The root API remains unchanged.

### Intents and durable boundaries

| Project Core item | Assessment |
|---|---|
| I-001 Runtime Use | Preserved; the experimental instance traverses a built machine. |
| I-002 Small and Fast | Potential tension: a parallel public model adds code and API surface; no speed claim or benchmark is made. |
| I-003 Ease of Use | Root ease is preserved. Advanced consumers get direct IDs, with extra method/type names and a separate error type. |
| I-004 Engineering Reference | Supports the intent through external consumer and invariant tests, pending execution. |
| I-005 Data-Driven Definition | String specialization preserves validated runtime definitions; typed IDs suit already-resolved definitions. |
| BI-001 Building Block | Preserved; no workflow or semantic behavior is added. |
| BI-002 Hidden Internal Storage | Root internals stay hidden. Generic module's borrowed `Transition<S>` is public logical topology, not a numeric index or storage key. |
| BI-003 Concurrency | Host scheduling remains out of scope. `Send`/`Sync` follow `S` and the contained standard-library types; the generic module does not force them. |

### Invariants

| Invariant | Assessment for experimental module |
|---|---|
| INV-001 Machine Immutability | Preserved after build; topology is behind private `Arc<HashSet<_>>`. |
| INV-002 No Self Transitions | Checked before inserting custom edges; validated names are compared for string edges. |
| INV-003 Non-Empty Transition Set | Checked by `build()`. |
| INV-004 Logical Transition Uniqueness | HashSet edge equality collapses same ordered pair under `S::Eq`/`Hash`. Requires the normal `Eq`/`Hash` consistency and stability contract. |
| INV-005 Case-Sensitive State Identity | Preserved for String names. For arbitrary `S`, identity follows `Eq`; string case sensitivity does not generalize. Clarification would be needed before generic identity becomes the primary contract. |
| INV-006 Runtime Configuration | Preserved for String specialization; custom IDs are an additive path. |
| INV-007 Earliest Valid Error Detection | Self-edge and invalid string names fail during insertion; empty edge set fails during build; missing instance/transition fails at call. |
| INV-008 Standard Error Integration | `IdentityError<S>` implements `Error` only when `S: Debug + 'static`; that conditional may be too restrictive for some consumers. Root `StateError` is unchanged. |
| INV-009 Shared Immutable Machine | Instances clone the `Arc` to one immutable edge set. |
| INV-010 Valid Traversal | Instance checks edge membership before changing its current identity. |
| INV-011 Safe Traversal Failure | Failed transition returns before assigning the new current state. |
| INV-012 Independent Instances | Current identity is stored per instance. |
| INV-013 Machine Equality | Derived equality compares logical edge sets, independent of insertion order and Arc identity, for the same `S` type. No cross-type equality is added. |
| INV-014 Instance Equality | Derived equality compares equal edge sets and equal current identities for the same `S` type. |

No invariant or official document was changed. The generic interpretation of
INV-005 is a specification question, not resolved here.

### Established decision implications

| Decision | Assessment |
|---|---|
| D-001 Declarative definition | Preserved; the experimental builder completes an immutable definition. |
| D-002 Builder and Machine as primary APIs | Root pair stays primary and unchanged; the generic module is explicitly secondary. |
| D-005 External transition API testing | Added a path-dependent consumer fixture, but it is not compiled. |
| D-006 Fail during construction when possible | Preserved in the prototype: name and self-edge errors occur during insertion; empty topology fails at build. |
| D-007 Property testing with `proptest` | One model-comparison property was added; execution is unverified. |
| D-008 Criterion benchmarking | No benchmark was added or run; the spike makes no performance claim. |
| D-009 Memory benchmarking | No retained-memory measurement was added; generic identity is not treated as indexing. |
| D-010 Mutable instances over shared machines | Preserved in shape with per-instance current `S` and shared `Arc` topology. |
| D-011 Performance tradeoff | No tradeoff is accepted without Criterion and memory measurements. |
| D-012 Runtime failures | Structural operation failures are represented as errors; `IdentityError<S>` standard-error support is conditional on `Debug + 'static`. |

## 9. Consumer integration observations

### Configuration-driven application

The root string builder remains the smallest path: no annotations or adapters.
The experimental module also has a string-specialized builder but creates a
parallel error type and repeats a small part of the façade. It does not show a
configuration consumer needs generic identity.

### Model-based testing

An enum or newtype model can use direct typed states and compare transition
sequences without string conversions. The experimental machine does not
generate operations, shrink failing sequences, report coverage, or provide a
test framework. Those remain responsibilities of the consumer's test harness.
No test-runner integration is included.

### VPE-style compiled consumer

The inspected VPE `CompiledProcess` stores `initial_state_idx: usize`,
`nodes: Vec<Node>`, and `state_index: HashMap<String, usize>`. A node keeps its
name and a vector of edges; each VPE edge carries `action`, `target_idx`,
`priority`, guards, and effects. The compiler resolves names to indexes. The
runtime also accepts string state values and looks up their index.

`Machine<usize>` could check a projection of structural endpoint pairs without
converting indexes to names. It would not represent VPE's semantic transition
identity: different actions, guards, priorities, or effects can share the
same endpoints, while Statekit intentionally collapses duplicate logical
edges. Statekit's no-self-edge and non-empty-machine invariants also do not
match every possible VPE process. AUTO_TICK cycle detection is filtered by
action and belongs to VPE semantics; ordinary Statekit topology cannot replace
it. No VPE source was modified and no VPE integration is claimed.

The external fixture demonstrates direct enum IDs and the unchanged root
string path. It does not yet measure adapter lines, allocations, or VPE
integration because it was not compiled or instrumented.

## 10. Compatibility experiment results

### Executed checks

- Baseline `git status` was clean at the recorded commit.
- `git diff --check` passed after the edits.
- A local commit was created. Git push had no write credentials, and GitHub
  blob/contents writes were rejected with 403; the remote branch remains at
  the baseline commit.
- Static inspection confirms no existing root implementation or original
  test file was modified.
- Source inspection found 216 existing `#[test]` attributes; seven new
  example-based integration tests and one property test were added.

### Unexecuted Rust checks

Every Rust command stopped before compilation because `cargo` is not installed
(`command not found`, exit 127):

```text
cargo fmt --check
cargo check --all-targets
cargo test
cargo test --doc
cargo test --all-features
cargo run --manifest-path research/generic-identity-consumer/Cargo.toml
cargo check --manifest-path research/generic-identity-consumer/Cargo.toml
```

Therefore: no successful compilation, formatting, unit, property, integration,
doc-test, all-feature, or external-consumer result is claimed. The generic
default fallback in `IdentityMachine::builder()`, generated trait bounds for
`IdentityError<S>`, and lifetime ergonomics of borrowed iterators remain
compiler-dependent questions.

The local commit is reviewable in the accompanying patch. The remote research
branch still needs the patch applied after integration write access is
available.

## 11. Testing additions and compatibility record

The seven added example-based integration tests cover:

1. Generic module string construction plus original crate-root string
   construction and borrowed `targets_from` results.
2. Enum identity, self-transition rejection, and duplicate collapse.
3. Numeric newtype identity and borrowed transition iteration.
4. Logical machine equality across insertion orders.
5. Independent instances and failed-transition atomicity.
6. Topology use with an identity that does not implement `Debug`.
7. Empty-machine rejection and unknown initial custom identity.

One `proptest` property compares generic transition membership with a
separate edge-set model over generated numeric state pairs, including duplicate
input edges and ignored generated self-edges.

All original tests remain byte-for-byte unchanged in this branch. No original
test required adaptation. This preserves traceability and allows the baseline
test suite to serve as the unchanged API compatibility suite once Rust is
available.

## 12. Risks and known incomplete work

- This module duplicates topology and runtime mechanics rather than
  demonstrating a shared generic core behind the root façade.
- It is not the proposed root-level `Machine<S = StateName>` API. The primary
  proposal's exact associated-function inference and migration impact remain
  unverified.
- The additive module's separate error type changes `?` conversions when a
  caller mixes root and experimental APIs.
- Formatting generic IDs with `Debug` is conditional; a production design
  should decide whether typed errors, opaque errors, or caller formatting is
  preferable.
- `Eq` and `Hash` stability is a caller obligation. Identity types with
  interior mutability that changes equality/hash behavior are unsafe to retain
  in this HashSet-based prototype.
- Generic machines are only comparable when they have the same `S` type.
- VPE parallel semantic transitions, self-edges, empty topologies, guards,
  actions, and filtered cycle analysis are not modeled by this structural API.
- No runtime, retained-memory, compile-time, or binary-size measurements were
  possible.
- This repository includes no `.github` workflow files in the inspected tree;
  no hosted CI result is available as a substitute for local Rust tooling.

## 13. Commands for local validation

From the Statekit repository root on this branch:

```sh
cargo fmt --check
cargo check --all-targets
cargo test
cargo test --doc
cargo test --all-features
cargo run --manifest-path research/generic-identity-consumer/Cargo.toml
cargo check --manifest-path research/generic-identity-consumer/Cargo.toml
```

For the first compiler pass, start with `cargo check --all-targets`, then send
the first compiler diagnostic and its surrounding source location. The
independent fixture is a path-dependent crate and exercises only public exports.

## 14. Preliminary recommendation and decision gates

**Recommendation: prefer an additive generic structural API for further
investigation, while retaining the crate-root string API.** The experiment
shows a plausible separation of default string validation from consumer-ID
construction without changing root signatures. It does not yet show that the
parallel implementation cost is acceptable or that direct IDs materially
help VPE.

Before considering a root generic `Machine<S = StateName>`:

1. Compile every unchanged original test and README/rustdoc example.
2. Compile the external fixture and at least one actual consumer adapter.
3. Decide whether public generic errors should require `Debug` for standard
   error formatting.
4. Confirm that borrowed iteration remains convenient without unnecessary
   `Clone` bounds.
5. Measure added code/API burden against a consumer-owned ID-to-name mapping.
6. Keep generic identity separate from v0.6 indexing benchmarks.

Abandon the generic proposal if ordinary string examples need annotations or
method-name changes, if errors/iterators impose disproportionate trait or
ownership burdens, or if no real consumer benefits beyond a small mapping
adapter. A successful compile alone is not architectural approval.

## 15. Suggested next actions

1. Run the exact command set above in a Rust 1.88+ environment and fix only
   prototype errors or clearly recorded experimental defects.
2. Review the first diagnostic from the compiler before making broader API
   changes; do not adapt baseline tests merely to hide incompatibilities.
3. Compare this additive module with a prototype that changes only a local
   copy of root `Machine` to `Machine<S>`; specifically test default generic
   inference for the exact README chain.
4. Build a test-only VPE projection over a subset that has no self-edges,
   retains at least one edge, and collapses parallel semantic edges
   intentionally. Measure adapter code before considering production use.
5. Decide separately whether a shared private generic core can reduce code
   duplication without exposing identity or storage details.

## Primary source files

- Statekit source at baseline commit: `src/lib.rs`, `src/builder.rs`,
  `src/state_name.rs`, `src/transition.rs`, `src/machine.rs`,
  `src/instance.rs`, `src/inner.rs`, `src/transitions.rs`, and `src/error.rs`.
- Statekit constraints: `docs/project-core.md`,
  `docs/statekit-specification.md`, and `docs/roadmap.md`.
- VPE representation: `crates/vpe/src/compiler/compiled.rs`,
  `crates/vpe/src/compiler/compile.rs`,
  `crates/vpe/src/compiler/validate.rs`, and
  `crates/vpe/src/runtime/evaluate.rs`.
