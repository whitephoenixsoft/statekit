# Statekit — Validation Diagnostics & Error Accumulation Research

**Status:** Research proposal; no architectural decisions are made here  
**Investigation:** Fifth report in Statekit Consumer Research & Capability Discovery  
**Repositories reviewed:** `whitephoenixsoft/statekit` and `whitephoenixsoft/verified-process-engine`, default branch `main`  
**Research date:** 2026-10-13 (UTC)

## Executive summary

**Finding:** Statekit's current fail-fast builder is a good fit for programmatic construction and preserves a strong invariant: a returned `Machine` is immutable and structurally valid. It reports one structural error at the point an edge is proposed, and the separate `build()` check rejects a definition with no transitions. `StateError` is typed, implements Rust's standard error interface, and retains offending values for most—but not every—error category.

Consumers can collect several edge errors today, but not by repeatedly calling `try_allow` on one builder: `try_allow` consumes the builder, and an error returns no builder to continue with. A host can validate each candidate edge with a temporary builder, attach its own row/path/location context, collect errors in input order, then build the complete machine only if no errors were found. That uses Statekit's rules rather than copying them, but revalidates valid edges during the final build and allocates temporary builders. An editor can use the same pattern for a save-time pass while retaining fail-fast feedback for one edit.

**Recommendation requiring no API change:** Document this consumer-owned accumulation pattern, including the consuming-builder behavior, how to attach source context, and why duplicate endpoint pairs remain idempotent rather than errors. This may address common configuration and editor needs without changing INV-007 or making the builder partially valid.

**Possible future addition:** If multiple independent consumers demonstrate that the temporary-builder/revalidation pattern is too costly or awkward, investigate a separate batch validation/construction API. Keep the existing builder fail-fast. A batch API should return a `Machine` only when all required structural checks succeed, report errors in input order, keep provenance as an input ordinal at most, and not add source locations, severities, or consumer policy. Its delayed return of accumulated diagnostics needs an explicit contract against INV-007 before adoption.

**VPE evidence:** VPE's compiler currently returns the first compile error as a `CompileError`, while collecting warnings separately as `Vec<String>`. Its source structs do not carry spans or record locations. VPE validates additional process semantics, so Statekit could at most contribute an edge-structure error; it cannot replace VPE's compiler diagnostics or location mapping. No evidence establishes repeated cross-consumer demand for a Statekit-owned diagnostic framework.

This report distinguishes **verified behavior**, **interpretation**, and **hypothesis**. It does not recommend changing the roadmap or making any observation such as disconnectedness a construction error.

## Continuity with the previous four investigations

The earlier reports established that:

- Statekit is a structural validation building block, not a parser, compiler framework, policy engine, or workflow engine.
- `MachineBuilder` checks edge-local invariants during addition; a successfully built `Machine` is immutable and valid.
- Statekit can already serve as a structural reference model and as a validated projection of richer consumer definitions.
- Consumer-owned source formats and serialization remain preferable; Statekit sees endpoint strings, not the source document or its locations.
- Structural facts such as reachability and cycles should remain observations unless a consumer applies its own policy.
- VPE is a useful concrete consumer, but owns actions, guards, priorities, effects, process rules, and compiler context.
- Earlier investigations did not establish a requirement for a new Statekit API. Accumulated diagnostics were identified as an open question, not a settled gap.
- The roadmap already places runtime identity/lowering in v0.5 and indexing/representation optimization in v0.6. This report does not propose moving or replacing either phase.

Prior reports: [configuration-driven consumer research](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md), [reference-model research](sandbox:/workspace/scratch/3524c4ebe062/statekit-reference-model-research.md), [structural IR and interchange research](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-ir-interchange-research.md), and [structural analysis research](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-analysis-algorithms-research.md).

## 1. Current Statekit validation contract

### Evidence reviewed

The supplied Project Core, specification, README, roadmap, and frozen v0.4 benchmark baseline were reviewed alongside current `main` source and tests. The relevant source includes `src/builder.rs`, `src/error.rs`, `src/state_name.rs`, `src/transition.rs`, `src/machine.rs`, `src/inner.rs`, `src/transitions.rs`, `tests/transition_tests.rs`, and the property tests. The baseline is treated as historical evidence and was not altered. Its graph workloads measure valid construction and runtime/query operations; they do not measure invalid-input diagnostic accumulation.

### Verified rules and timing

