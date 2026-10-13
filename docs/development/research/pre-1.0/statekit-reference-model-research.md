# Statekit — Reference Model & Model-Based Testing Research

**Status:** Research proposal; no architectural decisions are made here  
**Investigation:** Second report in Statekit Consumer Research & Capability Discovery  
**Repositories reviewed:** `whitephoenixsoft/statekit` and `whitephoenixsoft/verified-process-engine`, default branch `main`  
**Research date:** 2026-10-13 (UTC)

## Executive summary

**Finding:** Statekit can already serve as a practical *structural* reference model for applications whose runtime-defined state behavior is accurately described by a flat set of directed edges. It can construct an immutable oracle, validate a directly attempted move, replay a sequence with a `MachineInstance`, compare its current position with a system under test (SUT), check forbidden direct transitions, and compare machine definitions with structural equality. No new Statekit API is needed for these uses.

This is already a pattern in Statekit's own tests. `tests/property_tests.rs` builds each generated machine and an independent `HashSet<(String, String)>` edge model from generated edge data, then compares enumeration, membership, terminality, successful and failed transitions, state preservation, and instance independence. Proptest generates configurations and operation sequences. This is useful evidence of a working reference model; it does not make Statekit a model-based testing framework. The file [`tests/property_tests.rs`](https://github.com/whitephoenixsoft/statekit/blob/main/tests/property_tests.rs) is the most relevant present-day example.

Testing libraries provide the machinery Statekit should leave to consumers: operation generation, state-dependent preconditions, execution against a SUT, invariants/postconditions, sequence shrinking, seeds/replay, and coverage policy. Proptest's `proptest-state-machine` explicitly separates a reference state machine from the SUT and shrinks failing command sequences. Hypothesis supplies rules, state-dependent preconditions, bundles, and per-step invariants. XState combines a behavioral machine with generated event sequences and can compare an implementation against it; its current v6 documentation is marked alpha. These approaches validate a **behavioral model**, which is richer than Statekit's topology alone.

VPE is a concrete but not-yet-integrated consumer. Current `main` has a compiled representation (`Node`/`Edge` with state indices), a deterministic evaluator, compiler checks for target references, initial-root reachability warnings, and `AUTO_TICK` cycle checks. VPE's Cargo manifests do not depend on Statekit. The `simulation` and `migration` source modules are currently stubs. The most credible first experiment is **Architecture B**: use Statekit only as a development-time structural oracle in VPE tests, while VPE retains its own compiler/runtime. This can check that each emitted state edge exists in the compiled topology and that topology projections behave as expected. It cannot verify guards, priorities, effects, context, history, or semantic outcomes.

**Answer to the success question:** Yes, Statekit can be useful as a structural reference model now. For basic transition conformance, the smallest additional capability set is **zero**. Rooted reachability could reduce repeated structural code in consumers, including a test cross-check for VPE's BFS, but it is optional analysis rather than a testing feature. Incoming edges are similarly bounded but can be built by scanning `transitions()`. Sequence validation, reset, equality, and basic structural coverage can already be handled by callers.

## Continuity with the first investigation

The first report, [`statekit-consumer-research.md`](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md), established that Statekit owns runtime-defined topology, construction-time structural validation, immutable definitions, traversal, and basic introspection. Its proposed predecessor query and caller-rooted reachability remain proposals; neither is present in the current API. Rich diagnostics and portable representations remain unresolved.

This investigation preserves the same boundary: Statekit answers structural questions about states and directed edges. A test harness generates and executes cases. VPE or another consumer owns event interpretation, guards, effects, priorities, process meaning, simulation, migration, and coverage requirements.

## 1. Current Statekit testing capabilities verified against source

The supplied README, Project Core, specification, and roadmap were reviewed. Current Statekit `main` source and test files were then inspected.

### Runtime and topology APIs available today

- `Machine::builder().try_allow(from, to).build()` constructs a runtime-defined immutable topology. Edge names are validated as added; self-transitions are rejected; duplicate endpoint pairs collapse; an empty transition set fails at `build()`.
- `Machine` exposes `states()`, `sources()`, `transitions()`, `targets_from(state)`, `contains_state(state)`, `transition_count()`, `can_transition(from, to)`, `validate_transition(from, to)`, and `is_terminal(state)`.
- `Machine::instance(initial)` checks that the starting state exists. A `MachineInstance` exposes `state()`, `can_transition_to(target)`, `transition_to(target)`, `is_terminal()`, and `machine()`.
- A failed `transition_to` returns an error without changing the instance. Instances can be constructed again from the immutable `Machine`; there is no explicit clone/reset method, and none is required for test replay.
- `Machine` equality compares machine definitions; `MachineInstance` equality compares machine definitions plus current state.
- Enumeration order is documented as unspecified. Tests that need stable assertions should compare sets or sort their collected values.
- `is_terminal(unknown)` is true under the current “no outgoing edges” definition. If a test needs to distinguish an unknown state from a known leaf, it should check `contains_state` first.

