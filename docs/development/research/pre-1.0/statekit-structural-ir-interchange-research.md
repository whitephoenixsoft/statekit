# Statekit — Structural Intermediate Representation & Data Interchange Research

**Status:** Research proposal; no architectural decision is made here  
**Investigation:** Third report in Statekit Consumer Research & Capability Discovery  
**Repositories reviewed:** `whitephoenixsoft/statekit` and `whitephoenixsoft/verified-process-engine`, default branch `main`  
**Research date:** 2026-10-12 (America/New_York)

## Executive summary

**Finding:** Statekit's immutable `Machine` is already useful as a validated *structural projection* for flat state-machine topology. It can sit between a consumer's parser/resolver and components that need to inspect or traverse unlabelled directed edges. Its public iterators expose enough logical information to rebuild a consumer-owned edge definition, while its equality compares logical topology independently of allocation. Its private storage stays hidden.

That does not make `Machine` a general compiler IR or a portable interchange format. Statekit currently represents only states inferred from transition endpoints and transitions identified by `(source, target)`. It does not represent isolated states, labels/events, parallel semantic transitions, initial-state metadata, or provenance. It rejects self-transitions and requires at least one edge. These are deliberate current invariants, so some richer definitions cannot be projected without loss or failure.

The evidence supports **Option A, consumer-owned serialization**, as the smallest integration today: parse a consumer's versioned JSON/YAML/etc. document into the consumer's own data type, project valid edge pairs into `MachineBuilder`, and keep the original richer representation. For export, collect `Machine::transitions()` into the consumer's own wire type. Receiving code reconstructs through `try_allow(...).build()`, preserving current validation. `Machine::transitions()` and `Machine` equality already support the core operation; no serialization API is required.

Serde is a common Rust interchange layer, but adding `Serialize`/`Deserialize` implementations for `Machine` would make Statekit own more of the data contract. A custom deserializer could rebuild through public validation, but persisted format shape, schema versioning, ordering, compatibility, and malformed-input behavior would still need definition. Directly deserializing internal fields would risk bypassing invariants and contradict BI-002. No evidence yet shows enough cross-consumer demand for Statekit to own even an optional portable-definition type.

VPE's current `CompiledProcess` already has an indexed node/edge representation and richer semantic transitions. Projecting it into Statekit would discard action, priority, guard, and effect identity; it may also fail for a zero-edge or self-edge topology. VPE already validates targets, computes initial-root reachability, and checks `AUTO_TICK` structure. Current evidence therefore favors Statekit as an optional validation/test dependency, or no integration, over retaining it as VPE's authoritative compiled IR.

**Answer:** Yes, Statekit can serve today as a small validated structural IR *for consumers whose needed model is exactly its flat edge topology*. For data interchange, consumers should own the portable schema and reconstruct `Machine` through its builder. The smallest justified Statekit change set is currently **none**. Documentation can make the projection/export pattern and its loss boundaries explicit. State IDs and a separate definition type remain roadmap questions requiring real integration measurements.

## Evidence labels and continuity

- **Verified** means present in current repository source/docs or a linked primary external source.
- **Demonstrated** means an existing consumer or library docs show the use case.
- **Interpretation** is the architectural conclusion drawn from those facts.
- **Hypothesis** is an opportunity that needs a consumer, scale, or benchmark to validate it.

The first report found a bounded role in validating runtime-configured edge sets and identified predecessor and caller-rooted reachability as possible analysis additions. The second found that existing `Machine`/`MachineInstance` APIs already support structural reference-model tests; test generation and coverage belong in external test frameworks. This report carries those forward without reopening those questions as its main subject. The prior reports are available as [`statekit-consumer-research.md`](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md) and [`statekit-reference-model-research.md`](sandbox:/workspace/scratch/3524c4ebe062/statekit-reference-model-research.md).

Both prior reports also identified questions that matter here: string versus resolved identity, self-transition and empty-machine compatibility, loss of parallel semantic edges, and the difference between structural facts and consumer policy. The supplied README, Project Core, specification, and roadmap were reviewed alongside current `main` source and relevant tests.

## 1. Current Statekit representation and interchange surface

### Verified public model

- `MachineBuilder::try_allow(from, to)` validates endpoint names and rejects self-transitions immediately. Duplicate endpoint pairs have one logical identity. `build()` fails if no transition was added.
- A built `Machine` is immutable and `PartialEq + Eq`; equality is based on logical definition, not shared-allocation identity.
- States are inferred from edge endpoints. There is no API to declare an isolated state independently of a transition.
- `Machine::states()`, `sources()`, `transitions()`, `targets_from()`, `contains_state()`, `can_transition()`, `validate_transition()`, counts, and `is_terminal()` expose structural inspection without exposing internal storage. `Transition` exposes source and target names.
- `Machine::instance(initial)` constructs a stateful traversal position. This report focuses on the definition/interchange boundary; the previous report covers its runtime role.
- Enumeration order is explicitly unspecified. The backing collection in current source is a `HashSet`, but this is implementation evidence only; BI-002 says storage is not part of the domain contract.
- The current manifest has `thiserror` as its only runtime dependency. `criterion` and `proptest` are development dependencies; there is no Serde dependency or serialization feature.
- The roadmap already contains a “Multiple Representations” investigation: retain strings, make the machine generic, or lower symbolic names to a Statekit-owned ID are open alternatives. The roadmap says generic `Machine<S>` should be compared, not assumed.