| Rule or behavior | First detectable point | Current return point | Error information |
|---|---|---|---|
| Empty or whitespace-only endpoint | `StateName::try_from` while adding an edge | Immediately from `try_allow` as `StateError::EmptyState` | Unit variant; does not retain whether source or target was empty, nor the offending text. The consumer still has its input record. |
| Leading/trailing Unicode whitespace in endpoint | `StateName::try_from` while adding an edge | Immediately from `try_allow` as `AmbiguousStateName { state }` | Retains the exact offending state string, but not whether it was source or target. |
| Self-transition | After both endpoint names pass validation | Immediately from `try_allow` as `SelfTransition { state }` | Retains the state name. |
| Exact duplicate source/target pair | During insertion into `Transitions` | No error; duplicate has no additional effect | Consistent with INV-004 and specification: duplicate logical transitions collapse. |
| Empty completed definition | At `build()` | `StateError::NoTransitions` | Unit variant. It is a whole-definition error, not an edge-local error. |
| Disallowed runtime move | When `validate_transition` or instance transition is called | Immediately from that runtime/query operation as `InvalidTransition { from, to }` | Retains both endpoints. This is not a builder diagnostic. |
| Unknown instance initial state | At `Machine::instance(initial)` | Immediately as `UnknownInitialState { state }` | Retains the requested state. This occurs after a valid machine exists. |

State name identity is exact and case-sensitive. Names are not trimmed or normalized; leading/trailing whitespace is rejected. Endpoint validation occurs source first, then target, then self-transition comparison. Therefore, if both endpoints are invalid, the current builder returns only the source-side error first. Statekit does not represent states that are not incident to an edge.

`StateError` derives `thiserror::Error`, `PartialEq`, and `Eq`; it implements `std::error::Error` and `Display`. Most variants carry useful values, but `EmptyState` and `NoTransitions` do not. There is no diagnostic collection, severity, source span, record identifier, error code, or related-location field.

### Builder recovery and side effects

`MachineBuilder::try_allow(self, ...) -> Result<Self, StateError>` takes ownership of the builder. It first constructs and validates a `Transition`, and only then inserts it. On an error, insertion has not happened, but the builder value has been consumed and is unavailable to the caller. `MachineBuilder` is not `Clone`, and its constructor is crate-private; normal external callers start with `Machine::builder()`.

Consequences:

- A successful `try_allow` can be chained while all additions succeed.
- A failure does not mutate a builder that the caller can inspect or repair; it returns no builder at all.
- The caller cannot continue a single builder after one bad transition using only the existing method.
- `build(self)` also consumes the builder. On `NoTransitions`, it returns no reusable builder.
- There are no observable partial insertions from an invalid `try_allow`; valid transitions from earlier calls are dropped with the moved builder if a later call fails.

This is an API behavior in addition to the conceptual INV-007 requirement. It provides simple fail-fast construction, but it is not a recoverable editor/session API.

### Project Core and specification mapping

| Project Core / specification point | Current behavior | Diagnostic implication |
|---|---|---|
| INV-001: built machine immutable | `Machine` can only be built from validated transitions and exposes queries | Accumulation must not expose a partially built `Machine`. |
| INV-002: no self-transitions | Checked before edge insertion | In a batch scan, self-edge diagnostics depend on both endpoint names first being valid. |
| INV-003: non-empty transition set | Checked on `build()` | Empty candidate input can produce `NoTransitions`; a nonempty batch with all-invalid entries may make that a redundant cascade. |
| INV-004: logical uniqueness | Exact duplicate endpoints collapse; they are not invalid | A batch validator must not silently turn duplicates into errors without changing the invariant/API contract. A host may impose a separate duplicate-record policy. |
| INV-005: case-sensitive identity | Preserved during validation | Diagnostic comparisons must not case-fold or normalize endpoints. |
| INV-006: runtime configuration | Builder accepts data at runtime | A batch API is possible, but parser/schema ownership remains with the host. |
| INV-007: earliest valid error detection | Edge-local errors are returned by `try_allow`; completed-definition error at `build()` | A batch scan may detect errors early but only return the aggregate after scanning. That is a different reporting time and needs an explicit contract. |
| INV-008: standard error integration | `StateError` implements `std::error::Error` | Consumer wrappers can retain StateError as an error source without Statekit adopting a rendering dependency. |
| D-002: builder and machine are primary APIs | Current construction is builder to immutable machine | A second validation API should be additive and avoid making `MachineBuilder` a partially valid diagnostic accumulator by default. |

“Earliest detectable” and “earliest returned to a caller” are distinct. In the current API they coincide for each failed addition because the error is returned immediately. An aggregate API can preserve immediate *detection* during iteration but returns the report after the batch finishes. Whether that satisfies INV-007 depends on whether the invariant governs detection or caller-visible reporting; the current wording needs clarification if such an API is considered.

## 2. Validation strategies

