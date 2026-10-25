# Statekit Consumer Research & Capability Discovery

**Status:** Research proposal, not an architectural decision  
**Scope:** Configuration-driven state-machine validation and structural consumer needs  
**Repository reviewed:** `whitephoenixsoft/statekit`, default branch `main`, v0.4.0 manifest  
**Research date:** 2026-10-12

## Executive summary

There is a practical, bounded consumer role for Statekit: validate a runtime-provided list of directed state transitions, retain it as an immutable definition, answer adjacency questions, and let independent instances traverse allowed edges. This corresponds to the structural core found inside much larger state-machine and workflow products.

The clearest real consumer example is configuration validation before deployment. AWS Step Functions accepts a JSON state-machine definition and offers a validation API that reports errors and warnings with codes and, where available, a JSON pointer-like location. Python StateMachine demonstrates a different need: validating graph properties such as unreachable states, traps, and final-state reachability, with checks disabled for some loaded documents because those properties may be policy-dependent. GitLab CI configuration is an adjacent example: YAML validation can be followed by pipeline simulation to catch dependency issues. Its jobs form a dependency graph, not the same model as Statekit's finite-state runtime, so it is evidence for graph validation needs rather than a direct feature match.

Statekit already supplies the foundational subset: fail-fast edge/name validation, a non-empty immutable machine, state and transition enumeration, outgoing adjacency, terminality, edge checks, and stateful traversal. It does not currently accept a standalone list of declared states, parse formats, provide accumulated diagnostics, analyze reachability, or report source locations. The strongest immediate opportunities are better consumer examples and clearer positioning. A small structural-analysis addition such as incoming edges or reachability from a caller-supplied starting state is plausible, but should remain a proposal until a concrete consumer needs it and its cost/API shape are tested.

**Key boundary:** Statekit can say what the transition topology permits. A consumer decides whether that topology is complete, reachable from its configured start, semantically meaningful, safe, or executable.

## Evidence labels and limits

- **Verified** means supported by the supplied project documents, current public `main` source, or a primary source linked below.
- **Interpretation** means a synthesis of verified evidence.
- **Hypothesis** means a proposed consumer or opportunity that needs user or implementation evidence.

This review does not establish the number of external Statekit users. Public source and documentation show intended use, not adoption. Crate downloads alone would not prove active consumers.

## Current Statekit baseline

The supplied README, Project Core, specification, and roadmap were read before evaluating gaps. The live repository was then checked against those documents. The repository's current `main` branch reports package version `0.4.0` in `Cargo.toml`.

### Verified existing behavior

- `MachineBuilder::try_allow(from, to)` validates endpoint names and rejects self-transitions as each edge is added. Duplicate logical edges are ignored.
- `build()` rejects an empty transition set and constructs an immutable `Machine`.
- The topology is implicit in transition endpoints; there is no public operation to declare an isolated state without an edge.
- `Machine` can answer `contains_state`, `can_transition`, and `validate_transition`; it exposes counts, all states, source states, transitions, and outgoing targets through `targets_from`.
- `Machine::is_terminal(state)` reports whether a state has no outgoing edges. Unknown states also have no outgoing edges, so callers should check `contains_state` first when unknown and terminal must be distinguished.
- `Machine::instance(initial)` checks that the initial state exists. `MachineInstance` exposes its current state, tests and performs one allowed adjacent transition, and exposes terminality. Failed transitions leave the instance unchanged.
- A built machine is immutable and shared by instances through an `Arc`-backed internal representation. The internal representation is explicitly outside the domain contract.
- `StateError` is a typed enum, implements standard Rust error integration, and includes offending names/endpoints for invalid names, invalid edges, self-transitions, and unknown initial states. `NoTransitions` is a unit variant.
- The builder is fail-fast and consuming (`try_allow` returns `Result<Self, StateError>`). There is no import/parser adapter or accumulated diagnostic report in the inspected public API.
- The API returns iterators with unspecified order. Callers must not treat enumeration order as stable.
- The Project Core says earliest detectable error reporting is an invariant, and the specification locates endpoint validation at transition addition and machine-level validation at build time.
- Existing structural rules deliberately allow cycles. The documents do not make reachability, a single component, a required initial state, or terminal/final-state policy machine invariants.

### Relevant source links