### What this means for a structural IR

**Interpretation:** A useful IR does not have to expose its storage layout. It does need a stable logical vocabulary and operations that later stages can depend on. Statekit's public vocabulary—state names and directed transitions—does provide that for a narrow class of consumers. A compiler stage can accept an opaque `Machine` and query it, while Statekit remains free to change private storage.

**Limit:** The public logical model is intentionally smaller than a compiler's resolved or executable model. Rebuilding through `transitions()` preserves Statekit topology but cannot recover information that projection discarded. A consumer with rich edges should retain its own source or compiled definition and treat Statekit as a projection, not as the sole authority.

An IR typically earns its cost by allowing multiple passes or tools to share a common representation. Rust's compiler uses several phase-specific representations (HIR, THIR, MIR), rather than one universal object; LLVM documents an IR designed for in-memory transformations, on-disk bitcode, and readable assembly. These are much broader systems than Statekit. The transferable lesson is modest: a representation's contract should be shaped around the transformations and analyses that actually consume it. Statekit supplies a small topology contract, not a general pass manager or semantic lowering model.

## 2. Data interchange options

| Option | What it means | Consumer value | Cost and contract | Assessment from current evidence |
|---|---|---|---|---|
| **A. Consumer-owned serialization** | Each consumer serializes its authoritative config/process type. It extracts endpoint pairs and constructs Statekit through `MachineBuilder`. | High where consumers already have JSON/YAML/schema types. Preserves extensions, source locations, version fields, labels, and domain metadata in the owning type. | No Statekit dependency or wire-format contract. Projection and reconstruction are O(E) plus the current builder's costs. Consumer owns compatibility and diagnostics. | **Best-supported now.** Existing `transitions()` is enough to export topology; builder is enough to revalidate it. |
| **B. Portable logical topology type** | Statekit provides a public data-only definition (for example, transition pairs) that can be serialized by caller code or an adapter. | Could reduce boilerplate if multiple consumers repeatedly need exactly the same topology shape. Separates portable definition from private compiled `Machine`. | Public shape becomes a compatibility surface. Must decide whether it contains an explicit state list, schema version, order, duplicates, and unknown fields. Still needs caller-selected format or Serde. | **No demonstrated need yet.** A consumer struct is a short adapter today. Revisit only with repeated cross-consumer duplication. |
| **C. Optional Serde support** | A Cargo feature implements Serde traits for a portable definition or directly for `Machine`. | Convenient Rust-to-Rust JSON, YAML, CBOR, and other Serde ecosystem integration. Feature can keep Serde out of default builds. | Optional compilation does not remove long-term schema/compatibility obligations or the work of testing round trips. Direct impls on `Machine` couple its public semantics to an interchange contract. | **Plausible later; not justified by one consumer.** Prefer a separate logical definition if evidence warrants it. |
| **D. Serialize built `Machine` directly** | `Machine` itself implements data serialization and deserialization. | Convenient for callers who want save/restore of a topology. | Deriving traits on internal representation would expose storage and could deserialize invalid states. A manual impl can serialize logical edges and deserialize by rebuilding, but the machine type then carries format/version expectations. | **Avoid as the first design.** If ever considered, deserialization must call the same validated construction path; do not restore internal fields directly. |

### Validation-preserving round trip

The safe conceptual route is:

`consumer definition -> validated Machine -> consumer portable topology -> deserialize consumer type -> validated Machine`

On receive, reconstruct through `try_allow(...).build()` so invalid names, self-transitions, duplicates, and empty definitions follow the same current rules as normal construction. A deserialized wire object is not itself a validated `Machine` merely because the sender produced it.

The logical property worth testing is:

`rebuild(export(machine)) == machine`

That asserts **logical topology equality**. It does not assert stable transition iteration order, canonical serialization, byte-for-byte deterministic JSON, or backwards compatibility between schema versions. Those are separate promises.

```rust
use serde::{Deserialize, Serialize};
use statekit::{Machine, StateError};

#[derive(Debug, Serialize, Deserialize)]
struct TopologyV1 {
    schema_version: u16,
    transitions: Vec<EdgeDef>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EdgeDef {
    from: String,
    to: String,
}

fn build_machine(doc: &TopologyV1) -> Result<Machine, StateError> {
    let mut builder = Machine::builder();
    for edge in &doc.transitions {
        builder = builder.try_allow(&edge.from, &edge.to)?;
    }
    builder.build()
}

fn topology_from_machine(machine: &Machine) -> TopologyV1 {
    TopologyV1 {
        schema_version: 1,
        transitions: machine.transitions().map(|t| EdgeDef {
            from: t.source().to_owned(),
            to: t.target().to_owned(),
        }).collect(),
    }
}
```

