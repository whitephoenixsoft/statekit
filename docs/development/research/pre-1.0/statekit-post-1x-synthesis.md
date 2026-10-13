---
project: statekit
category: research
status: draft
review-status: needs-review
---

# Statekit Post-1.0: Structural Analysis, Engine Integration, and Cargo Features

**Research synthesis — October 2026**  
**Scope:** Findings from Statekit FSM-crate comparisons, VPE code analysis, the draft post-1.0 roadmap, and the discussion of Cargo features. This is a planning note, not an approved API specification or implementation commitment.

## Executive summary

Statekit's strongest prospective role is **a reusable structural foundation for runtime-defined finite-state machines**, including applications and authors of FSM/process engines. It should continue to **validate machine topology, expose structural facts, and safely traverse validated machines**, without taking ownership of guards, actions, effects, priorities, or process meaning.

The current plan remains unchanged: **finish Statekit 1.0, use it as the structural foundation for VPE, and use actual integration friction to prioritize Statekit 1.x enhancements**. The post-1.0 roadmap proposes local topology inspection in v1.1 and transitive reachability in v1.2. Advanced graph analysis and Petgraph remain possibilities, not commitments.

Cargo features could support this evolution without making every consumer carry optional runtime or analysis capabilities. The favored *candidate* configuration is an always-available `machine` foundation with optional `instance` and `analysis` features, both enabled by default.

## 1. Architectural identity and boundaries

Proposed direction from the roadmap:

> Statekit validates, inspects, and safely traverses data-defined state machines.

Three complementary responsibilities:

1. **Validate:** Ensure constructed machines satisfy Statekit's structural invariants; reject invalid definitions early; preserve immutability after construction.
2. **Inspect/analyze:** Report facts about direct relationships, reachability, and eventually other graph properties.
3. **Traverse:** Allow instances to move only along valid structural transitions, without changing state after a failed transition.

**Boundary rule:** If a question can be answered solely from states and structural transitions, it is a candidate for Statekit. If it requires guards, events/actions, execution context, history, priority, effects, or workflow meaning, it belongs to VPE or another host.

Analysis should be **descriptive rather than prescriptive**: a root need not be a designated initial state; a terminal need not represent successful completion; a cycle need not be erroneous; and an unreachable state need not invalidate a machine.

## 2. Lessons from comparisons with other FSM crates

Comparisons included `state_machine_future`, `fsm-rs`, `rust-fsm`, `statig`, `smlang`, `typed-fsm`, `fsm`, `fsm-runtime`, `state-machines`, and `sfsm`.

The recurring distinction is between **compile-time/type- or macro-driven FSM definitions** and Statekit's **runtime, data-defined topology**. Frameworks may own generated dispatch, guards, events, hierarchy, async execution, or actions. These features do not automatically belong in Statekit. The additional `state-machines` and `sfsm` comparisons reinforce the potential for Statekit to complement typed or generated FSM engines with structural validation and diagnostics when dynamic or intermediate machine definitions are involved.

**Synthesis:** Statekit could serve authors of FSM engines as a focused validation/analysis facade, without becoming a general FSM framework or a general graph library.

## 3. VPE as a real consumer and test case

Inspection of the existing VPE implementation identified several generic graph mechanisms implemented locally:

- **Reachability:** BFS using `HashSet` and `VecDeque` to find states unreachable from the law's designated initial state.
- **Cycle detection:** DFS with visiting/visited sets for cycles formed by `AUTO_TICK` transitions.
- **Terminal detection:** Checking whether a state has no outgoing transitions.
- **Compiled topology:** State-to-index mapping and `Node`/`Edge` structures for runtime execution.

This supports Statekit providing generic traversal and structural facts, while VPE decides whether a finding is a warning, error, or permitted condition.

### Important integration risks to test before promising seamless reuse