### Existing property/reference-model tests

`tests/property_tests.rs` defines `Model = HashSet<(String, String)>`. For each generated topology, it builds the Statekit machine and a separate edge-set model. The properties compare machine enumeration and edge queries to the model; instance-level properties compare allowed moves, expected state, terminality, failed-transition atomicity, and independent instances. One property generates a list of transition attempts and checks the reference state after each one.

This is a **reference model**, but it is a flat structural model, not an abstract business process. The test suite does not use a stateful testing runner that independently chooses state-dependent commands and shrinks whole valid command programs. Proptest itself and the `proptest-state-machine` crate offer that next testing layer.

The existing test arrangement also has an important limit: both the machine and the `HashSet` oracle receive the same generated edge data. It tests Statekit's interpretation of that input against a simple mathematical set. It does not test an external config parser, prove the fixture represents the intended business rule, or eliminate a shared bug in a consumer that supplies the edge list.

### Relevant Statekit files

- [Machine queries](https://github.com/whitephoenixsoft/statekit/blob/main/src/machine.rs)
- [Machine instance](https://github.com/whitephoenixsoft/statekit/blob/main/src/instance.rs)
- [Builder](https://github.com/whitephoenixsoft/statekit/blob/main/src/builder.rs)
- [Typed errors](https://github.com/whitephoenixsoft/statekit/blob/main/src/error.rs)
- [Property tests](https://github.com/whitephoenixsoft/statekit/blob/main/tests/property_tests.rs)
- [Property-test outline](https://github.com/whitephoenixsoft/statekit/blob/main/docs/property-test-outline.md)
- [Transition integration tests](https://github.com/whitephoenixsoft/statekit/blob/main/tests/transition_tests.rs)

## 2. Model-based testing approaches

| Tool | Reference-model representation | Operation generation and execution | Comparison, shrinking, coverage | Relevance to Statekit |
|---|---|---|---|---|
| **Proptest** | `proptest-state-machine` defines a reference state and transition type. The model supplies an initial-state strategy, a transition strategy (optionally dependent on model state), `apply`, and preconditions. A separate `StateMachineTest` binds the model to the SUT. | Generates sequential operation lists, updates the abstract model, then applies each operation to the SUT. | Checks postconditions and invariants after each step; shrinks failing sequences and transition values; preserves a reproducible regression seed. The current documented strategy is sequential. | Best Rust comparison for testing VPE or another stateful runtime. Statekit can supply structural state facts, but must not own the runner, SUT adapter, shrinking, or test policy. |
| **QuickCheck (Rust)** | Core crate tests properties over generated `Arbitrary` values. A caller can define a `Vec<Command>` or implement a custom model, but the core crate does not present the same dedicated command/precondition/reference/SUT abstraction as `proptest-state-machine`. | Caller generates operation sequences and manually replays them, or uses an additional state-machine testing library. | Random generation and shrinking are core capabilities. State-dependent validity and shrinking a coherent command sequence are caller responsibilities unless an extension is used. | Statekit's topology can help the test decide if a generated target is allowed. Keep operation generation/shrinking in QuickCheck or a test helper. |
| **Hypothesis stateful testing (Python)** | `RuleBasedStateMachine` holds model/SUT state; rules are operations; generated bundles can pass outputs from one operation into another. | Hypothesis generates chained rules and data. `precondition` avoids selecting rules that are currently inapplicable. | Invariants can be checked after every step; failing action sequences are minimized by Hypothesis. Stateful tests generate whole tests, not just independent input values. | Demonstrates that a reference model needs state-dependent action generation, but that machinery belongs to the testing framework. A Python consumer could use a Statekit-like structural service behind an adapter, though cross-language binding is not established. |
| **XState / `@xstate/test` and `xstate/graph` (JavaScript/TypeScript)** | A declarative behavioral state machine includes states, events, transitions, guards/context/actions as applicable. | Generates event sequences from the model; paths can also be generated and then run against an implementation through `testPaths` / test utilities. | Compares model observations with SUT observations; offers transition/state/configuration coverage reports and failure replay/shrinking through fast-check integrations. | Illustrates behavioral model-based testing and coverage, beyond Statekit's generic edge model. Statekit should not absorb event generation, model-to-SUT mapping, or coverage. The current v6 docs identify v6 as alpha; API maturity/version should be rechecked before adopting an exact pattern. |

### What transfers from model-based testing, and what does not

The reusable pattern is:

1. Keep an abstract expected state separate from the implementation state.
2. Generate or choose a sequence of commands.
3. Apply each command to the model and SUT.
4. Compare observable state and operation outcomes after each step.
5. Check invariants after each step and shrink a failing sequence in the test framework.

Statekit already provides a useful structural component for steps 1 and 3 when the expected state is simply a state name and the operation is “move along one configured edge.” The framework owns the sequence generator, shrinking, run count, seed/replay, test reporting, and coverage policy. If operations contain events, data payloads, guards, or effects, the consumer's model must represent those semantics; Statekit's state/edge topology is only one constraint layer.

## 3. Concrete testing scenarios

The examples below are illustrative. The Statekit calls use the current API. `StatusSut` is a consumer test adapter, not a Statekit trait.

### Example 1 — Configuration-driven application

**System under test:** An application loads status edges from tenant configuration and exposes a status transition operation.

**Reference model:** A `Machine` built from the parsed edge list, plus one `MachineInstance` initialized at the entity's current status.

**Operations:** A test framework can generate attempted target states. To include both valid and invalid attempts, sample from the known state set and also generate unknown target strings. Statekit does not generate those attempts.

**Comparison:** The SUT should accept exactly the direct edges Statekit accepts, land in the same state after an accepted operation, and preserve its state after a rejected one.

```rust
use statekit::{Machine, StateError};

trait StatusSut {
    fn state(&self) -> &str;
    fn transition_to(&mut self, target: &str) -> Result<(), ()>;
}

fn check_attempts<S: StatusSut>(
    machine: &Machine,
    sut: &mut S,
    initial: &str,
    attempts: &[&str],
) -> Result<(), StateError> {
    let mut model = machine.instance(initial)?;
    assert_eq!(sut.state(), model.state());

    for target in attempts {
        let prior = model.state().to_owned();
        let expected = model.can_transition_to(target);
        let observed = sut.transition_to(target).is_ok();

        assert_eq!(observed, expected);
        let model_result = model.transition_to(target);
        assert_eq!(model_result.is_ok(), expected);
        assert_eq!(sut.state(), model.state());

        if !expected {
            assert_eq!(sut.state(), prior);
        }
    }

    Ok(())
}

fn example_machine() -> Result<Machine, StateError> {
    Machine::builder()
        .try_allow("open", "triaged")?
        .try_allow("triaged", "active")?
        .try_allow("active", "resolved")?
        .build()
}
```

**Limitations and false confidence:** If the SUT and Statekit both read the same malformed parsed representation, a parser bug can affect both. Keep some hand-written expected topology fixtures, test parser mapping separately, and include forbidden moves. If the application operation is an event like `APPROVE`, Statekit cannot decide which edge that event means; the host adapter must map event to candidate edge, and guards may reject a structurally allowed edge. This example tests structural conformance, not authorization or business correctness.

### Example 2 — Independent runtime implementation

**System under test:** A custom runtime stores adjacency in its own structure and implements `can_transition` and `transition_to` independently of Statekit.

**Reference model:** Statekit built from a small, independently authored test fixture. For a separate machine-definition comparison, collect `machine.transitions()` into a `HashSet` and compare it with the SUT's normalized edge set. `Machine` equality is useful when comparing two Statekit definitions, but it cannot compare the SUT's private representation directly.

**Operations:** Test all ordered pairs of known states for direct-edge membership, then generate stateful sequences from the current state's targets plus invalid targets. Test invalid-operation state preservation separately.

```rust
use statekit::{Machine, StateError};

fn compare_pair_queries(
    machine: &Machine,
    runtime: &impl CustomRuntime,
) {
    let states: Vec<_> = machine.states().collect();
    for from in &states {
        for to in &states {
            assert_eq!(
                runtime.can_transition(from, to),
                machine.can_transition(from, to),
                "edge mismatch: {from} -> {to}",
            );
        }
    }
}

trait CustomRuntime {
    fn can_transition(&self, from: &str, to: &str) -> bool;
}
```

To test a sequence, create a fresh Statekit instance with `machine.instance(initial)` for each generated case. After each SUT operation, compare its visible state with the model's `state()`. A new `MachineInstance` provides reset/replay; no `reset()` API is necessary.

**Limitations and false confidence:** A pairwise matrix catches direct-edge discrepancies, not bugs in configuration loading, concurrency, persistence, transaction boundaries, or semantic event mapping. If both implementations are generated from the same flawed canonical input, independent expected fixtures and parser tests are still needed. The test author must choose meaningful operation sequences and invariants; Statekit does not decide coverage sufficiency.

### Example 3 — VPE process execution

**System under test:** VPE compiles a process law into named `Node`s and indexed `Edge`s. Edges also carry action, priority, guards, and effects. The evaluator chooses among matching transitions, checks history/context, evaluates guards, may follow `AUTO_TICK` transitions, and returns a verdict plus emitted events.

**Reference model:** A structural projection of the compiled process into Statekit, if the topology is compatible with Statekit's rules. The projection includes only `(source state name, target state name)`.

```rust
// Illustrative projection from the current VPE compiled representation.
// It succeeds only if the projection has at least one edge and no self-edge.
fn structural_model(process: &CompiledProcess) -> Result<Machine, StateError> {
    let mut builder = Machine::builder();

    for node in process.nodes() {
        for edge in &node.transitions {
            let target_name = &process.nodes()[edge.target_idx].name;
            builder = builder.try_allow(&node.name, target_name)?;
        }
    }

    builder.build()
}
```

After evaluating a VPE request, a test can check that each emitted `state_before -> state_after` pair exists in the structural model and replay those emitted edges through a fresh `MachineInstance`. It can also compare a selected sequence of VPE verdict positions against the structural instance. This checks that VPE's observable transitions remain inside its compiled topology.

**What this does not check:** Statekit cannot tell whether the action was valid, a guard should pass, the priority winner was correct, context/history was sufficient, the effect intent was right, or the verdict is lawful. A structurally allowed transition may be semantically unavailable because a VPE guard fails; absence of a VPE transition is not automatically a Statekit failure.

**Current compatibility caveats:** Statekit requires at least one projected edge and rejects self-edges. VPE's current source has a valid-state list and records transitions by action/priority; it does not model its edges as unique endpoint pairs. Statekit deduplicates same-endpoint pairs, so the projection deliberately loses action identity and parallel semantic transitions. Its current string-based identity also does not directly reuse VPE's compiled `usize` state indices. These are reasons to keep the first experiment test-only and to treat any direct runtime integration as a later decision.

**Simulation and migration:** VPE's current `simulation` and `migration` modules are stubs, so there is no implemented behavior to assess against Statekit today. Once implemented, Statekit could check that simulated emitted steps lie on the candidate topology, or that a migration mapping preserves selected structural edges. It cannot validate baseline/candidate semantic equivalence, history transformation, guard behavior, migration compatibility, or which state mapping is correct. Those remain VPE properties.

## 4. VPE integration analysis

### Verified current VPE structure

- The repository is early development and contains separate `vpe`, `vpe-cli`, and `vpe-ffi` crates. `crates/vpe/Cargo.toml` does not currently list Statekit as a dependency.
- `CompiledProcess` contains a `Vec<Node>`, a `HashMap<String, usize>` state index, and an initial state index. Each `Node` has a name and `Vec<Edge>`; each edge carries an action, target index, priority, guard objects, and effects.
- The compiler validates state names and duplicates, verifies the initial state and target names, validates guards/effects, warns on terminal states and initial-root-unreachable states, and checks `AUTO_TICK` cycles.
- The unreachable-state check is a VPE-local BFS from `law.initial_state`; the `AUTO_TICK` cycle check is a VPE-local DFS over only `AUTO_TICK` edges.
- The runtime evaluates VPE transitions using state, requested action, priorities, guards, context, and chronicle; it emits a verdict/event sequence. This is intentionally richer than direct edge membership.
- Unit tests exist inline in compiler/runtime modules; examples test priorities, guard fallback, no matching action, auto-tick sequences, and runtime state errors.
- The source tree's `simulation` and `migration` modules are presently empty stubs.

Source links: [VPE README](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/README.md), [design philosophy](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/DESIGN_PHILOSOPHY.md), [compiled representation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compiled.rs), [compiler validation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/validate.rs), [compiler lowering](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compile.rs), [runtime evaluator](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/runtime/evaluate.rs).

### Architecture A — VPE uses Statekit directly

**Possible arrangement:** During compilation, VPE projects its resolved transition endpoints into a Statekit machine. During evaluation, VPE first selects a transition using its own action/priority/guard rules, then uses the structural machine to check or perform the chosen source-to-target move.

**Benefits**

- One reusable implementation could own generic endpoint validation and direct-edge checks.
- If Statekit later supports the identity type VPE already uses, it might reduce repeated topology lookups or make shared topology analysis cheaper to maintain.
- A successful transition could be structurally checked at the boundary where VPE emits a new state.

**Costs and limitations**

- Current VPE already validates targets, constructs an index, and evaluates against compiled edges; an extra Statekit check may duplicate work rather than simplify it.
- A string-based projection from VPE's indexed nodes introduces conversion/mapping work. Calling `MachineInstance::transition_to` also adds a second mutable position to keep synchronized with a stateless evaluator that receives `current_state` on each request.
- Statekit rejects an empty edge set and self-edges, while VPE's current schema/model has separate “state exists” and “transition list may be empty” concepts. Compatibility must be proven against real valid VPE laws.
- Statekit's edge equality collapses same source/target pairs even if VPE has multiple semantic edges for different actions/guards/priorities. That is acceptable for a topology projection only if VPE does not mistake it for transition identity.
- Direct dependency introduces release/version coupling and risks making VPE's independent value appear to rely on Statekit's runtime semantics.

**Assessment:** Possible, but not yet shown to remove enough code or runtime cost to justify a production dependency. Preserve VPE as the semantic authority and do not move action/guard selection into Statekit.

### Architecture B — Statekit is a test-only reference model

**Possible arrangement:** VPE keeps its compiler and evaluator. A dev-dependency test projects a compiled process's topology into Statekit and checks generated evaluator outcomes against the structural oracle.

**Benefits**

- No production dependency, runtime overhead, or semantic coupling.
- Provides an independent implementation of simple directed-edge behavior for conformance tests.
- Can test that every VPE-emitted edge exists, replay emitted sequences, compare allowed/forbidden direct moves, and cross-check topology equality/projection.
- Encourages VPE to retain its own identity, state index, and richer process semantics.

**Costs and limitations**

- VPE still owns topology validation and its BFS/DFS checks; tests can detect disagreement but do not remove implementation duplication.
- Projection rules can become test code that has its own bug. Keep a small hand-authored fixture and direct assertions independent of the projection.
- Statekit's current empty-machine/self-transition constraints may make some compiled VPE definitions unprojectable without a deliberate policy.
- This only validates structural consistency. Guard, action, history, effect, simulation, and migration properties still need VPE-specific reference models.

**Assessment:** Best first experiment if a concrete VPE test benefits from an independent structural oracle. The integration can remain optional; no conclusion that VPE must depend on Statekit follows from this research.

### Where Statekit could contribute to VPE tests

| VPE test target | Statekit contribution | VPE-only part |
|---|---|---|
| Compiled topology | Build/compare projected endpoint set; query direct edge membership. | Source-law resolution, duplicate-state policy, action-specific transition identity, schema validation. |
| Allowed/forbidden moves | Check whether the reported source-to-target edge is present; replay direct state sequences. | Determine event/action applicability, guard result, priority, context/history, lawful verdict. |
| Position after a sequence | `MachineInstance` maintains expected structural position and failure atomicity. | Verify event chronology, emitted effects/events, and expected VPE state semantics. |
| Structural equivalence between definitions | Build `Machine`s and compare exact topology equality; normalize using a VPE mapping first if names differ. | Compare law version, action/guard/effect semantics, manifests, compatibility. |
| Simulation | Check simulation-generated moves against candidate edges after simulation exists. | Baseline/candidate process outcome, replay semantics, classification and simulation policy. |
| Migration | Check mapped source/target topology before/after migration after migration exists. | State mapping, history lift, compatibility report, semantic preservation. |

## 5. Capability and gap matrix

| Capability | Current Statekit status | Easy consumer implementation? | Small reusable API value | Complexity / performance / scope assessment |
|---|---|---|---|---|
| 1. Transition enumeration and outgoing adjacency | **Already supported** by `transitions()` and `targets_from()`. | Yes. Collect and normalize for tests. | None apparent. | Current iterators are allocation-light; order is unspecified. Preserve BI-002 by avoiding public storage details. |
| 2. Incoming transitions / predecessors | **Not supported directly.** | Yes: scan `transitions()` and filter target. | Moderate for repeated reverse queries or cleaner generic analysis. | A scan is O(E) per query and needs no index; a stored reverse index adds O(E) construction work and memory. Does not change an invariant. Require real repeated-query evidence. |
| 3. Reachability from selected start | **Not supported.** | Yes: BFS/DFS over `targets_from()`, using a visited set. | Highest candidate for structural analysis because rooted reachability is reused by VPE and other consumers. | O(V+E) traversal and O(V) working memory; repeated calls may justify caching only if measured. Root must be caller-supplied. Report facts; do not reject disconnected graphs by default. |
| 4. Path existence | No direct query; only one-edge membership. | Yes: BFS/DFS using current adjacency. | Some value if multiple consumers repeatedly ask whether any route exists. | O(V+E), O(V) memory. Start/goal semantics and unknown-state result need a clear contract. Avoid path enumeration/shortest path without demand. |
| 5. Validate a transition sequence | No whole-sequence method; `MachineInstance::transition_to` validates each step. | Trivial loop with one instance; caller knows the failed index from its loop. | Low today. An optional validator returning failing position might help replay/config tools, not specifically tests. | O(k) edge checks. Whole-sequence API may raise atomicity/error-report questions; no test-specific API is warranted. |
| 6. Reset/reconstruct a reference instance | No `reset()` or `Clone`; `Machine::instance(initial)` creates a fresh position. | Yes. Construct per generated test case. | Low; existing constructor is enough. | O(1) shared-machine clone plus owned current name under current internals; no new API needed. Keep representation contract hidden. |
| 7. Compare machine definitions | **Already supported** by `Machine: PartialEq + Eq`. | Yes; for non-Statekit SUTs, compare normalized transition sets. | No new API needed for same-type definitions. | Equality is exact logical topology and independent of shared allocation. It does not imply process/behavior equivalence. |
| 8. Structural transition coverage | No coverage accumulator/report. | Yes: count visited endpoint pairs against `transitions()` in the test harness. | Little library value; coverage goals vary by test/application. | Coverage policy, thresholds, generated paths, and reports are test-framework responsibilities. Avoid built-in coverage APIs. |
| 9. Deterministic enumeration/order | **Not promised**; docs say unspecified. Query truth is deterministic. | Yes: sort or compare sets in assertions. | Only if external consumers require reproducible serialization/output. | Stable order constrains representation and may affect cost; would strengthen BI-002. No testing evidence requires it. |
| 10. Structurally impossible transitions | Direct absent-edge queries are **already supported** by `can_transition` / `validate_transition`; instance queries use current position. | Yes. | No API for direct forbidden edges. | A non-edge is structurally forbidden; a multi-hop-reachable target is still not a permitted single transition. Path reachability is distinct and would be a later analysis query. |

## 6. Capability categories

### A. Already supported

- Use `Machine` as a topology oracle for direct edge membership, sources/targets, states, and terminality.
- Use `MachineInstance` as a simple structural state position, replay one allowed move at a time, and assert failed moves preserve state.
- Build fresh reference instances for each case; compare the reference position with SUT-visible state.
- Compare two Statekit definitions with `Machine` equality; compare a non-Statekit implementation through a normalized edge set.
- Generate tests externally with Proptest, QuickCheck, Hypothesis, or another runner.
- Follow Statekit's existing property-test pattern: separate an edge-set model from the library and compare public operations against that model.

### B. Consumer-implemented

- Generate valid target operations from the current state's `targets_from` result and invalid operations from other states/unknown values.
- Replay sequences and report the first invalid command index.
- Reinitialize a reference machine from a chosen starting state.
- Compute edge coverage by tracking which `Transition`s were exercised.
- Compute predecessors or rooted reachability with a small local traversal.
- Compare a SUT's edge set, state value, accepted/rejected result, and post-failure state.
- Map events/actions to target-state attempts and decide which transitions should be considered applicable under guards/policy.

### C. Small reusable improvements (proposals)

1. **Caller-rooted reachability / reachable-state iteration.** Strongest structural candidate; could reduce repeated local BFS implementations and provide an independent cross-check for VPE. Return observations, not build errors. Unknown-root behavior and placement (`Machine` query versus future analysis view) remain open.
2. **Incoming-edge/predecessor query.** A clean dual to `targets_from`; repeated reverse checks may justify it. A scan implementation can avoid new index memory, though its repeated cost is O(E).
3. **Possibly a path-exists query.** Defer until a consumer demonstrates a need beyond a local BFS or rooted reachable set.

None is required for reference-model testing. Any API proposal should be checked against current version/API stability, INV-001 through INV-014, memory benchmarks, and caller use before acceptance.

### D. Future research

- State identity options (`String`, resolved IDs, generic IDs) for VPE and other compiled consumers.
- Portable topology snapshot/projection and structural equivalence across versions or name mappings.
- Optional delayed/accumulated diagnostics and source provenance; these interact with INV-007's earliest-error behavior.
- Structural path/cycle/SCC analyses only after a consumer supplies a test or analysis case.
- How to test concurrency/races: Statekit BI-003 leaves scheduling/synchronization to the host; current sequential `MachineInstance` properties do not test a consumer's concurrent update protocol.
- How VPE simulation and migration eventually represent structural traces and mappings; those modules are currently stubs.

### E. Out of scope

- A Statekit command generator, property-test runner, random strategy, shrinker, seed/replay store, fixture system, coverage collector, HTML report, or test orchestration API.
- Statekit interpreting events/actions, guards, priorities, payloads, effects, context/history, async operations, or application-specific failure/retry behavior.
- Statekit declaring that every state must be reachable, that a terminal state is success, that a topology must be complete, or that a process must terminate.
- Statekit validating VPE's business/process semantics or implementing process simulation/migration.
- General graph algorithms disconnected from state-machine consumer needs.

## 7. Invariant and design-intent impact of proposals

| Proposal | Project Core impact | Assessment |
|---|---|---|
| Use existing `Machine` as test oracle | Supports I-001/I-005 and INV-006, INV-009, INV-010, INV-011, INV-012. No new invariant. | Fits existing model directly. Consumer owns generation and semantic assertions. |
| Add incoming-edge query | Preserves immutability, logical edge identity, error timing, equality, and runtime configuration. Must remain read-only and not expose internal storage. | Low semantic risk; memory/performance depends on scan versus index. BI-002 argues for keeping representation hidden. |
| Add rooted reachability | Preserves machine validity rules if observational only. Should not redefine disconnected topologies as invalid or add canonical start state. | Strongest proposed analysis; O(V+E), temporary O(V) set. Caller-supplied root avoids introducing initial-state semantics. |
| Add path query | Could be a derived query over existing topology; does not alter machine construction. | Moderate API value but overlaps reachable set and local consumer code. Do not turn into pathfinding policy/all-path enumeration. |
| Add sequence validator/reset/clone | Could preserve immutability and failed-transition atomicity, but `MachineInstance` already validates each step and `Machine::instance` already reconstructs it. | Weak case for new API; likely convenience duplication. Sequence-level partial progress/error index contract adds decisions. |
| Add coverage APIs or generated test sequences | Not directly required by Statekit's machine invariants. | Conflicts with BI-001 building-block boundary by taking on testing-framework work. Keep in host/test tools. |
| Promise stable enumeration | Would be compatible with immutability but strengthens observable behavior and constrains hidden representation/performance (BI-002, I-002, D-011). | No current consumer evidence; normalize in tests instead. |
| Generic state IDs | May help already-resolved VPE topology; could impact public API compatibility and name validation semantics. | Separate identity investigation; not necessary to test reference-model value. |

## 8. Prioritized recommendations

Recommendations are evidence-weighted proposals, not accepted architecture.

1. **Use the existing API for an external reference-model experiment.** Add no production API first. A small VPE or config-driven consumer test can project an explicit topology, replay operations through `MachineInstance`, and compare state/result after each operation. This directly measures usefulness before feature design.
2. **Treat Statekit as a structural oracle, never as the full expected-behavior specification.** Keep independent expected fixtures and consumer-owned semantic assertions. A passing topology check is necessary evidence for an emitted edge, not proof of a valid action/guard/effect decision.
3. **Keep generation, shrinking, replay, and coverage in test frameworks.** Proptest's state-machine support already separates a reference model and SUT and shrinks sequences; Statekit supplies only generic topology/traversal where useful.
4. **If an actual consumer repeats BFS, evaluate rooted reachability next.** It may reduce duplicate code and support cross-checks, but leave unreachable policy to the caller. Compare with local traversal and benchmark expected graph sizes first.
5. **Evaluate predecessors after reachability or with an actual reverse-query consumer.** It is easy to compute from `transitions()` today. Add only if repeated query code and scale justify a public method or bounded index.
6. **Do not add clone/reset, whole-sequence test helpers, coverage reports, or stable ordering on present evidence.** Existing construction and query methods handle these needs simply in caller code.
7. **For VPE, prefer Architecture B for the first experiment.** Keep VPE's independent compiler/evaluator, add only a development-time projection/reference comparison, and explicitly test compatibility for empty edge sets, self-edges, same-endpoint semantic transitions, and identity mapping. Decide on Architecture A only if measured duplication/runtime needs remain after the experiment.

## 9. Architectural risks and test-quality limitations

- **Structural/semantic mismatch:** An edge in topology says only that a directed move exists. It does not say an event/action can trigger it or guards pass.
- **Common-mode error:** If Statekit and the SUT receive the same bad parse/config projection, they can agree on the wrong topology. Use independent fixtures, parser tests, and test expectations from another source.
- **Oracle overreach:** Statekit's no-self-edge and non-empty-machine invariants are its own contract. Consumers with those cases need an explicit projection policy; do not silently treat projection failure as a domain error.
- **Parallel semantic transitions:** Statekit deduplicates endpoint-equal edges. VPE edges with different actions, priorities, guards, or effects must remain distinct in VPE; topology equality intentionally forgets those fields.
- **Partial traces:** A failing operation should be checked for unchanged SUT and model state. A multi-step helper should make clear whether earlier successful steps remain applied when a later step fails. Fresh instances are the simplest test isolation.
- **Coverage illusion:** Visiting every edge structurally does not cover every guard outcome, action, data boundary, history condition, effect, or state/context combination.
- **State identity conversion:** Current Statekit names are strings, while VPE compiled edges use indices. Conversion is acceptable in test fixtures but may be a cost or source of bugs in production integration.
- **Iteration order:** Existing order is unspecified. Tests should normalize outputs; deterministic behavior means the same query membership/result, not a promised traversal order.
- **Concurrency:** Statekit's instance independence is not a concurrency-testing framework. Host locking, transaction isolation, races, and synchronization require consumer-specific tests.
- **Correlation with Statekit tests:** The existing HashSet model is a good independent edge-set oracle for library API behavior, but it shares generated topology input. It should not be overstated as proof of consumer configuration correctness.

## 10. Open questions for later architectural discussion

- Is Statekit's current `MachineInstance` enough for consumers beyond plain adjacent-edge traversal? Which concrete test needs something more?
- Does sequence validation offer value outside test code (for example, validating imported/replayed traces), or is a caller loop simpler and clearer?
- If a sequence validator were justified, should it live on `Machine` (start state plus sequence) or on an instance (continuation from current position)? Should it mutate, clone, or only return a final state/error index?
- Does any caller need instance cloning/reset, or does constructing a fresh instance meet isolation needs at negligible cost?
- Can state/edge coverage remain a test helper derived from `transitions()` and generated operations? Current evidence says yes.
- Should consumers normalize enumeration results themselves rather than request a stable ordering contract? Current docs say order is unspecified; changing that promise has compatibility/performance cost.
- How should tests model guard-rejected edges? Structural allowedness should be checked separately from the expected semantic rejection.
- How can Statekit avoid common-mode bugs when the topology model and SUT are both built from the same source? Which fixture or specification is authoritative?
- Would rooted reachability meaningfully reduce VPE's BFS or mostly duplicate compiler validation? Does it deserve a public API for other consumers?
- Would a generic/resolved state ID make direct VPE projection safer or faster, and would it preserve the simple string API and current name-validation invariants?
- What minimal `Machine` projection should VPE expose to tests without making state identity, edge identity, or serialization part of Statekit's contract?
- When VPE simulation and migration are implemented, which structural assertions can be independent and which require VPE semantic reference models?

## 11. Primary sources

### Statekit and VPE

- [Statekit README](https://github.com/whitephoenixsoft/statekit/blob/main/README.md)
- [Statekit Project Core](https://github.com/whitephoenixsoft/statekit/blob/main/docs/project-core.md)
- [Statekit specification](https://github.com/whitephoenixsoft/statekit/blob/main/docs/statekit-specification.md)
- [Statekit roadmap](https://github.com/whitephoenixsoft/statekit/blob/main/docs/roadmap.md)
- [Statekit property tests](https://github.com/whitephoenixsoft/statekit/blob/main/tests/property_tests.rs)
- [Previous investigation: Consumer Research & Capability Discovery](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md)
- [VPE README](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/README.md)
- [VPE design philosophy](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/DESIGN_PHILOSOPHY.md)
- [VPE compiled process representation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compiled.rs)
- [VPE compiler validation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/validate.rs)
- [VPE compiler lowering](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compile.rs)
- [VPE runtime evaluator](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/runtime/evaluate.rs)
- [VPE crate manifest](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/Cargo.toml)

### Testing libraries

- [Proptest: state-machine testing](https://proptest-rs.github.io/proptest/proptest/state-machine.html)
- [`proptest-state-machine` crate](https://docs.rs/proptest-state-machine/latest/proptest_state_machine/)
- [QuickCheck for Rust](https://docs.rs/quickcheck/latest/quickcheck/)
- [Hypothesis stateful testing](https://hypothesis.readthedocs.io/en/latest/stateful.html)
- [XState v6 model-based testing](https://dev.stately.ai/docs/xstate/v6/model-based-testing) — current page labels v6 alpha.
- [XState v6 graph and paths](https://stately.ai/docs/graph)

---

## Conclusion

Statekit already has the core of a useful structural reference model, and its own property suite demonstrates the pattern. The highest-value use is to check a consumer's direct state topology and position behavior while a separate testing framework handles commands, generated sequences, shrinking, and coverage. For VPE, a test-only oracle is the least-coupled experiment; direct production integration is not currently justified by source evidence and has edge/identity compatibility questions. No new Statekit functionality is necessary to establish this value. Reachability remains the most plausible future analysis addition if real consumers show repeated need.