This is illustrative consumer code, not a Statekit API. The consumer supplies Serde and a format crate such as `serde_json`. It must handle format parse errors separately from `StateError`; it may map builder failures back to document locations if it retains edge indices or paths. Since iteration order is unspecified, the output vector order is not a canonical byte format. Sort a consumer-owned copy if its own format requires deterministic output; do not infer that requirement from Statekit equality.

If a consumer needs isolated state declarations, the wire type needs a separate `states: Vec<String>` field. The current `Machine` still cannot retain isolated states, so its structural projection would omit them. Treat that as a representational limitation, not a parse error to hide.

### Serde and format compatibility

Serde deliberately separates data structures from format implementations and supports many human-readable and binary formats. That reduces the argument for Statekit to own JSON/YAML specifically. It does not itself provide a versioned Statekit schema or guarantee that a Rust type's serialized representation is a durable interchange contract.

If Statekit directly exposed serialization later, the questions would include: Is the type a portable definition or compiled machine? Is format version explicit? Are duplicate edges normalized or rejected? Is iteration order canonical? Are unknown fields accepted, ignored, or preserved? How do errors identify a malformed edge? Does deserialize return a validated machine or an untrusted definition requiring a second call? These are public API and compatibility decisions, not consequences automatically solved by `#[derive(Serialize, Deserialize)]`.

## 3. State identity alternatives

| Identity model | Strengths | Costs / risks | Fit indicated by current evidence |
|---|---|---|---|
| **String names (current)** | Natural for JSON/config/editor authors; readable diagnostics; runtime-defined names; exact, case-sensitive equality. | Hashing/string comparison and possible repeated ownership/copying; compiled consumers may already have IDs. | Strong default for Statekit's documented user and simplicity goals. No verified consumer has shown this to be a blocking barrier. |
| **Numeric or opaque IDs** | Fast compact keys, convenient for resolved compiler tables, can match existing node handles. | Need a mapping to human names for validation and diagnostics; IDs may be process-local or unstable across recompilation; portable formats need a separate stable name/key. | Potential VPE integration option, but not enough by itself to define a portable identity. |
| **Generic `Machine<S>`** | Lets consumer reuse its resolved ID type without translation; flexible for domains that already have typed keys. | Expands public API and trait bounds, builder/query signatures, equality/hash requirements, ownership constraints, docs, and migration paths. String-specific name validation no longer makes sense for arbitrary `S`; users may face more type complexity. | Roadmap candidate to compare, not assume. Requires benchmark and API design evidence. |
| **Interned/resolved Statekit IDs behind names** | Could keep a simple input surface while using compact internal lookups. | Already an internal representation strategy under BI-002; adds mappings and construction memory. Exposing IDs would make them contractual; keeping them private means consumers still convert. | Consider only with benchmark evidence and a concrete query/runtime requirement. |
| **Consumer-owned identity map** | Consumer keeps its canonical names/IDs; Statekit receives names for a projection; only conversion glue is added. | Mapping cost and possible conversion bugs; may duplicate a table. | Lowest API commitment; likely sufficient for test-only or compile-time structural checking. Measure before replacing it. |
| **Separate public definition + private compiled machine** | Wire/editor data can preserve explicit states and metadata while `Machine` remains optimized and immutable. | Two types and conversions; validation boundary must be obvious; risks duplicating what consumer schemas already provide. | A reasonable future architecture if multiple consumers need a shared topology type, but not required to preserve storage opacity. |

### VPE state identity example

Current VPE compilation builds a `Vec<Node>`, `state_index: HashMap<String, usize>`, and `initial_state_idx`. Each `Edge` points to `target_idx: usize`; the node's position is its compiled source identity. Names remain on nodes for reporting and lookup. Statekit currently accepts `&str` endpoints, so a projection from VPE would use node/target names or construct an explicit name-to-index adapter; it would not directly reuse the edge indices.

That conversion is **not yet evidence of a Statekit defect**. It may be a small compile-time cost and simpler than adding a generic identity surface. If a benchmark or real integration shows that translating is costly, the roadmap's `String` vs generic ID vs Statekit ID comparison is the right place to evaluate it. A numeric type also cannot replace VPE's stable external process/state identity unless its stability rules are separately defined.

### Migration implications

Changing `Machine` from strings to generic identifiers, or adding a parallel generic machine, would affect builders, query methods, transitions, errors, equality, rustdoc examples, and the current name-validation invariants. Existing users may rely on `&str` ergonomics and actionable string errors. Since Statekit is pre-1.0, the project documents that its API can evolve between minor releases, but source compatibility still has consumer cost. A wrapper or adapter lets consumers test ID integration without forcing every user onto a more general API.

## 4. External interoperability patterns