| Strategy | What it does | Preserves valid-machine guarantee? | Main tradeoff |
|---|---|---|---|
| **A. Existing fail-fast builder** | Chain `try_allow`; first invalid edge returns `StateError`; call `build()` after successful additions. | Yes. The only returned `Machine` comes from successful `build()`. | Small, direct, efficient for trusted/programmatic data. Stops at first edge failure and the consumed builder cannot recover. |
| **B. Consumer-owned accumulation** | Validate each candidate edge through a temporary builder, wrap errors with input context, retain accepted pairs, and build all pairs only if no diagnostics exist. | Yes, if the host does not publish a machine when diagnostics exist. | Uses existing Statekit validation, but validates valid edges twice on the successful final path and allocates temporary builders. Host controls source context and policy. |
| **C. Batch validation/construction** | Statekit accepts a sequence and returns either a machine or an ordered error collection. | Yes, if `Machine` is produced only when every required check passes. | Avoids host-side validation duplication and can make ordering explicit, but defines aggregation, provenance, duplicate, empty-input, and INV-007 semantics. |
| **D. Separate validation pass then construction** | A validation function inspects candidate definitions; a later builder constructs the machine. | Yes, if construction still validates and errors cannot be bypassed. | Duplicated checks or two passes over data. Shared internal validation avoids logic drift, but if validation returns a “validated token,” that becomes another public construction type. |
| **E. Diagnostic-aware builder** | Builder retains valid partial edges and an internal list of errors while additions continue; finalization refuses a machine if errors remain. | Yes, if finalization returns no `Machine` when errors exist. | Introduces a partially valid builder state and error-retention policy into the primary builder API; callers must understand accumulated and finalization behavior. INV-007 tension and API surface are substantial. |

**Assessment:** A remains the right default. B demonstrates that aggregation is possible without weakening the machine invariant, but its cost and awkwardness should first be tested against actual consumers. C could be a useful optional API if more than one consumer needs the same host-side pattern. D and E are more complex and have no demonstrated advantage over C for this flat topology.

## 3. Error accumulation and dependency semantics

“Collect errors” should mean collecting independently actionable structural findings, not maximizing the number of messages. Each transition is checked in dependency order:

1. Validate the source name.
2. Validate the target name if the source passed.
3. Reject a self-transition only if both names are valid.
4. Insert or collapse an exact duplicate only if the edge is valid.
5. At the end, apply completed-definition checks such as the nonempty edge requirement.

| Candidate input | Current fail-fast result | Reasonable batch interpretation |
|---|---|---|
| Valid edge | Accepted | Retain as a candidate edge. |
| Invalid source, valid target | One source error | Report the source error; do not insert. |
| Valid source, invalid target | One target error | Report target error; do not insert. |
| Both endpoints invalid | Source error only today | Could report both independent endpoint errors, but this expands the per-transition diagnostic contract and requires stable endpoint roles. Do not report a self-transition as well. |
| Valid self-edge | One self-transition error | Report self-edge only. |
| Exact duplicate valid edge | No error today | Collapse or ignore duplicate. If the file format forbids repeated records, that is a separate host policy diagnostic. |
| Duplicate of an invalid edge | No duplicate concept applies | Invalid transitions should not enter duplicate tracking; otherwise a second malformed occurrence can produce a misleading duplicate message. |
| Nonempty input, every edge invalid | One edge error per edge under current checks | Return those edge diagnostics and no machine. Avoid also reporting `NoTransitions` unless the API defines it as a separate summary; it is a consequence of excluding invalid proposals. |
| Empty input | `NoTransitions` at build | Report `NoTransitions`; no edge-level failures exist. |

A batch pass can be called “complete” only relative to a stated rule set. It can report every endpoint/name/self-edge error it checks in the provided collection. It cannot report syntax errors, unresolved names in the source language, source schema violations, guard/effect errors, or host policies it was not given. It also cannot guarantee “all possible errors” if an earlier invalidity blocks later checks on that same edge. The API should say which independent checks are continued and which downstream checks are suppressed.

**Ordering:** Preserve input order, then a fixed within-record order (source, target, edge-level). This is easier to explain than sorting based on `HashSet` iteration. Existing machine enumeration order is unspecified and should not leak into diagnostic order. For unordered inputs such as a database query without an explicit sort, the host must choose a stable ordering if stable diagnostics matter.

## 4. Diagnostic representation and provenance

Statekit knows the error category and, for most errors, endpoint/state strings. It does not know which file, YAML key, JSON pointer, database row, editor control, compiler token, or VPE source record supplied the strings.

| Representation | Appropriate owner | Advantages | Limitations |
|---|---|---|---|
| Plain `StateError` | Statekit / Rust caller | Small, typed, matchable, standard error integration | No record index, span, related edge, severity, or parser context; `EmptyState` has no value. |
| Consumer wrapper `{ location, input_index, source: StateError }` | Loader/editor/compiler | Preserves exact source context, keeps StateError matchable through `source()`, no Statekit dependency on parser types | Host defines wrapper and maps the returned error to the originating record. |
| Structured Statekit diagnostic | Statekit only if shared consumer demand appears | Could include stable code and endpoint role | Adds a public compatibility contract; duplicate warnings, severity and location are not generic structural facts. |
| Generic context wrapper | Consumer or an error-context crate | Can annotate a `StateError` with record and operation | String context is less convenient for structured UI unless the host retains typed fields too. |
| Rendering framework (`miette`, `ariadne`, `codespan-reporting`) | Application/compiler/CLI | Can associate diagnostics with source code, spans, files, labels, and render snippets | These are presentation/source-map concerns. They do not supply Statekit's structural rule validation and should not be a default dependency. |