1. **Parallel semantic transitions:** VPE can have multiple transitions with the same source and target but different actions, priorities, or guards; Statekit rejects duplicate *structural* transitions. A promising adapter is to deduplicate source/target pairs in Statekit while keeping VPE's richer transition candidates separately. This needs real integration testing.
2. **Self-transitions:** Statekit rejects them. Whether VPE laws need them should be verified rather than assumed.
3. **Empty-transition laws:** Statekit requires at least one transition; some process models may contain a single terminal state. Verify whether VPE needs this case.
4. **Filtered analysis:** `AUTO_TICK` cycle detection concerns a semantic subset of edges. An unlabeled Statekit topology alone cannot distinguish that subset. VPE may need to build a filtered view or supply a filter; this is not yet an established Statekit requirement.
5. **Initial-state ownership:** VPE designates an initial state; Statekit does not. `reachable_from(start)` is therefore a better primitive than a machine-global `unreachable_states()` assumption.

VPE is evidence of *potential* shared functionality, not proof that every historical VPE implementation detail belongs in Statekit.

## 4. Current post-1.0 roadmap

### Phase 1 — Additional Identity (tentative v1.1)

**Goal:** Establish Statekit's additional inspection identity and update the Project Core with relevant values, requirements, and invariants as appropriate.

Candidate methods:

| Method | Structural meaning | Assessment |
|---|---|---|
| `sources_to(state)` | Iterator over unique states with a **direct** transition into `state` | Strong candidate |
| `roots()` | States with zero incoming transitions | Strong candidate |
| `terminals()` | States with zero outgoing transitions | Strong candidate |

These are immediate-topology queries. They need no advanced graph dependency. A valid machine can have zero roots, zero terminals, or multiple of either. `sources_to` should not be confused with transitive predecessors.

### Phase 2 — Analysis Additions (tentative v1.2)

Candidate methods:

| Method | Structural meaning | Assessment |
|---|---|---|
| `reachable_from(state)` | States reachable along zero or more transitions | Highest priority; directly addresses VPE's BFS |
| `can_reach(from, to)` | Whether a path exists | Strong, likely derived from the same traversal |

Before implementation, specify whether zero-length paths count. Conventional reachability includes the source state itself, so `can_reach(A, A)` can be true even though a direct `A → A` transition is forbidden. This is a semantic/API decision, not a contradiction.

An `analysis` Rust module, a public `MachineAnalysis` object, and a Petgraph-backed representation are **three separate design decisions**. The roadmap commits to none of the latter two.

### Deferred candidates

- `can_reach_terminal(state)`
- `has_predecessors(state)` / `has_successors(state)`
- `in_degree(state)` / `out_degree(state)`
- `has_cycles()` (machine-level)
- `cycles()` (requires defining which cycles and result form)
- `is_cyclic(state)` (explicitly: whether the state *belongs* to a cycle, not merely whether it can reach one)
- `instance.reachable_states()` / `instance.can_reach(target)` (conveniences that should reuse machine-level analysis)
- Potential future strongly connected components, path queries, and filtered graph traversal, if consumer demand justifies them

Avoid redundant aliases where existing outgoing/incoming APIs already provide the same capability. Enumerating **all simple cycles** can be expensive; do not promise it casually.

## 5. How deep should analysis become?

**Level 1 — Lightweight structural inspection:** incoming/outgoing relationships, roots, terminals, reachability, simple cycle existence. Small API, straightforward algorithms.

**Level 2 — Reusable FSM analysis facade:** an optional analysis representation, indexes, strongly connected regions, structured results, and possibly filtered views. This is a plausible eventual destination for supporting engine authors.

**Level 3 — Broad graph-analysis platform:** dominators, advanced path enumeration, condensation, graph comparisons, and sophisticated diagnostics. High API and maintenance cost; no current commitment.

The most promising exploration is **Level 2**, contingent on VPE or other consumers showing demand. A facade is valuable not simply because it hides Petgraph, but because it presents graph algorithms in **FSM terminology and state identities** while leaving policy to the host.

## 6. Petgraph: costs and implementation options

Petgraph is **not required** for the proposed v1.1 queries or basic v1.2 BFS/DFS. If advanced analysis is later added, assess three costs independently:

- **Dependency/build footprint:** Extra dependency and compiled code; actual binary effect requires measurement and depends on use and linking.
- **Runtime/memory:** A duplicate graph/index may increase construction cost and memory. Avoid imposing this on ordinary machine creation and traversal.
- **Architectural maintenance:** Public algorithms, result types, complexity promises, testing, documentation, and compatibility are likely the most durable costs.