| System | Definition and validation | Immutability / identity / interchange | Structural inspection | Applicable lesson and boundary |
|---|---|---|---|---|
| **Statekit (Rust)** | Runtime builder accepts string endpoint pairs and validates on addition/build. | Immutable `Machine`; string identity; no first-party serialization. | Public topology iterators and equality. | Small validated projection; current logical edge contract is narrower than statecharts or compiler IR. |
| **XState v6 (TypeScript)** | JSON-compatible machine config includes nested states, event transitions, actions, guards, actors, context, and metadata; creation validates config. | Docs provide `serializeMachine` and `createMachineFromConfig`; runtime functions may be represented as code expressions or require supplied runtime sources, and some features have no JSON representation. Machine behavior and portable data are related but distinct. | Machine/graph tools include path-generation APIs; serialization docs describe what is preserved and limitations. | Valuable example of explicit separation between data definition and executable sources. Its serialized shape and semantics are far richer; do not copy the scope. Verify v6 maturity/API status before depending on it. |
| **python-statemachine** | Can load native JSON/YAML and SCXML; native format has a published JSON Schema. Document loading can validate schema before constructing a machine. | A declarative document is a separate data form compiled into the runtime machine. JSON/YAML express the same library vocabulary; SCXML is an adapter to a richer standard. | Definition-time graph checks and rendering; loaded-document structural checks are configurable/disabled in some cases because policy assumptions can reject legitimate data. | Strong evidence that a portable definition is often consumer/library-specific and richer than a bare graph. The schema and conversion layer carry format responsibilities. |
| **pytransitions (Python)** | Runtime objects can be built from state/transition collections; extensions support hierarchical machines. | Machine instances can be pickled, but project docs advise sharing configurations rather than instantiated complex hierarchical machine state in some cases; JSON/YAML configuration is a separate route. | GraphMachine can visualize a richer runtime graph. | Pickling a runtime object is different from promising a portable logical definition. Prefer data formats that reconstruct through validation over opaque object snapshots. |
| **SCXML (W3C)** | XML language for event-based statecharts with initial configurations, events, hierarchy, parallel states, executable content, and processor semantics. | Standard interchange language, but semantically much richer than Statekit's flat transitions. A projection can be lossy. | Explicit legal state configurations and statechart execution rules. | An established interoperability option for users who need statechart semantics; Statekit should not claim SCXML compatibility without a well-defined subset/adapter and round-trip policy. |
| **rustc / LLVM IR** | Compiler IRs are designed around compiler phase needs; rustc uses several distinct representations. LLVM IR supports transformations/analysis and has in-memory, bitcode, and readable forms. | Identifiers/types and serialization forms are deliberate contracts; LLVM's equivalence between forms is an explicit design decision. | Rich visitors, passes, analyses, and transformations. | Don't use “IR” to imply a pass framework or serialization guarantee. Statekit can be one narrow phase representation without adopting compiler infrastructure. |
| **petgraph (Rust)** | Generic graph data structures, node/edge weights, graph indices, and algorithms. | Representation and index behavior depend on graph type; `StableGraph` keeps indices stable across removals and adds storage/graph concerns. | Broad graph traversal and algorithms. | Useful if a consumer needs broad graph operations. Statekit's narrower domain API should not become a graph-storage substitute. |

These are architectural comparisons, not adoption recommendations. The Python and XState libraries show that machine-definition interchange is commonly owned by a richer domain vocabulary, while compiler IRs show that one system may need multiple representations with explicit phase contracts.

## 5. VPE integration comparison

### Current VPE structure verified

- `CompiledProcess` stores `Vec<Node>`, a string-to-index `HashMap`, initial-state index, digest, process reference, and manifests. `Node` stores its name and `Vec<Edge>`.
- `Edge` stores `action`, `target_idx`, `priority`, runtime guard objects, and effects. Its source is implicit in the containing node.
- The compiler resolves names into indices, builds nodes and semantic edges, and sorts transitions by descending priority.
- Validation checks schema/law agreement, non-empty and unique state names, initial-state existence, target existence, guards/effects, transient-state requirements, `AUTO_TICK` structure/cycles, and warns on terminal states and states unreachable from the selected initial state.
- VPE currently checks that state and target names are non-empty, while Statekit additionally rejects whitespace-only names and names with leading or trailing Unicode whitespace. A projection can therefore reject some names VPE's source validator accepts.
- There is no `statekit` dependency in `crates/vpe/Cargo.toml`.
- Existing fixtures include multiple transitions with the same source, action, and target but differing priority/guard/effect details. VPE transition identity therefore cannot be reduced to Statekit's endpoint pair.
- The validator does not currently reject a transition whose source and target are the same name. Statekit rejects that pair. VPE also permits a valid law with no transitions if its domain rules otherwise allow it; Statekit's builder rejects an empty edge set.

### Architecture comparison