Primary Rust documentation describes `std::error::Error::source` for causal error chains. `miette` exposes source code and labels/spans on diagnostics; `ariadne` represents labeled source sections; `codespan-reporting` renders diagnostics against file IDs and spans. These are useful patterns for the host boundary, not evidence that Statekit needs to depend on them.

**Smallest useful context:** input ordinal or consumer record key, plus the underlying `StateError`. The consumer should retain any exact source span or JSON/YAML path. Statekit should not retain file names, byte offsets, line/column positions, severity, localized text, or source documents.

## 5. Rust API alternatives (hypothetical only)

The following names are not current APIs.

### Existing fail-fast API

```rust
let machine = Machine::builder()
    .try_allow("queued", "running")?
    .try_allow("running", "done")?
    .build()?;
```

This is concise and preserves early return for programmatic callers. `try_allow` currently consumes the builder.

### Consumer-owned aggregation using current APIs

```rust
use statekit::{Machine, StateError};

struct LocatedError {
    row: usize,
    source: StateError,
}

fn build_with_errors(
    rows: &[(String, String)],
) -> Result<Machine, Vec<LocatedError>> {
    let mut errors = Vec::new();
    let mut accepted = Vec::new();

    for (row, (from, to)) in rows.iter().enumerate() {
        match Machine::builder().try_allow(from.as_str(), to.as_str()) {
            Ok(_) => accepted.push((from.clone(), to.clone())),
            Err(source) => errors.push(LocatedError { row, source }),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let mut builder = Machine::builder();
    for (from, to) in &accepted {
        builder = builder.try_allow(from.as_str(), to.as_str())
            .expect("each edge was validated above");
    }

    builder.build().map_err(|source| {
        vec![LocatedError { row: rows.len(), source }]
    })
}
```

This illustrates existing calls, not a recommended final implementation. It validates each accepted edge once in a temporary builder and again in the final builder. It reports the empty-input `NoTransitions` from `build()` with a host-defined definition-level context. When some rows are invalid, it returns their errors without attempting to build a partial machine, avoiding a cascading `NoTransitions`. A production adapter should avoid `expect` if data transformations can occur between the validation and construction passes; both passes should use the same retained inputs.

The wrapper's `row` is host provenance. It can be expanded to hold a config path, row ID, or span. `StateError::EmptyState` itself does not identify which endpoint failed, but the wrapper can retain both original endpoints and the validator’s fixed source-first behavior can be documented.

### Hypothetical batch API

One possible shape, if evidence justifies it, is:

```rust,ignore
// Hypothetical: not present in Statekit today.
let machine = Machine::try_from_transitions(transitions)?;
// Error could be Vec<IndexedStateError>, kept in input order.
```

Alternative shape:

```rust,ignore
// Hypothetical: validate raw pairs, then construct only on an error-free report.
let report = Machine::validate_transitions(transitions);
let machine = report.into_machine()?;
```

`Result<Machine, Vec<E>>` is conventional but `Vec<E>` can technically be empty, and Statekit would need to define how `NoTransitions` relates to edge errors. A nonempty error collection type tightens semantics but adds a public type. Returning a validated intermediate report/token makes separate validation and final construction explicit, but creates another type whose invariants must be maintained. Borrowed input avoids copying but ties errors or reports to input lifetimes if they retain references; owned endpoint strings are simpler but allocate. An iterator can keep a single scan, but to construct the machine after discovering later errors it must retain valid transitions until completion.

**Compatibility:** Additive APIs can coexist with the current builder. Do not change `try_allow` to accumulate silently: existing callers rely on `Result<Self, StateError>` and INV-007. Any diagnostic ordering promised by a batch API becomes public behavior; input order is the lowest-surprise contract.

## 6. Consumer case studies and worked examples

### Example 1 — Multiple independently invalid transition rows

**Input:**

| Row | From | To |
|---:|---|---|
| 0 | `new` | `review` |
| 1 | ` draft` | `ready` |
| 2 | `closed` | `closed` |
| 3 | `new` | `review` |
| 4 | empty string | whitespace-only |
| 5 | `review` | `approved` |

**Current API result if chained:** row 1's `AmbiguousStateName` is returned; the chain cannot continue. Row 2 and later are not examined.