A potential architecture is an immutable `Machine`, optional `MachineInstance`, and an optional analysis facade that builds/caches graph-specific structures only when needed. This is a **candidate**, not an adopted design. Keep Petgraph's internal node/edge types out of public contracts unless there is a compelling reason to expose them.

## 7. Cargo feature proposal

The discussion introduced a new post-1.0 candidate: **default to the complete Statekit experience while allowing selective compilation**.

Recommended dependency shape:

```text
machine (foundational, always available)
├── instance (optional)
└── analysis (optional)
```

Preferred illustrative `Cargo.toml` configuration:

```toml
[features]
default = ["instance", "analysis"]
instance = []
analysis = []
```

`machine` remains a module/API foundation rather than a Cargo feature because neither instance nor analysis is useful without it. This is simpler than exposing a `machine` flag that all meaningful configurations must enable.

Examples for consumers:

```toml
# Default: all capabilities
statekit = "1.x"

# Machine only
statekit = { version = "1.x", default-features = false }

# Machine + analysis, no instance runtime
statekit = { version = "1.x", default-features = false, features = ["analysis"] }
```

If Petgraph becomes useful later, an optional dependency can be enabled by `analysis` or by a distinct `advanced-analysis` feature. **Do not introduce an advanced-analysis split before there is a measurable reason**.

Cargo features are **additive and unified across a dependency graph**. `default-features = false` on one dependency declaration does not guarantee another dependency will not enable a feature.

### Feature testing matrix

```bash
cargo test --no-default-features
cargo test --no-default-features --features instance
cargo test --no-default-features --features analysis
cargo test --all-features
```

Tests and exports must be correctly feature-gated. Preserve the full public API for default-feature consumers, subject to independent compatibility review.

## 8. Guardrails for implementation

These principles are copied or directly developed from the revised roadmap:

1. **Compatibility:** Preserve existing public methods unless a change is independently justified.
2. **Performance:** Ordinary construction and instance traversal should not pay for unused analysis.
3. **Semantics:** Analysis reports structural facts, not domain judgments.
4. **Reuse:** Instance-relative queries delegate to machine-level analysis.
5. **Dependencies:** Petgraph is an option, not a requirement.
6. **Testing:** Each analysis feature gets focused unit tests and useful property tests, traceable to the Project Core.

Suggested reachability properties include closure under outgoing edges, immediate successors being reachable, and adding a valid edge never shrinking a reachable set. The detailed specification should define invalid-state behavior and iterator/result ordering explicitly.

## 9. Recommended sequence and open decisions

1. **Complete Statekit 1.0** without broadening its scope based solely on research.
2. **Integrate VPE** and document duplicated generic traversal, topology adaptation, and actual performance/ergonomic pain.
3. **Revisit v1.1** immediate-topology APIs and Project Core identity; consider whether Cargo feature boundaries should be introduced alongside this work.
4. **Revisit v1.2** reachability APIs; choose simple algorithms first unless evidence supports more infrastructure.
5. **Consider deeper analysis** only after consumer use establishes the needed semantics, cost envelope, and API vocabulary.

Open questions to resolve in future specifications:

- Is `MachineAnalysis` a query view, cached snapshot, or hybrid?
- Should reachability include the starting state by default?
- How should filtered/semantic-subgraph analysis work without embedding VPE concepts?
- What is the smallest lossless mapping from VPE's rich transitions to Statekit's unique structural edges?
- Do VPE's self-transition or zero-transition cases require adaptation or a deliberate boundary?
- Does Petgraph measurably improve maintainability/performance enough to justify its footprint?
- Which Cargo feature combinations are public commitments, and what are their compatibility guarantees?

## Conclusion

**Keep Statekit small at its foundation and ambitious only where reusable structural facts justify it.** Its emerging value is as a trustworthy FSM topology validator, inspector, and traversal layer that other engines can build upon. The immediate roadmap remains modest; optional Cargo features can protect lightweight consumers if analysis grows. VPE is the next practical test of where that boundary should move.

---

**Reference context:** User-authored `roadmap-1.x.md` (draft), Statekit project-core/specification materials, the project's FSM crate comparison conversations, and inspection of `whitephoenixsoft/verified-process-engine` compiler/runtime source. Method names, versions, and architectural sketches above are proposals unless already documented as existing APIs.