- [Current README](https://github.com/whitephoenixsoft/statekit/blob/main/README.md)
- [Builder API and validation timing](https://github.com/whitephoenixsoft/statekit/blob/main/src/builder.rs)
- [Machine query and inspection API](https://github.com/whitephoenixsoft/statekit/blob/main/src/machine.rs)
- [Machine instance traversal](https://github.com/whitephoenixsoft/statekit/blob/main/src/instance.rs)
- [Typed errors](https://github.com/whitephoenixsoft/statekit/blob/main/src/error.rs)
- [Crate exports](https://github.com/whitephoenixsoft/statekit/blob/main/src/lib.rs)
- [Project Core document supplied for this review](project_sources/02-project-core-2-.md)
- [Statekit specification supplied for this review](project_sources/01-statekit-specification.md)
- [Roadmap supplied for this review](project_sources/04-roadmap-2-.md)

## Consumer use-case catalog

These are plausible application patterns, not claims of known Statekit customers.

| Consumer pattern | Concrete example | Structural work Statekit can do today | Work left to consumer |
|---|---|---|---|
| Tenant- or user-defined lifecycle | An issue tracker lets each organization configure `open -> triaged -> active -> resolved`, plus `active -> blocked`. | Build and freeze configured edges; reject malformed names/self-edges/empty graph; query legal adjacent moves; create an instance at a known state. | Parse/authorize saved config; decide initial state; actor permissions, guards, audit, side effects, and whether terminal states mean “done.” |
| Database-backed entity status | A commerce service loads allowed order-status edges from a versioned table at startup or tenant refresh. | Reuse an immutable machine definition across many entity instances; validate direct transitions; expose topology for UI/admin display. | Persistence, config versioning/hot reload, concurrency, transaction boundaries, historical compatibility, and authorization. |
| Plugin-provided lifecycle | A host loads a plugin's transition declarations for a device, document, or job. | Check and use plain structural edges without baking plugin states into Rust enums. | Plugin schema/versioning, trust/security, callback resolution, and whether the plugin's promised behavior is implemented. |
| Pre-deployment configuration gate | A CI job reads a generated state-transition table and checks each declared edge before deployment. | Validate its structural subset and exercise expected edge membership. | File parsing, line/path diagnostics, environment-dependent validation, policy such as “all states reachable from start,” and deployment integration. |
| Runtime state exploration | A service or simulation starts an instance at a selected known state and applies transitions as application events arrive. | Verify that each requested adjacent move is in the shared definition; isolate each entity's current state. | Event-to-transition mapping, guard conditions, effects, retries, external I/O, and event history. |
| Structural projection from a richer compiler | VPE compiles a process/law, projects just source and target states, and asks Statekit to validate/traverse that topology. | Supply a generic structural layer and instance traversal, with no VPE dependency. | Resolve names/IDs, events, guards, data-flow, execution result, simulation, and migration stay in VPE. |
| Imported state-machine document | An application imports a state chart from JSON/YAML/SCXML or a vendor DSL. | Once an adapter has extracted edges, Statekit can validate and represent that edge set. | Format parsing, schema validation, hierarchy/parallel-region normalization, source locations, and vendor semantics. This is a future adapter question, not current Statekit functionality. |

### What a “small validation dependency” buys a consumer

**Interpretation:** A consumer that already owns a configuration format could use Statekit after parsing, as a structural checker and immutable topology object. It need not adopt Statekit's file format or runtime semantics. This is the most composable integration: consumer parser/diagnostics -> edge projection -> Statekit `Machine` -> consumer policy/runtime.

**Hypothesis:** The value is highest when the application's states are genuinely runtime-defined and many entities share one topology. For a small fixed set of compile-time states, ordinary Rust enums and exhaustive matching may be simpler and safer. Statekit should position itself for the former case, not compete with typed state representations generally.

## Comparative capability matrix

| Library/system | Problem and definition form | Structural validation | Analysis / introspection | Diagnostics | What the comparison suggests for Statekit |
|---|---|---|---|---|---|
| **Statekit (Rust, v0.4.0)** | Dynamic flat edges built through `MachineBuilder`; source/target names are strings. | Validates endpoint spelling, rejects self-edges, deduplicates equivalent edges, requires at least one edge, checks instance initial state and requested adjacent transitions. | States, sources, transitions, outgoing targets, terminality, counts, edge lookup, current instance state. No general reachability or reverse-adjacency API verified. | Typed `StateError`; fail-fast, with names/endpoints on applicable variants. No source-file location or warning channel. | Small structural foundation already exists; most visible gaps are importer-facing docs and selected analysis, not a need for a broad schema/workflow system. |
| [**pytransitions** (Python)](https://github.com/pytransitions/transitions) | Object-oriented runtime FSM; states/transitions can be passed as Python data structures; extensions add hierarchy, callbacks, async, diagrams. | The comparison in python-statemachine's docs says `transitions` does not perform its set of consistency checks for unreachable/trap states or nonexistent callback names. This should not be generalized into “no validation of any kind.” | GraphMachine can render graph diagrams; its extension set supports hierarchical state machines and runtime actions. | Many invalid configurations/callback references may only surface when operations execute; this differs from early static checking. | A consumer often wants definition-time validation before state is exercised. Statekit can stay below event/callback concerns and still offer useful topology checks. |
| [**python-statemachine**](https://python-statemachine.readthedocs.io/en/latest/validations.html) | Class-based statecharts and imported SCXML/JSON/YAML documents. | Checks include one initial state, final-state transition restrictions, unreachable states, traps, final-state reachability, callback names, and guard expression syntax. Some structural checks are intentionally disabled for loaded docs because parallel/eventless paths or intentional trap/error states make assumptions unsafe. | Graph rendering and structural graph checks; loaded-document tooling includes JSON Schema validation. | Raises definition/instance errors with messages; checks can be configured. Loaded documents' disabled checks can mean a real structural mistake goes unreported at load. | Strong evidence that reachability/trap rules are policy/context dependent. Statekit should expose facts separately from mandatory invariants and let a consumer decide acceptance. |
| [**AWS Step Functions / Amazon States Language**](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html) | A JSON ASL document specifies named states, `StartAt`, `Next`, branch/choice behavior, task resources, and terminal states. | Dedicated validation API checks document syntax/schema and state-machine rules, including dangling transition targets, missing terminal state, invalid resource, and excessive nesting. | Service executes the workflow; validator's scope is definition validation, not a reusable graph-analysis SDK. | Returns `OK`/`FAIL` plus zero or more `ERROR`/`WARNING` diagnostics with stable code field, message, and optional JSON-pointer-like location. AWS advises clients not to depend on diagnostic ordering/count/message text. | Real proof of pre-deploy configuration validation need. A small library could provide typed structural facts; format-specific schema and rich locations belong in adapters/hosts. Warning/error distinction is useful, but adds API surface and policy questions. |
| [**XState (JavaScript/TypeScript)**](https://stately.ai/docs/graph) | Declarative machine/statechart object with initial state, event-to-target transitions, nested/parallel/final states, actions, context, and actors. | Configuration and TypeScript typing constrain many shapes; machine semantics include much more than a flat edge list. | Current docs describe `xstate/graph` path generation (`getShortestPaths`, `getSimplePaths`) and test-oriented path coverage. | Developer-facing config/type/runtime diagnostics; graph-path APIs return paths with states and event steps. | Demonstrates demand for topology exploration, but its path generation is event-semantic/model-based and not an appropriate scope template for Statekit. A modest one-path/reachability query is a different, simpler capability. |
| [**statig (Rust)**](https://docs.rs/statig/latest/statig/) | States are Rust methods with attributes; an event is dispatched to the current handler; supports hierarchy, state-local storage, actions, context, async/no_std. | Rust source/attributes and generated types structure the machine; this is not a runtime-loaded edge-table validator. The checked docs emphasize execution model rather than loaded configuration diagnostics. | Current state and dispatch/transition hooks for instrumentation; not a general topology-inspection API in the cited overview. | Event handler outcomes distinguish handled, transition, and superstate; runtime is oriented around event handling. | A Rust alternative for code-defined event-driven machines, not a direct substitute for Statekit's runtime-configured structural contract. It also clarifies what should remain outside Statekit (events, hierarchy, actions, context). |
| [**smlang (Rust)**](https://docs.rs/smlang/latest/smlang/) | Procedural macro DSL declares transitions, events, optional starting state, guards, and actions; generates enums and APIs. | Macro parses a static source definition and generates code. It does not target arbitrary runtime-supplied topology in the same way. | Generated state machine API exposes current state and processes events. | Generated errors include invalid event / guard failure; compile-time errors are natural for malformed DSL source. | Compile-time authoring and runtime configuration solve different problems. Statekit's audience is the application whose graph only exists after loading data. |
| [**GitLab CI Lint / pipeline editor**](https://docs.gitlab.com/ci/yaml/lint/) | YAML defines jobs, stages, `needs`, and rules; a simulated pipeline depends on the triggering context. | Lint validates syntax/configuration; pipeline simulation can reveal dependency/rules problems. | Includes jobs that would exist under a simulation and a visual pipeline editor. | Reports configuration and simulation errors; API can return generated jobs. | Adjacent evidence that users need topology checks after includes/conditionals are resolved. CI jobs form a DAG/dependency model, not a state-transition machine; Statekit should not add DAG scheduling semantics. |

## Gap analysis

### Definition versus validation

Statekit already handles **edge-local validity** and **machine-level minimum validity**. `try_allow` is suitable for a consumer that has already parsed edge pairs. Unlike richer config systems, Statekit does not receive a standalone `states` collection, so it cannot represent or flag a declared state that has no incident transition. This is consistent with the current model, where states are inferred from transition endpoints.

The `NoTransitions` check intentionally means a successfully built machine has at least one edge. It does not establish a start state, final state, connected component, or reachable-state set. Adding those as construction failures would strengthen validity rules and risk rejecting configurations currently allowed by the specification. Keep them as optional analyses or consumer checks if pursued.

### Diagnostics

Statekit's typed errors are useful for Rust callers and fit Project Core INV-008. However, a configuration author typically needs to map an error back to a file, JSON/YAML key, database row, or plugin entry. Statekit sees only the strings passed to the builder, so source locations cannot be inferred by the current API.

**Proposal boundary:** An adapter can preserve original entry indices/locations while calling `try_allow`, or a future importer-facing API can accept caller-supplied provenance. A universal diagnostic system with severities, codes, locations, multiple accumulated errors, warning policy, localization, or format paths would be a significant API responsibility and is not justified by this initial scope. It also raises a design tension with INV-007's earliest-error contract. Treat delayed/accumulated diagnostics as future research, as requested, rather than silently replacing fail-fast construction.

### Analysis and introspection

Today, consumers can enumerate nodes/edges, inspect outgoing neighbors, count edges, test exact edge existence, and ask whether a state is terminal. This is enough for basic UI listing, edge-by-edge config assertions, and simple traversal.

The strongest missing low-level queries suggested by consumer examples are:

1. **Incoming edges / predecessors.** Useful for displaying dependencies and inspecting why a state can be entered. It is a direct dual to existing `targets_from` and can be implemented without domain semantics.
2. **Reachable states from a caller-supplied root.** Useful to find disconnected portions relative to a consumer's selected start. A caller-supplied root preserves Statekit's lack of canonical initial-state policy. The result should be an observation; it should not cause `build()` to fail.
3. **Path existence or one path.** A possible convenience after reachability exists, but not required for the first discovery increment. Do not expand into all-path enumeration or shortest-path policy without concrete demand.

The consumer should distinguish “unknown” from “known but terminal”; today `is_terminal(unknown)` returns true by the no-outgoing-edge definition. The existing `contains_state` lets callers do so, and the docs should make this pattern explicit before adding a new result type.

### Identity and scale

The current API is string-based and the roadmap already identifies state identity/representation as an unresolved v0.5 topic. This research does not settle it. VPE and other compiled consumers may already own resolved IDs, but config-oriented consumers may prefer symbolic names. A public generic-ID or ID-lowering layer could deepen the integration story but has nontrivial compatibility, ergonomic, and memory/lookup tradeoffs. Preserve as a separate investigation rather than coupling it to validation/analysis proposals.

## Prioritized shortlist of proposals

These are ranked research candidates, not accepted roadmap items.

| Priority | Category | Proposal | Rationale | Tradeoffs / decision evidence needed |
|---|---|---|---|---|
| 1 | Documentation | Add a database/config-loaded example that builds from an iterator of edge records, shows the typed error boundary, reuses one `Machine` for many entity instances, and explicitly puts parsing, start-state policy, authorization, guards, and effects in the host. | Maps directly to Statekit's stated audience and demonstrates what a consumer can do today without adding API. | Keep examples short and avoid implying file parsing or workflow orchestration are built in. Validate example against release API. |
| 2 | Documentation | Add a “structural validation vs application policy” section with concrete checks: Statekit guarantees valid edge topology, while the host may decide root reachability, terminal-state conventions, role permissions, or completion semantics. | Prevents consumers from treating absence of a Statekit invariant as a business guarantee; reinforced by Python StateMachine's configurable/disabled graph checks. | Additional wording must remain simple and not promise future analysis methods. |
| 3 | Small API improvement | Evaluate `predecessors_of` / `sources_to` query on immutable `Machine`. | Direct counterpart to `targets_from`; useful for configuration UIs and reverse inspection; scope stays generic. | Adds public surface and perhaps indexing cost. Measure against existing representation and memory benchmarks; leave iteration order unspecified. |
| 4 | Small API improvement | Evaluate caller-rooted reachability or reachable-state iteration, with unknown-root behavior explicit. | Tests whether a configured root can structurally reach selected nodes without defining a canonical initial state or rejecting the machine. | O(V+E) traversal and possible allocation/API complexity; connectedness can be intentional, so return facts only. Confirm repeated consumer need before committing API. |
| 5 | Documentation / boundary | Document `contains_state` before `is_terminal` when unknown input must be distinguished from a known leaf. | Prevents a reasonable but subtle misuse of existing API. | Could be docs-only unless user experience shows the combined call is awkward. |
| 6 | Future research | Consider a source-provenance adapter/diagnostic pattern for format imports. | AWS-style tools show that code + message + location help pre-deploy pipelines. | Format locations belong to parsers; a universal collector adds policy, allocation, complexity, and may conflict with fail-fast invariant. Need actual consumer and explicit error-timing design. |

### Defer for now

- Full path enumeration, shortest paths, SCC/component reports, dominators, or a graph-library abstraction: useful in larger analysis/testing systems, but not established as necessary for Statekit's initial consumer contract.
- Declared isolated states, single-start/single-final policies, “all states reachable,” “no traps,” or “must terminate”: each assumes an application policy absent from the current flat transition model.
- Format-specific JSON/YAML/SCXML parsing, schema validation, and source maps: better owned by adapters/host applications unless repeated, independent demand emerges.
- Multi-error diagnostic accumulation: worthwhile research as a separate topic; it is not a direct extension of today's fail-fast builder without design tradeoffs.
- Event names, guards, callbacks, actions/effects, payloads, priorities, retries, async execution, hierarchy, parallel states, workflow scheduling, or graph rendering as a built-in: these introduce semantics/framework responsibilities currently outside scope.

## A. Already supported

Consumers can already use Statekit to:

- Build an immutable topology from dynamic state-name edge pairs.
- Reject malformed endpoint names, self-transitions, empty definitions, unknown initial states, and attempted non-edges.
- Reuse one immutable machine definition across independently moving instances.
- Query exact edge membership and inspect states, transitions, source nodes, and outgoing targets.
- Identify a terminal state in the structural sense “no outgoing edges,” and distinguish unknown state via `contains_state`.
- Project richer consumer-defined behavior onto a plain structural edge set, without asking Statekit to understand it.

## B. Documentation opportunities

The README already names configuration, database, user-defined, and plugin use. It could more concretely show the production integration boundary:

1. Parse an application-owned config to edge records.
2. Add each edge through `try_allow`; map its typed error to the config record held by the caller.
3. Build one immutable machine.
4. Apply consumer-specific policy checks separately.
5. Create one instance per entity from the host-selected current state.
6. On an application event, ask/perform the structural move and run host-owned effects in the host's transaction model.

It should also make clear that Statekit does not currently parse a config file, validate callback names, require a root/final state, detect disconnected components, or guarantee a workflow eventually completes.

## C. Small API improvements

Only predecessor inspection and caller-rooted reachability currently appear to deepen the existing responsibility with limited domain risk. They should be evaluated with actual consumer inputs, performance/memory measurements, and API alternatives before implementation. They are not requirements inferred solely from competitors.

## D. Future research

1. **Consumer evidence:** Search for real crates/apps that depend on runtime-defined edge tables; interview or inspect one concrete integration before broadening the API.
2. **Reference-model applications:** Examine how users can treat a topology as a reference model for coverage/behavior checks, without placing test frameworks in Statekit.
3. **Structural intermediate representation:** Determine whether consumers need stable machine IDs, typed IDs, or generic state identity (including already-resolved VPE identifiers) and how an IR would preserve abstraction hiding.
4. **Serialization/portable definitions:** Study whether Statekit should define a canonical portable representation or simply consume consumer-provided edge iterators. Versioning, compatibility, and identifiers must be considered separately.
5. **Delayed validation and diagnostics:** Compare fail-fast construction with an optional diagnostic collector, preserving typed errors and INV-007 semantics. Determine who owns source locations and warnings.
6. **Analysis algorithms:** Establish a smallest useful set from concrete queries. Explore incoming edges and rooted reachability first; examine cycles/SCCs only if a consumer case needs them.
7. **VPE integration:** Test a structural projection from a compiled process/law and measure whether Statekit saves duplicated topology checks while VPE retains guards, effects, data-flow, simulation, and migration.
8. **Performance under real shape distributions:** Use small and larger machine sizes and repeated instance queries; avoid adopting graph/index structures absent measured benefit.

## E. Out of scope

The following should remain owned by VPE or the host unless the project deliberately revises its boundaries: file formats and schema languages; statechart hierarchy/parallel regions; event dispatch; guard evaluation; actions/effects and callbacks; policy and authorization; workflow orchestration/scheduling; business completion or deadlock judgment; persistence, concurrency coordination, retries, audit history, migration, simulation, and test-case execution. Statekit may be a component used by such systems, but should not absorb those responsibilities.

## Open questions for future sessions

- Do real consumers need to declare states independently of edges, or is endpoint-derived topology sufficient?
- Which query is most valuable after current enumeration: incoming edges, rooted reachability, reachable-state set, or path existence?
- Should unknown-root analysis return an error, empty result, or an explicit outcome? Which choice best fits existing `contains_state`/`targets_from` semantics?
- Does source provenance belong in Statekit at all, or should parsers retain record locations and map Statekit's typed error themselves?
- Can optional accumulated diagnostics coexist with “earliest valid error detection” without undermining the current simple builder contract?
- Should Statekit stay string-identified, adopt a generic state identity, or introduce a Statekit-owned state ID after external evidence and benchmarks?
- Would a VPE structural projection use strings, resolved IDs, or a separate adapter, and what duplication would it eliminate?
- What benchmarks establish that incoming/reachability indexes are worth their memory and construction cost?
- Are downloaded definitions expected to preserve unknown/extension data, and if so, is a canonical serialization format appropriate for Statekit or an adapter?
- Which structural conditions are facts to report versus hard invariants? Can each candidate rule be justified without domain assumptions?

## Primary sources

- Statekit `main`: [README](https://github.com/whitephoenixsoft/statekit/blob/main/README.md), [`Cargo.toml`](https://github.com/whitephoenixsoft/statekit/blob/main/Cargo.toml), [`builder.rs`](https://github.com/whitephoenixsoft/statekit/blob/main/src/builder.rs), [`machine.rs`](https://github.com/whitephoenixsoft/statekit/blob/main/src/machine.rs), [`instance.rs`](https://github.com/whitephoenixsoft/statekit/blob/main/src/instance.rs), [`error.rs`](https://github.com/whitephoenixsoft/statekit/blob/main/src/error.rs).
- pytransitions: [project README and API overview](https://github.com/pytransitions/transitions).
- python-statemachine: [validations](https://python-statemachine.readthedocs.io/en/latest/validations.html), [document IO and validation caveats](https://python-statemachine.readthedocs.io/en/latest/io/index.html), [comparison with pytransitions](https://python-statemachine.readthedocs.io/en/v3.0.0/how-to/coming_from_transitions.html).
- AWS Step Functions: [Amazon States Language](https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html), [ValidateStateMachineDefinition CLI/API behavior](https://docs.aws.amazon.com/cli/latest/reference/stepfunctions/validate-state-machine-definition.html), [diagnostic fields and codes](https://docs.aws.amazon.com/step-functions/latest/apireference/API_ValidateStateMachineDefinitionDiagnostic.html).
- XState: [graph and path utilities](https://stately.ai/docs/graph), [machine configuration](https://dev.stately.ai/docs/xstate/v6/configuration).
- Rust comparators: [statig](https://docs.rs/statig/latest/statig/), [smlang](https://docs.rs/smlang/latest/smlang/), [runtime `fsm` crate](https://docs.rs/fsm/latest/fsm/).
- Adjacent configured graph consumer: [GitLab CI lint](https://docs.gitlab.com/ci/yaml/lint/), [pipeline editor simulation](https://docs.gitlab.com/ci/pipeline_editor/).

---

**Conclusion:** The research supports developing Statekit as a narrow, reusable structural validator and topology query layer for runtime-defined machines. Current functionality is already useful as a post-parse validation dependency. Better documentation is the safest next step; predecessor inspection and rooted reachability are bounded candidates for later evidence-driven API work. Richer workflow semantics, configuration formats, policy checks, and accumulated diagnostics remain unresolved or outside scope.