**Consumer-owned aggregate:** Apply the temporary-builder pattern above. In input order, report row 1 as an ambiguous source name, row 2 as a self-transition, and row 4 as `EmptyState` (the source is examined first, so the target-specific issue is suppressed). Rows 0, 3, and 5 validate individually. If no errors existed, the final build would collapse duplicate row 3 according to INV-004. Because errors exist, return no `Machine`; do not add a redundant “machine has zero transitions” message.

**Limitation:** The consumer gets one Statekit error for each invalid row, not necessarily every field problem within a row. A batch API could deliberately inspect both endpoints and report two endpoint errors, but that is a changed accumulation contract and can create duplicate/cascade noise. That value has not been demonstrated.

### Example 2 — Configuration loader with file or record locations

**System:** A loader parses a JSON/YAML/TOML file into a richer list of transition records. Parsing and schema validation happen before Statekit. The host retains each record's JSON pointer, source span, or database primary key.

**Current Statekit role:** Validate each `(from, to)` pair. It has no format parser and no source-span parameter. Keep a side table from input ordinal to source metadata; wrap each returned `StateError` with that context. For exact source rendering, the host can pass the resulting wrapper to its own diagnostic renderer.

```rust,ignore
// Host-defined context; Statekit has no SourceLocation type.
struct ConfigDiagnostic {
    pointer: String,
    endpoints: (String, String),
    source: StateError,
}
```

If the format parser itself fails, that is a parser diagnostic and no Statekit edge validation occurs for records that were not produced. If name resolution fails in a host language, report that before or alongside structural validation according to the host's phase policy. Do not force a source location into Statekit: it has no stable, format-neutral meaning across file, database, and generated inputs.

**Performance:** A one-pass host loop can retain metadata and typed errors. With current APIs, a full successful aggregate path revalidates accepted edges at final construction. A host with very large definitions should measure the invalid-input path and success overhead before asking for an optimized batch API.

### Example 3 — Administrative editor with incremental and save-time validation

**System:** A UI edits one transition at a time, but also wants a complete error list before saving.

**Immediate edit:** Validate just the edited pair using `Machine::builder().try_allow(from, to)`. On success, update the host's draft model; on failure, leave the previous draft unchanged and show the field error. The temporary builder can be dropped. This is compatible with transactional UI edits.

**Before save:** Revalidate the full draft and attach editor row IDs. If any errors exist, show the ordered list and keep the previously active immutable `Machine` in use. If validation succeeds, build the replacement `Machine` and swap it at the host's chosen boundary.

The two experiences do not inherently require two Statekit APIs: one-edge fail-fast feedback and the consumer aggregation wrapper use the existing API. A separate batch API would reduce adapter boilerplate only if that code is repeated across hosts. Statekit should not own editor draft state, severities, highlighting, save policy, or atomic configuration replacement.

### Example 4 — Compiler pipeline and VPE

**Compiler phases:** A consumer can parse source, resolve names and symbols, run structural validation, run domain-semantic checks, then construct a Statekit machine as a structural projection. Statekit belongs at the structural boundary. It cannot repair parse errors, unresolved references, type errors, guard/effect failures, or source mapping.

**Verified VPE behavior:** Current VPE `validate_law` returns `Result<Vec<String>, CompileError>`. It checks schema/law compatibility, nonempty/unique state names, initial state, transition targets, effects, guards, transient-state rules, and `AUTO_TICK` structure. It returns on the first `CompileError` and separately accumulates warning strings, including terminal/unreachable-state and ambiguous-action/priority observations. `VpeCompiler::validate` returns a `ValidationReport` with `ProcessRef` and warnings. `LawSource`, `StateSource`, and `TransitionSource` are Serde data structs without source-span fields. `CompileError` is typed in categories but some errors contain a formatted string.

VPE's source set is richer than Statekit's topology. VPE must retain transition action, priority, guard/effect data, plus other process metadata. A structural projection can identify invalid endpoint names or self-edges only if VPE wants Statekit's stricter rules. However, current compatibility caveats remain: VPE supports indexed compiled states, can have zero transitions, allows same-state targets at its own source validation layer, and has multiple semantic transitions that may share endpoints. Statekit rejects self-edges, requires at least one edge, and collapses same endpoint pairs. Therefore, a projection can reject or erase VPE distinctions. VPE's own validator remains authoritative unless a compatibility subset is explicitly established.

**Potential test-only use:** Build a Statekit machine from a VPE fixture's unique non-self endpoint pairs and compare the projected topology or replay structural sequences in tests. A test wrapper maps Statekit errors back to the originating `TransitionSource`. This only covers topology, not whether a guard passes, which priority wins, an effect is correct, or a process verdict is lawful. There is no evidence that a Statekit production dependency improves VPE diagnostics today.

## 7. Performance and memory implications

The frozen v0.4 baseline records valid machine construction, query/runtime operations, and memory on linear graphs from 100 to 100,000 transitions. It describes the current internal HashSet and scan-based operations. It does not include malformed definitions, a batch validator, diagnostic allocations, or source-provenance storage. No new timings were run, so none are claimed here.