| Architecture | Benefits | Costs and risks | Evidence-weighted view |
|---|---|---|---|
| **A. Structural validation dependency** | Can reuse Statekit construction checks and some direct edge queries during VPE compilation. | VPE already checks endpoint resolution and has semantic validation; would repeat construction/indexing and name conversion. Statekit can reject VPE definitions (self-edge, empty edge set) that VPE currently permits. Endpoint deduplication would not validate VPE semantic edge uniqueness. | Could be tested at the compile boundary, but no code elimination or runtime win is currently demonstrated. Use only if a specific duplicate check is removed or independently valuable. |
| **B. Structural IR dependency** | Could give later VPE phases a shared immutable topology view and possible future analysis API reuse. | Retains a second graph alongside `Vec<Node>/Vec<Edge>` or replaces the existing source-of-truth. Parallel semantic edges, actions and edge indices still live in VPE; state-name/index conversions and memory duplication remain. Current `Machine` cannot represent every VPE topology. | Weakest fit today as authoritative compiled IR. A Statekit topology projection is not a replacement for the semantic compiled process. |
| **C. Test-only structural reference** | Avoids production dependency/runtime cost; checks a projection against simple transition semantics and can act as independent oracle for selected tests. | Keeps VPE's implementation in place; test projection may share source bugs. Empty/self-edge cases require explicit skip/failure expectations. Same endpoint edges collapse. | Still the lowest-coupling experiment from investigation 2, if a targeted test needs it. It does not establish a need for interchange. |
| **D. No integration** | Keeps one authoritative compiled representation and avoids adaptation, version coupling, and a second representation. | VPE retains its own structural validation and testing for generic edge behavior. | A fully reasonable choice. Existing compiler has real additional semantics and its own indexed topology; no repeated consumer need is yet measured. |

**Assessment:** A or C may be useful for a bounded validation/test. B is not justified as the default architecture. D may be simpler if Statekit's checks do not remove duplicated code or help test a real discrepancy. Do not choose based on the general appeal of “one IR”; require an integration to name which phase consumes it, what code disappears, and how the additional topology memory/lookup affects compilation and runtime.

## 6. Concrete consumer examples

### Example 1 — Configuration to validated machine

**System:** A service loads a versioned tenant lifecycle JSON document and needs to verify permitted adjacent state changes.

**Authoritative data:** The service's `TopologyV1`/`EdgeDef` structs and source JSON, including schema version and any comments or metadata.

**Statekit projection:** Call `try_allow` per edge and `build`; retain both the parsed data and `Machine`. Parser/schema failures and `StateError`s remain distinguishable. A Statekit error can be associated with the current edge index because the consumer controls the loop.

The code above demonstrates this with current APIs. Statekit eliminates hand-written loops for duplicate-edge handling, state-name validation, self-transition rejection, and membership queries. It does not parse JSON, validate unknown JSON fields, preserve source locations, declare isolated states, decide initial state, or define tenant-specific policy. Construction is O(E) in edge count at the API level, plus the library's internal representation; no serialization work occurs on transition checks.

**Failure modes:** If the adapter maps the wrong field into `from` or `to`, Statekit validates the wrong topology perfectly. Keep schema tests and expected fixtures independent. If a document intentionally permits self-loops or isolated states, current `Machine` is not a lossless model for that definition.

### Example 2 — Structural interchange between two components

**System:** A configuration editor exports topology and a Rust service imports it. The editor and service may release independently.

**Portable boundary:** Define an application-owned, versioned JSON object, for example `{ "schema_version": 1, "transitions": [{"from":"draft","to":"review"}] }`. The editor writes that schema; the service deserializes to its own struct and rebuilds Statekit. If the editor sends labels, node positions, source provenance, or isolated states, keep those fields in the portable schema even if Statekit ignores or cannot represent them.

**Comparison:** The receiver may build a second `Machine` from its local input and compare it with another reconstructed `Machine`; `Eq` says the logical endpoint set is equal. It does not certify that the two documents have equal labels/metadata or that the application semantics are equivalent. To compare a foreign graph implementation, normalize its edges into a `HashSet<(String,String)>` and compare that to Statekit's transitions.

**Compatibility:** Use the wire schema's own explicit version and migration policy. Do not use Rust crate version, `Machine`'s private representation, HashSet iteration order, or serialized bytes as the interchange identity. A receiver should reject unsupported versions before building; then use Statekit's builder so invalid edge data cannot bypass machine invariants. Unknown-field preservation/acceptance is a schema policy the consumer decides.

**Code eliminated:** Statekit provides a validated topology object and logical equality/queries. It does not eliminate the two components' format adapters, schema versioning, transport, or semantic compatibility tests. If all components already share the same config type and validation, adding a separate Statekit portable type may be redundant.

### Example 3 — VPE compiler integration

**System:** VPE compiles a process law into indexed state nodes and semantic edges.

**Candidate projection:** For each VPE node and edge, translate the target index back to its node name and add `(node.name, target.name)` to Statekit. This is illustrative and hypothetical integration code; it uses current Statekit methods but current VPE types.

```rust
use statekit::{Machine, StateError};

fn topology_projection(process: &CompiledProcess) -> Result<Machine, StateError> {
    let mut builder = Machine::builder();

    for node in process.nodes() {
        for edge in &node.transitions {
            let target = &process.nodes()[edge.target_idx];
            builder = builder.try_allow(&node.name, &target.name)?;
        }
    }

    builder.build()
}
```