| Cost | Fail-fast builder | Consumer aggregation | Hypothetical batch API |
|---|---|---|---|
| Valid construction | One construction pass | Per-edge temporary validation plus final revalidation unless optimized by host-specific logic | Could validate and retain valid edges in one pass, then construct once |
| Invalid input | Stops at first error; low diagnostic allocation | Allocates wrapper/error per invalid row and continues | Allocates per returned diagnostic; may scan all rows |
| Ordering | One error, no ordering question | Naturally preserves host input order | Must explicitly guarantee or disclaim ordering; input order is simplest |
| Provenance | Host can retain one context at call site | Host naturally attaches per-row context | API may return row ordinal, but source spans remain host-owned |
| Retained machine memory | No diagnostic state retained | No diagnostic state retained after validation | Should not retain diagnostics in `Machine`; return them separately |
| Runtime transition cost | Unchanged | Unchanged | Unchanged if the same `Machine` representation is produced |

**Roadmap interaction:** Statekit's v0.5 identity/lowering and v0.6 indexing work may change validation cost and temporary representation choices. No diagnostic proposal should preempt those decisions. Accumulated error memory scales with the number and payload size of errors, while a diagnostics object stored in every valid `Machine` would increase steady-state footprint without value; do not do that.

**Focused benchmark plan if a batch API is prototyped:** Compare current fail-fast valid construction, consumer wrapper valid construction, consumer wrapper with 1%, 10%, and 100% invalid rows, and a prototype batch API on the same ordered datasets. Include duplicate-heavy input and both invalid endpoint and self-edge cases. Measure total construction time, allocations/peak bytes, and diagnostic count for 1k, 10k, and 100k edges; retain the existing v0.4 baseline unchanged as a control. Include the same graph transition lookup benchmarks to verify there is no runtime regression. Use realistic state-name lengths and separately test source-location side tables in the host, since those costs do not belong to Statekit.

## 8. Correctness and testing recommendations

Any proposal must retain current invariants and clearly specify which additional behavior is promised.

| Test category | What to verify |
|---|---|
| Existing fail-fast behavior | `try_allow` returns the first endpoint/self-edge error; a failed call returns no builder; `build()` rejects empty definitions. |
| Valid-machine guarantee | No code path returns a `Machine` when any required structural diagnostic exists. For aggregate API, assert the result is `Err` and no partial machine is accessible. |
| Error classification | Empty/whitespace-only, leading/trailing Unicode whitespace, self-transition, empty definition, and runtime invalid move map to existing `StateError` variants. |
| Error dependencies | Invalid source suppresses target/self checks under current API; decide explicitly if any new batch API collects both endpoint errors. Invalid edges do not enter duplicate tracking. |
| Duplicates | Exact duplicate valid pairs remain idempotent and do not become errors unless a separate opt-in host policy is used. |
| Ordering | If batch promises input order, permuting records permutes diagnostics accordingly; repeated runs with same ordered input have identical diagnostic order. Do not rely on HashSet iteration. |
| Incremental recovery | A host adapter can continue with later records after an isolated edge error, then build only if the full pass succeeds. The Statekit builder itself does not recover. |
| Empty and all-invalid cases | Empty input returns `NoTransitions`; all-invalid nonempty input returns actionable row errors and does not add an accidental cascade unless documented. |
| Batch/incremental consistency | Every accepted candidate produces the same final logical `Machine` as the current builder; compare machine equality after normalizing order. |
| Property-based tests | Generate valid/invalid endpoint pairs and compare batch classifications with `Transition::try_new` behavior or a reference implementation. Keep the reference rule model independent enough to catch shared bugs. |
| Permutation and duplicate properties | Reordering inputs preserves the set of valid logical edges and classification per record; exact duplicate inputs collapse in final topology. |
| Wrapper provenance | Every returned StateError points back to the correct host record and preserves original endpoint text, especially for `EmptyState`. |

Promises about error ordering, duplicate reporting, multi-error-per-row behavior, or batch-vs-builder equivalence belong in the specification/API docs because they become public compatibility obligations. A user-facing batch capability that changes observable errors needs focused tests even if machine construction remains unchanged.

## 9. Capability classification