The projection preserves the set of unique, non-self source/target pairs if at least one exists and all names satisfy Statekit's stricter name rules. It loses action, priority, guard, effect, parallel edge multiplicity, manifests, initial-state metadata, digest, transient flags, and compiled indices. It fails on any self-edge, leading/trailing-whitespace or whitespace-only endpoint, and on no edges. VPE must retain its own `CompiledProcess` for execution. Avoid keeping the `Machine` in `CompiledProcess` unless tests or a later analysis pass show that repeated construction/query benefit outweighs conversion and duplicate memory.

**Validation boundary:** VPE validates its full law first and owns domain rules; Statekit could then validate only the projected topology. It cannot decide which edge applies to a requested action or which guard/priority wins. VPE already computes root reachability and `AUTO_TICK` analysis; any Statekit reachability API would overlap the former but not replace the latter.

**Potential failure:** A “successful” projection can make multiple VPE semantic edges appear as one structural edge. Consumers must not use the Statekit projection as a serialization of VPE's compiled process or as an equality test for process behavior.

## 7. Capability and gap matrix

| Category | Capability | Current Statekit position | Value / next action | Cost and invariant implications |
|---|---|---|---|---|
| **A. Already supported** | Validated immutable flat topology | Builder validates endpoints, self-edge rule, duplicate logical edges, and non-empty machine. | Can serve as post-parse projection today. | Preserves INV-001..007 and BI-002. |
| **A. Already supported** | Structural export | `transitions()` exposes all logical endpoint pairs; `states()` exposes endpoint-derived state names. | Consumers can build their own DTO/wire schema. | No order guarantee; no isolated states. Does not expose internal storage. |
| **A. Already supported** | Logical equivalence | `Machine: Eq` compares machine definitions. | Round-trip property can assert reconstructed machine equals original. | Not serialization/order/byte/schema equality. Supports INV-013. |
| **B. Consumer-implemented** | JSON/YAML/CBOR encoding and decoding | No Statekit parser or Serde impl. | Consumer DTO with `Serialize/Deserialize`, then `try_allow`/`build`. | Keeps dependency and schema responsibilities at consumer boundary. Caller owns extension fields and source errors. |
| **B. Consumer-implemented** | Deterministic output | Transition order unspecified. | Sort an owned list before encoding if the consumer needs reproducible bytes. | Avoids changing enumeration contract or constraining BI-002. |
| **B. Consumer-implemented** | ID/name mapping | API takes string names. | Keep a consumer map at compilation or tests. | O(V) mapping may be sufficient; benchmark before API redesign. |
| **B. Consumer-implemented** | Round-trip/property testing | Builder and `Eq` exist. | Assert `build(export(machine)) == machine`; use independent fixtures. | Test policy remains consumer-owned. |
| **B. Consumer-implemented** | Rich-definition preservation | Statekit retains only edge endpoint topology. | Keep authoritative source/compiled representation alongside any projection. | Prevents silent semantic loss. |
| **C. Small reusable improvements** | Document export/projection pattern | Capability exists but no explicit portable-interchange example was verified. | Add a consumer example explaining logical export and loss boundaries if repeated questions arise. | Documentation only; no code/dependency/perf impact. |
| **C. Small reusable improvements** | Caller-rooted reachability / predecessor queries | Not currently public; previous reports identify them as proposals. | Could make structural analysis more reusable, including consumer cross-checks. Not necessary for interchange or IR. | Must remain observations; no new build rejection. Reachability O(V+E), predecessor scan O(E) unless indexed. |
| **D. Future research** | Separate public `MachineDefinition`/portable topology | No such type. `Machine` is built opaque representation. | Investigate only after multiple consumers repeat DTO/mapping code or need explicit state list. | New API and equality/version questions; must funnel through validation. |
| **D. Future research** | Optional Serde feature | No Serde runtime dependency today. | Benchmark build/size/dependency benefit only if Rust consumers request direct integration. | Optional feature avoids default dependency but not public data-contract support burden. Preserve default smallness and invariants. |
| **D. Future research** | Generic or ID-based identity | Roadmap expressly leaves multiple representations open. | Test with compiled consumers and benchmark conversion/lookup costs. | Affects name rules, method bounds, error reporting, equality, migrations, and APIs. |
| **D. Future research** | Isolated-state support | Current states come from edge endpoints. | Ask consumers whether they need empty/isolated/terminal-only states represented distinctly. | Adding them changes the logical model and may interact with INV-003 (non-empty transitions). |
| **E. Out of scope** | Canonical JSON/YAML/SCXML ecosystem or general schema loader | Not part of Statekit. | Consumers/adapters own format syntax and versioning; established standards remain separate integrations. | Would add parsing, format, diagnostics, evolution, and potentially semantic scope. |
| **E. Out of scope** | General compiler pass manager / graph database | Not part of Statekit. | A host compiler or graph library should own these if needed. | Conflicts with BI-001/BI-002 and small-library intent. |
| **E. Out of scope** | Workflow actions, events, guards, effects, migrations | Not part of Statekit's edge-only definition. | Keep in consumers such as VPE. | Would conflate structural and semantic edge identity. |