| Category | Finding | Assessment |
|---|---|---|
| **A. Already supported** | Typed, fail-fast construction errors; early endpoint/self-transition checks; nonempty-machine check at build; immutable valid machine; standard Rust error integration; invalid transition/init errors retain relevant names. | Good fit for programmatic callers and immediate feedback. Errors are not rich diagnostics and `EmptyState` lacks raw value/context. |
| **B. Consumer-owned** | Input parsing, source locations, input record IDs, error accumulation through temporary builders, UI severity/rendering, application-specific duplicate policy, ordering of unordered records, parse/name-resolution/semantic errors. | Smallest practical current solution. Must not expose a machine after any required errors. Successful aggregate path revalidates edges. |
| **C. Small reusable improvement** | Documentation example explaining per-record validation, contextual wrappers, the consuming builder, and delayed final build; consider clarifying INV-007's detection-vs-report wording. | High confidence, no runtime or compatibility cost beyond wording. No new API needed. |
| **D. Future research** | Optional batch `try_from_transitions` or `validate_transitions` if repeated consumer evidence appears; richer endpoint-specific errors; benchmarked validation scratch/index strategy. | Requires an explicit aggregation contract, input-order semantics, error dependency policy, and evidence. Keep additive and separate from fail-fast builder. |
| **E. Out of scope** | Parsers, source maps/spans, JSON/YAML pointers, localization, severity policy, diagnostic rendering, compiler phase orchestration, semantic guard/effect errors, generic diagnostic framework dependency. | Belongs to the consumer or its diagnostic/rendering stack. |

## 10. Ranked recommendations

1. **Document consumer-owned aggregation with current APIs.** Show one-edge temporary-builder validation, retained row/location context, ordered errors, and final build only on success. Explain that `try_allow` consumes the builder and that duplicate edges are idempotent. This directly answers the practical configuration/editor need without changing INV-007 or adding retained state.
2. **Clarify the scope of INV-007 before considering aggregation APIs.** State whether “earliest” governs when invalidity is detected, when an error is returned, or both. Preserve existing fail-fast behavior either way. A batch API can only claim its own aggregate-return behavior explicitly.
3. **Collect evidence from independent consumers before adding a batch API.** VPE is not proof of reusable demand: it has its own semantic compiler and source structures do not retain spans. Seek a second independent configuration consumer or a repeated adapter implementation, and measure realistic definition sizes.
4. **If evidence supports an API, prototype an additive batch constructor before a diagnostic-aware builder.** Prefer one owned result containing either a valid immutable `Machine` or ordered contextualized structural errors. Do not expose partially valid machine state. Decide whether the batch accepts borrowed pairs or owned transitions, how it handles both invalid endpoints, how it avoids a redundant `NoTransitions`, and whether ordinal context is worth the public type.
5. **Do not add source-location types or rendering dependencies to Statekit.** Consumer location wrappers and established diagnostic renderers already solve that boundary while keeping Statekit small and format-neutral.

**Tradeoff:** A batch API could eliminate consumer boilerplate and double validation, but it raises semantic and compatibility questions that documentation alone does not. Until repeated demand or a benchmark identifies that cost, the no-API recommendation is stronger.

## 11. Architectural questions answered

1. **Can consumers already collect sufficient errors with repeated `try_allow()` calls?** Not on one builder: it is consumed and unavailable after an error. They can collect one edge-level error at a time using temporary builders, preserve valid pairs, and build only if the pass has no errors. This is sufficient for many basic configuration cases but incurs revalidation and cannot report multiple endpoint errors from one pair.
2. **What errors cannot be accumulated correctly with current APIs?** A caller cannot continue a single builder after failure, inspect or repair its partial state, or ask Statekit for multiple independent errors from one transition. There is also no Statekit source provenance or batch ordering contract. Duplicate input is not an error under current semantics.
3. **Would batch validation weaken or preserve INV-007?** It can preserve the valid-machine guarantee and early internal detection. It returns the aggregate later than the first detectable error, so whether it conflicts with INV-007 depends on wording. Keep current fail-fast behavior unchanged and define batch semantics separately.
4. **Is fail-fast validation a library invariant, an API behavior, or both?** Both: INV-007 records earliest error behavior, and `try_allow(self) -> Result<Self, StateError>` concretely exposes fail-fast ownership semantics.
5. **Should accumulation be separate and optional?** If added, yes. A separate batch operation is clearer than changing the primary builder into an implicit error collector.
6. **Should Statekit retain source locations?** No current evidence supports it. Locations are format- and host-specific; consumers should wrap `StateError` with their own source context.
7. **Does a diagnostic-aware builder introduce an undesirable partially valid state?** It can, unless the type is clearly a draft/validation object and cannot yield a machine while errors remain. That adds state and API complexity without demonstrated need.
8. **Can a consumer wrap `StateError` without losing essential information?** Yes. It can retain the typed error as a source and add its own row/path/span. For `EmptyState`, the host should also retain both raw endpoint strings because StateError is a unit variant.
9. **Is repeated demand established across independent consumers?** No. The reports identify configuration loaders as plausible, and VPE as a concrete but semantically richer compiler. No independent adoption evidence or repeated implementation was established.
10. **Could documentation deliver most value?** Likely for the current evidence: it explains how to aggregate without changing the builder or storing diagnostics in `Machine`. The missing ergonomic batch operation remains a hypothesis until consumer use validates it.

## 12. Risks, unresolved questions, and next research