## 8. Prioritized recommendations

These are proposals, not accepted design decisions.

1. **Keep serialization consumer-owned for now.** The existing iterator and builder form a complete narrow bridge: export endpoint pairs, deserialize into the consumer's DTO, and reconstruct through the builder. This adds no Statekit dependency or wire contract and preserves validation.
2. **Describe `Machine` as a structural projection/analysis object, not a universal IR.** It can be shared across stages that need only endpoint topology. Consumers should retain their authoritative or executable model when it carries semantic or format information.
3. **If documenting interchange, show a round-trip equality example and state exactly what equality means.** Normalize/sort only in consumer-owned output when deterministic serialization is required. Distinguish logical topology equality from stable ordering and durable schema compatibility.
4. **Do not derive deserialization over internal machine storage.** If a future Statekit-owned import path is justified, it must reconstruct through `MachineBuilder` and preserve earliest error behavior, name rules, duplicate semantics, no-self-edge, non-empty-machine, and logical equality.
5. **Treat VPE as a measurement case, not a design mandate.** A test-only projection is a low-risk probe; production storage as a second IR should require evidence that it removes repeated code or improves a phase materially. Account for VPE's current zero-edge/self-edge compatibility and semantic edge multiplicity first.
6. **Defer a public portable type, Serde feature, and generic state IDs.** They might help several consumers, but no shared demand has been demonstrated. The roadmap's multiple-representation investigation and benchmarks should decide whether the string API remains enough.
7. **Keep reachable-state and predecessor work orthogonal.** Those are candidate structural analysis APIs from the first report; they do not imply Statekit should own interchange or compiler infrastructure.

### Project Core fit

- **I-001/I-005, INV-006:** Data-driven projection supports runtime configuration. Existing builder is the correct validation boundary.
- **I-002/I-003, BI-001:** Consumer-owned DTO avoids format dependencies and broad responsibilities. Avoid adding a portable schema without repeated need.
- **BI-002:** Public transition iterators are a sound logical export surface; direct storage serialization or public indices would expose more representation than needed.
- **INV-001/INV-009/INV-013:** Immutable `Machine` can be shared and compared logically. A wire round trip should build a new valid immutable definition rather than resurrect shared allocation/runtime internals.
- **INV-002/INV-003/INV-004/INV-005/INV-007:** Reconstructing through `try_allow` and `build` preserves current edge validity, duplicate logical identity, case-sensitive names, and fail-early validation. A serialization implementation that bypasses these checks would violate project intent.
- **INV-008:** Consumer parser errors and Statekit `StateError`s should remain matchable/distinct; adding a general diagnostics type is not required by this use case.
- **D-011:** If a future compiled ID or index makes repeated runtime operations faster, measure lookup and memory tradeoffs against conversion/build cost. No representation proposal is accepted by this report.

## 9. Architectural risks and limitations

- **Projection loss:** Mapping rich definitions to endpoint pairs is many-to-one. Equal Statekit machines can originate from processes with different actions, guards, effects, priority, labels, or metadata.
- **No isolated nodes:** Endpoint-derived state enumeration cannot round-trip a portable state list containing nodes with no incident edge.
- **Invariant mismatch:** Self-loops and edge-free definitions that are legal in another consumer cannot be represented by current Statekit. A consumer must define an explicit compatibility policy rather than silently drop the edge/state.
- **Equality overclaim:** `Machine::eq` validates logical topology equality only. It does not validate the source document, identify the same serialization bytes, or prove semantic equivalence.
- **Ordering and canonicalization:** Unspecified iterator order is compatible with set-like logical identity. Canonical output is a separate consumer contract requiring explicit sorting and encoding rules.
- **Schema drift:** A portable DTO requires versioning and migration whether defined in Statekit or a consumer. Adding Serde derives alone does not solve that lifecycle.
- **Validation bypass:** Direct deserialization of internal fields risks creating malformed `Machine`s or tying serialized data to implementation details. Rebuild through validation.
- **Common-mode error:** If producer and consumer share a faulty projection or wrong schema interpretation, Statekit will faithfully validate the mistaken topology. Keep format conformance and semantic tests separate.
- **Duplicated storage:** Retaining both VPE's `Vec<Node>/Vec<Edge>` and a Statekit `Machine` increases construction and memory costs. A topology projection could be built on demand for tests instead.
- **Identity stability:** Positional numeric indices can change across compilations. A serialized ID requires explicit stability/version rules; an internal index should not accidentally become a portable identifier.
- **Version coupling:** A Statekit-owned portable definition could couple data compatibility to crate API changes; consumer-owned schema evolution can move independently.

## 10. Open questions and follow-up research