### Risks

- A batch report can overstate completeness if it suppresses dependent checks or only reports one error per record.
- Reporting both endpoints and a self-edge for one malformed pair can create misleading cascades.
- Treating duplicate pairs as errors would alter INV-004 semantics.
- A `Vec<StateError>` has no source index; a public ordinal wrapper is useful but increases API commitment.
- Stable error ordering becomes a compatibility guarantee if documented.
- Retaining partial valid transitions in a builder can make callers confuse “candidate edges” with a valid definition.
- Revalidating during aggregate construction may be expensive for large inputs; no invalid-input benchmark exists.
- VPE projection can reject currently accepted forms or collapse multiple semantic transitions. It is not evidence for a general diagnostic API.

### Open questions for future work

- Does any real consumer need to report both invalid source and invalid target on one record?
- Is error accumulation needed for a large number of entries, or is the main need preserving source context?
- Should a future API return `Result<Machine, Vec<IndexedStateError>>`, a validation report, or a separate candidate type?
- Should a batch validator treat a nonempty all-invalid input as only edge errors, or also summarize that no machine can be produced?
- What does INV-007 mean for an explicitly aggregating API: earliest detection, earliest delivery, or default API behavior only?
- Would consumer wrappers be enough across Rust CLI, web editor, database, and compiler use cases?
- Which VPE source-loading layer can retain line/column or JSON pointer data if compiler diagnostics later need locations?
- Would changing internal validation during v0.5/v0.6 affect useful batch processing or allocation patterns?

### Suggested next tracks

1. Validate the consumer-owned example against a real configuration loader and measure its successful and invalid paths.
2. Inspect diagnostic provenance in a broader compiler/configuration ecosystem only if source mapping remains the sticking point.
3. Prototype a batch API in an isolated branch or scratch experiment only after a concrete consumer can state its needed error-completeness contract; compare against the frozen v0.4 baseline without overwriting it.
4. Continue the planned v0.5 identity/lowering and v0.6 indexing work on their own terms; do not couple them to diagnostics without measured evidence.

## 13. Primary sources

### Statekit

- [Statekit README](https://github.com/whitephoenixsoft/statekit/blob/main/README.md)
- [Builder implementation](https://github.com/whitephoenixsoft/statekit/blob/main/src/builder.rs)
- [StateError variants](https://github.com/whitephoenixsoft/statekit/blob/main/src/error.rs)
- [State-name validation](https://github.com/whitephoenixsoft/statekit/blob/main/src/state_name.rs)
- [Transition validation](https://github.com/whitephoenixsoft/statekit/blob/main/src/transition.rs)
- [Machine API](https://github.com/whitephoenixsoft/statekit/blob/main/src/machine.rs)
- [Builder and transition integration tests](https://github.com/whitephoenixsoft/statekit/blob/main/tests/transition_tests.rs)
- [Frozen v0.4 benchmark baseline](https://github.com/whitephoenixsoft/statekit/blob/main/docs/benchmarks/baseline-v0.4.md)
- Supplied local documents reviewed: [`Project Core`](sandbox:/workspace/scratch/3524c4ebe062/project_sources/02-project-core-2-.md), [specification](sandbox:/workspace/scratch/3524c4ebe062/project_sources/01-statekit-specification.md), [README snapshot](sandbox:/workspace/scratch/3524c4ebe062/project_sources/03-README-1-.md), and [roadmap](sandbox:/workspace/scratch/3524c4ebe062/project_sources/04-roadmap-2-.md).

### VPE

- [VPE source model](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/source.rs)
- [VPE validation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/validate.rs)
- [VPE compilation/lowering](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compile.rs)
- [VPE error types](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/error.rs)
- [VPE compiled representation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compiled.rs)
- [VPE crate dependencies](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/Cargo.toml)

### Rust diagnostics ecosystem

- [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
- [`miette` Diagnostic documentation](https://docs.rs/miette/latest/miette/trait.Diagnostic.html)
- [`ariadne` labeled source sections](https://docs.rs/ariadne/latest/ariadne/struct.Label.html)
- [`codespan-reporting`](https://docs.rs/codespan-reporting/latest/codespan_reporting/)

## Conclusion

Statekit does not currently need a diagnostic framework to preserve its core value. Its fail-fast builder is explicit and keeps invalid machines from escaping. Consumers can collect per-edge errors today by validating isolated candidates and retaining their own source context, then building only when the collection is clean. That approach has a real ergonomic and repeated-validation cost, but the available evidence does not show that the cost is shared widely enough to justify a new API.

The lowest-risk next step is a documentation example and a clarification of INV-007's meaning if aggregation is ever explored. If consumer evidence later justifies an addition, evaluate an optional batch constructor with ordered errors and no partial `Machine`, while keeping parsing, locations, rendering, semantic validation, and source maps in the host application.