- Which real consumer, if any, needs to share a topology DTO instead of sharing its own source definition?
- Do consumers need isolated/declared states, or is endpoint-derived state identity sufficient?
- Would a separate `MachineDefinition` reduce repeated adapter code across at least two independent consumers, enough to justify another public type?
- Should a logical definition contain an explicit state list, transition list, or both? How should it represent duplicates before validation?
- Is Serde a requested integration for Statekit itself, or is it enough that consumer DTOs derive Serde?
- If Statekit directly supports serialization, should only a portable definition serialize, leaving `Machine` opaque? What feature/version policy is maintainable?
- Should deserialization return a candidate definition or a validated `Machine`? The latter must rebuild through the public invariants.
- Are unknown fields rejected, ignored, or preserved? This should likely follow a consumer-owned schema/editor requirement rather than become a universal Statekit behavior.
- Does `transitions()` suffice as a stable *logical* export surface across future implementation changes? It suffices today, but public API changes must remain semver-aware.
- Does any consumer require canonical bytes or cross-language portable identifiers, rather than logical equality? If so, what exact compatibility guarantee is needed?
- For VPE, how many compiler checks would disappear if Statekit were used? What are the compile-time and runtime costs of string mapping or duplicate storage? Does VPE even need Statekit when its state IDs are already resolved?
- Would generic identity preserve useful `StateName` validation and diagnostics, or require a separate named-state layer? Is an adapter measurably burdensome?
- If a topology analysis API is added later, should it return analysis results without changing the portable definition schema?

### Suggested next investigations

1. **VPE projection spike, test-only:** Build a topology adapter over current `CompiledProcess`; categorize incompatible valid laws (no edges, self-edges), measure conversion cost, and test one structural property. Do not store the projection in production code during the experiment.
2. **State identity experiment:** Compare current string projection with a consumer-owned `usize -> name` mapping on representative machine sizes. Measure compilation, query, and memory behavior; include API ergonomics and diagnostics in the result, not just lookup timings.
3. **Interchange demand survey:** Identify two independent consumers and document their source formats, extension fields, isolated-state needs, and compatibility lifecycle. Only then compare consumer DTOs with a Statekit-owned portable type.
4. **Portable definition contract review:** If demand appears, draft a logical schema and round-trip properties separately from internal `Machine`; analyze versioning, ordering, validation, and unknown-field policy before choosing Serde support.
5. Continue earlier research tracks independently: delayed/accumulated diagnostics, structural algorithms, and other runtime-configured consumers should not be bundled into an interchange feature by default.

## 11. Primary sources

### Statekit and VPE

- [Statekit README](https://github.com/whitephoenixsoft/statekit/blob/main/README.md)
- [Statekit Project Core](https://github.com/whitephoenixsoft/statekit/blob/main/docs/project-core.md)
- [Statekit specification](https://github.com/whitephoenixsoft/statekit/blob/main/docs/statekit-specification.md)
- [Statekit roadmap](https://github.com/whitephoenixsoft/statekit/blob/main/docs/roadmap.md)
- [Statekit builder](https://github.com/whitephoenixsoft/statekit/blob/main/src/builder.rs), [machine API](https://github.com/whitephoenixsoft/statekit/blob/main/src/machine.rs), [transition](https://github.com/whitephoenixsoft/statekit/blob/main/src/transition.rs), [Cargo manifest](https://github.com/whitephoenixsoft/statekit/blob/main/Cargo.toml)
- [VPE compiled representation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compiled.rs), [compiler lowering](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compile.rs), [compiler validation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/validate.rs), [VPE crate manifest](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/Cargo.toml)
- [Previous report: Statekit Consumer Research & Capability Discovery](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md)
- [Previous report: Statekit Reference Model & Model-Based Testing Research](sandbox:/workspace/scratch/3524c4ebe062/statekit-reference-model-research.md)

### Interchange and state-machine systems

- [Serde overview](https://serde.rs/) and [derive documentation](https://serde.rs/derive.html)
- [XState v6 serialization](https://dev.stately.ai/docs/xstate/v6/serialization)
- [python-statemachine I/O formats](https://python-statemachine.readthedocs.io/en/latest/io/index.html), [declarative formats](https://python-statemachine.readthedocs.io/en/latest/io/formats.html), and [JSON Schema validation](https://python-statemachine.readthedocs.io/en/latest/io/json_schema.html)
- [pytransitions repository](https://github.com/pytransitions/transitions)
- [W3C SCXML Recommendation](https://www.w3.org/TR/scxml/)
- [Rust compiler IR overview](https://rustc-dev-guide.rust-lang.org/overview.html), [MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html), and [LLVM IR reference](https://llvm.org/docs/LangRef.html)
- [petgraph `StableGraph`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html)

---

## Conclusion

Statekit is already a useful validated structural projection for consumers that need a runtime-defined set of unlabelled directed edges. Its public iterator and builder provide enough interchange glue while preserving an opaque immutable implementation. That role is narrower than a general IR, and a consumer should retain richer definitions when labels, isolated states, events, multiple semantic edges, or execution metadata matter.

Keep serialized schemas and format compatibility in consumer-owned types for now. Any future Statekit-owned format should be a separate logical definition, reconstruct through existing validation, and make no unstated promise about order or bytes. VPE offers a useful test case, but its current indexed semantic representation makes an authoritative Statekit IR appear duplicative and lossy. Current evidence supports documentation and targeted experiments; it does not establish a need for new Statekit APIs.
