# Statekit — Runtime State Identity, Lowering & Representation Research

**Status:** Research proposal; no architectural decisions are made here  
**Investigation:** Sixth report in Statekit Consumer Research & Capability Discovery  
**Repositories reviewed:** `whitephoenixsoft/statekit` and `whitephoenixsoft/verified-process-engine`, default branch `main`  
**Research date:** 2026-10-13 (UTC)

## Executive summary

**Primary finding:** Statekit does not need a new *public* state identity model to address the performance opportunities visible in its v0.4 baseline. Its documented logical identity is already clear: a state is identified by its exact, case-sensitive string name. The most conservative v0.5 outcome is to confirm that strings remain the public symbolic identity, that different names never alias, and that internal layout remains private. That leaves v0.6 free to test internal indexes or dense per-machine state slots without promising a public `StateId`.

**The important distinction:** state identity, interning, indexing, and storage layout are related but different choices.

- The **logical state** is currently the exact validated state name.
- An **internal index** can be a compact implementation handle derived from that name.
- **Interning** can store each repeated name once and map it to a handle.
- **Indexes** can accelerate membership, adjacency, and transition lookup.
- A **public `StateId`** adds a new contract about machine scope, cross-machine misuse, rebuilds, and stability; it is not required for internal indexing.

**Verified current implementation:** Each `Transition` owns two private `StateName(String)` values. Repeated state names therefore occur in multiple transition endpoint strings. Transitions are stored in a `HashSet`, but direct transition lookup, state membership, outgoing-target lookup, and terminal checks scan that set. `states()` and `sources()` create temporary sets. A `Machine` shares its immutable `MachineInner` through `Arc`; each `MachineInstance` stores another `Arc` and an owned `String` for its current state. Its public state methods remain string-oriented.

**Verified performance evidence:** The frozen v0.4 baseline’s 100,000-transition linear graph measured about 146 µs for a missing direct transition, 645 µs for fully consuming `targets_from`, 25.9 ms for `states()`, and 62.1 ms for construction plus drop. The finished machine retained about 8.60 MB in the measured workload; the complete construction workload requested about 17.42 MB cumulatively and reached about 10.87 MB peak additional live memory. These measurements support testing indexes and compact storage. They do not measure an indexed design or show that numeric IDs alone improve performance.

**VPE comparison:** VPE resolves names into `HashMap<String, usize>`, stores states in a `Vec<Node>`, and stores each compiled edge’s target as `usize`. Nodes retain names, and runtime requests still carry state names; evaluation looks up the current name, follows indexed targets, and often copies the next node’s name back into runtime/event state. This is a useful compile-time lowering example, not proof that Statekit should adopt VPE’s model. VPE permits topologies Statekit does not (including zero-edge definitions and self-targets at its current validator boundary), and its multiple semantic edges can share endpoints while differing in action, priority, guards, or effects.

**Recommendations:**

1. Keep the public string API and canonical logical equality in v0.5 unless a real consumer demonstrates that passing names is a blocking integration cost.
2. Do not introduce public `StateId` or `Machine<S>` on current evidence. Public IDs require machine-scoping and stability rules; a generic machine weakens the present string-name validation model and expands nearly every API.
3. Permit an internal, machine-local dense index as an implementation experiment, but require logical equality and public behavior to ignore its assignment order.
4. Keep representation decision and optimization measurement separate: v0.5 can settle semantic/public identity; v0.6 can compare string-keyed indexes, private dense IDs, adjacency structures, and the current baseline.
5. Preserve `transitions() -> Iterator<Item = &Transition>` deliberately in any prototype or account for the API change. Replacing stored string transitions with integer pairs alone cannot supply references to the current public `Transition` type without retaining or reconstructing a string-backed view.

The strongest alternative to “strings remain the sole identity” is a **private dense state index with names retained for public queries**. The baseline supports evaluating that design. The evidence that would reverse the conservative recommendation is a same-workload prototype showing material improvement in transition/query and memory workloads after name-resolution, construction, public iterator, and equality costs are included.

## Evidence labels and continuity

- **Verified:** current repository source, tests, supplied project documents, or frozen benchmark data.
- **Established:** a property of the representation or algorithm under stated assumptions.
- **Interpretation:** a conclusion from those facts and consumer examples.
- **Hypothesis:** a possible benefit that needs a prototype, consumer example, or measurement.

The five prior reports are available as [configuration-driven consumer research](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md), [reference-model research](sandbox:/workspace/scratch/3524c4ebe062/statekit-reference-model-research.md), [structural IR and interchange research](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-ir-interchange-research.md), [structural analysis research](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-analysis-algorithms-research.md), and [validation diagnostics research](sandbox:/workspace/scratch/3524c4ebe062/statekit-validation-diagnostics-research.md).

They established that Statekit is a structural validation/runtime building block; its immutable `Machine` can serve as a validated projection or reference model; consumer formats and domain semantics remain external; configuration-facing strings are useful; structural analysis should report facts without imposing policy; and no previous investigation established a requirement for a generic identity or public ID. VPE already uses indices, but that representation is embedded in a richer process model. Those are context for this report, not conclusions immune to new evidence.

## 1. Current Statekit representation verified against source

### Public logical model

The supplied specification identifies a state by its state name: equal names identify the same logical state, names are case-sensitive, input is not trimmed or normalized, and the name must not be empty or have leading/trailing Unicode whitespace. Transitions are directed pairs of distinct state names. A machine has at least one logical transition, duplicate endpoint pairs are one logical transition, and a built machine is immutable.

The public API today is string-oriented:

- `Machine::builder()` returns a `MachineBuilder`.
- `try_allow(from, to)` accepts `impl AsRef<str>` endpoint inputs and validates each edge immediately.
- `Machine::instance(initial)` accepts `&str` and checks membership.
- `can_transition`, `validate_transition`, `contains_state`, `targets_from`, and `is_terminal` take string names.
- `states()`, `sources()`, and `transitions()` expose topology as string names and public `Transition` references.
- `MachineInstance::state()` returns `&str`; `transition_to` and `can_transition_to` take `&str`.

No public `StateId`, generic `Machine<S>`, serialization contract, or state-name-to-index conversion API exists. `StateName` is crate-private.

### Private storage and ownership

Verified source structure:

```text
Machine
  Arc<MachineInner>
    Transitions
      HashSet<Transition>
        source: StateName(String)
        target: StateName(String)

MachineInstance
  Arc<MachineInner>
  current: String
```

`MachineInner` contains only `Transitions` today. There is no persistent state table, state ID, outgoing adjacency index, reverse index, or transition-membership index. A public `Machine` does not derive `Clone`, but machine instances share the immutable inner definition with `Arc`; `MachineInstance::machine()` produces another handle by cloning that `Arc`. Independent instances keep independent current-state strings.

`Machine`, `MachineInner`, and `Transitions` derive logical equality over the contained transition set. `Transition` derives equality/hash from its endpoint `StateName` values. Hash-set equality does not depend on insertion/iteration order. `MachineInstance` derives equality from its shared machine value and current string; it does not compare `Arc` pointer identity. This matches the specification’s logical equality requirements for the present representation.

### String cost points visible in source

| Operation | Current behavior visible in source | Allocation/hash/compare consequences |
|---|---|---|
| Builder input | `try_allow` receives `AsRef<str>`; `Transition::try_new` converts each endpoint through `TryFrom<&str>` | Valid endpoint strings become owned `String` values in `StateName`. The `TryFrom<String>` implementation can move a string, but the current `Transition::try_new` path calls `as_ref()` and selects the borrowed-string conversion. |
| Unique transition storage | `Transitions.items` is `HashSet<Transition>` | Inserting hashes both string endpoints. Exact duplicates are detected by the set and discarded after their candidate transition has already been built. |
| Repeated state names | Every stored transition has its own source and target `StateName(String)` | The same logical state appearing in different edges is held in multiple owned endpoint strings; there is no canonical one-string-per-state table. |
| Machine lookup | `contains`, `contains_state`, and `targets_from` call `iter().any` or filter over the full set | String comparisons happen while scanning edges; the HashSet is not queried with a constructed edge key for these functions. Misses require a full scan. |
| State/source enumeration | `states()` and `sources()` build temporary `HashSet<&str>` collections | Hashing and comparisons are performed on borrowed string slices; the strings themselves are not cloned into these temporary sets. The set allocation/growth is per call. |
| Public endpoint access | `Transition::source()` / `target()` borrow the stored strings | No clone is needed to return a transition endpoint. Public iteration order is unspecified. |
| Instance creation | `Machine::instance` clones the input with `to_owned`; `MachineInstance::try_new` checks membership | One owned current-state string is stored per instance; the machine `Arc` is cloned, not the definition. |
| Instance transition | Validates current and target against the edge set, then `clear()` and `push_str(target)` on `current` | The current string’s capacity can be reused when sufficient; a longer target may cause reallocation. There is no separate string clone in the success path shown. |
| Failure values | Invalid move and unknown initial errors own the relevant strings | Error construction on those failure paths copies input values into the typed error. |

**Important distinction:** The implementation stores transitions in a hash set, but its public membership functions still scan. The container choice alone does not make `can_transition` a hash lookup. Similarly, a dense numeric identity is not an optimization until the relevant operations actually use an index or indexed adjacency.

### Existing performance baseline

The supplied frozen v0.4 baseline is a historical measurement, not a guarantee. It uses deterministic linear graphs of 100, 1,000, 10,000, and 100,000 transitions on a Google Pixel 10 Pro / Android 16 / native AArch64 Rust 1.90 toolchain. Graph-query timings exclude machine construction; build-and-drop includes state name generation only outside the timing, Statekit construction, and drop. No new measurements were run for this investigation.

Selected recorded data:

| 100,000-transition workload | Frozen v0.4 result | What it measures |
|---|---:|---|
| Missing `can_transition` | ~146 µs | Full transition scan for an absent edge. |
| Existing `can_transition` | ~26.6 µs in that run | Position-sensitive successful scan; not a stable scaling curve because HashSet iteration order varies. |
| `targets_from(existing).count()` | ~645 µs | Full transition traversal and filtering. |
| Missing `contains_state` | ~187 µs | Full scan for an absent state. |
| `sources().count()` | ~8.52 ms | Full traversal plus temporary unique-source HashSet construction. |
| `states().count()` | ~25.9 ms | Full traversal plus temporary unique-state HashSet construction. |
| Build and drop | ~62.1 ms | Statekit construction and machine destruction from prepared input strings. |
| Finished machine retained bytes | 8,600,385 (~8.60 MB) | Measured heap bytes live after construction for the workload. |
| Cumulative bytes requested during build | 17,422,837 (~17.42 MB) | Includes temporary input strings and freed allocations; not retained size. |
| Peak additional live memory during build | 10,873,182 (~10.87 MB) | Additional live bytes above workload start. |
| Incremental instance data | 39 bytes for the measured `state_0` case | 32 bytes per instance in the preallocated collection plus 7-byte current-state buffer; not a universal struct-size guarantee. |

The baseline explicitly warns that the allocation figures describe the complete public benchmark workload and cannot be assigned solely to Statekit’s internal representation. The results show real scan and construction costs worth revisiting. They do not compare alternatives, cover branching/dense/disconnected graph shapes, or include rooted analysis.

## 2. Identity semantics and invariants

### Core interpretation

The current specification treats a valid state name as both the user-facing label and the logical identity key. Thus:

1. Within one machine, equal exact names are the same logical state.
2. Different names remain different states; there is no aliasing layer.
3. Names are case-sensitive and not normalized.
4. A name can be represented by different implementation values in different machine builds, but those values must not alter observable machine meaning.
5. A machine-local integer assigned to `"ready"` in one machine has no established relationship to the integer assigned to `"ready"` in another machine.

This interpretation preserves INV-004, INV-005, INV-013, and INV-014. If internal numeric indices are introduced, they are derived implementation handles, not a redefinition of state identity.

### Invariant review by representation

| Invariant | String representation | Internal dense index with names | Public StateId / generic identity risk |
|---|---|---|---|
| INV-001 machine immutability | Directly supported after build | Directly supported if all tables freeze with machine | Public mutation/ID-validity model becomes part of API; still possible but more rules. |
| INV-002 reject self-transition | Compare validated names | Compare names before lowering or compare equal resolved IDs | Must not accidentally allow distinct IDs for equal names or conflate aliases. |
| INV-003 nonempty edge set | Existing builder check | Same check before or after lowering | No need to change. |
| INV-004 unique logical edge | Hash/Eq on endpoint names | Resolve names consistently, then collapse `(from_id,to_id)`; equality must reflect names | IDs from separate machines cannot define global duplicate identity. |
| INV-005 case-sensitive names | Existing validation and exact comparisons | Name map must use exact case-sensitive equality and no normalization | Generic `S` has no inherent case/name semantics; a string label layer would be needed. |
| INV-006 runtime configuration | Directly convenient for data names | Names still parse into machine-local indexes | Public IDs do not replace config names or parsing. |
| INV-007 early validation | `try_allow` checks at insertion | Check names before ID allocation/edge insertion | Generic and batch constructors must not defer current edge-local failures silently. |
| INV-009 shared immutable machine | Shared via `Arc` | Index tables can be shared in the same immutable inner | IDs tied to machine identity must work across shared handles, not just one wrapper address. |
| INV-010 valid traversal / INV-011 failed move atomicity | String state and scan | Resolve target and verify edge before replacing state ID | Unknown/foreign IDs must fail without changing state if IDs become public. |
| INV-012 instance independence | Each instance owns current String | Each instance owns a small ID | Machine-local ID state must remain per-instance, and multi-thread sharing must remain safe. |
| INV-013 machine equality | Set equality on exact edge labels | Compare logical names/edges, independent of ID assignment | Raw vector/index equality would violate this if construction order changes. |
| INV-014 instance equality | Logical machine equality plus state name | Must compare logical state name or guarantee equivalent canonical ID assignment | Raw `usize` equality is insufficient when equal machines assign IDs differently. |
| BI-002 hidden storage | Current fields are private | IDs/indexes can be private implementation details | A public ID, stability statement, or index ordering would become a domain/API commitment. |

**Specification implication:** An internal ID can be invisible to users and preserve the existing public contracts. A public ID is not just a faster integer: it needs defined scope, validity, equality, and error behavior. If a proposal changes `StateName` from logical identity to display label or permits aliases, it is a semantic change and requires explicit specification/invariant review.

## 3. Representation alternatives

| Alternative | What it means | Strengths | Costs, risks, and fit |
|---|---|---|---|
| **A. Strings remain identity** | Exact names remain both public and internal logical keys; implementation may still build auxiliary indexes. | Preserves existing API and validation; easy configuration integration; errors/readouts use names; lowest complexity. | Current scans/repeated endpoint storage remain unless implementation changes. This is a logical model choice, not a requirement to keep the current HashSet layout. |
| **B. Private numeric identity** | During construction each unique name receives a machine-local dense index; runtime and analysis structures may use IDs; public APIs map to/from names. | Natural for `Vec` adjacency, visited arrays, and compact instance state; keeps string APIs; IDs cannot leak across machines. | Needs maps/name table, custom logical equality, and a design for public `Transition` references. Construction/memory can rise; name-to-ID resolution still costs hashing when callers use strings. IDs may vary with input order. |
| **C. Public Statekit `StateId`** | `Machine` exposes state handles and methods that accept/return them. | Lets a resolved host such as a compiler avoid repeated name conversion; can enable direct numeric calls. | Must define machine scope, foreign-ID behavior, rebuild validity, stable ordering, serialization, equality, and clone/handle semantics. A bare `usize` can silently mean a different state on another machine. Not justified by current consumer evidence. |
| **D. Generic `Machine<S>`** | The state identity type is chosen by the consumer. | Could accept a consumer’s numeric or opaque key without translation; high flexibility. | Almost every API, error and iterator becomes generic; traits for equality/hash/clone/display/ownership need design. Current Unicode/name validation no longer applies to arbitrary `S` unless labels are retained separately. Type inference and docs become more complex. No multiple consumer requirement established. |
| **E. Consumer-owned mapping** | Host retains symbolic-name ↔ own-ID mapping and adapts data to Statekit; Statekit remains string-facing. | No new Statekit type or stability promise; host keeps authoritative identity and source labels; appropriate for test-only projection. | Conversion code and mappings can be duplicated; production hot path still uses string lookup unless host does not call Statekit there. VPE already has such a mapping. |
| **F. Interned/compact identity** | Store each unique label once, map it to a compact handle; edges and state instances may carry handles. | Reduces repeated string content storage and can support array-oriented indexes; can keep names available for diagnostics and iterators. | Interning itself does not create fast edge lookup; mapping/index structures still cost memory. Shared-string handles, hash map, and public iterator constraints can add complexity. Dependency not automatically warranted. |

### Distinguish the mechanisms

- **Interning** answers “how many copies of each state name are stored?”
- **Identity assignment** answers “what representation denotes this logical name within one machine?”
- **Indexing** answers “what lookup structure answers this query quickly?”
- **Layout** answers “how are edges/names/neighbors arranged in memory?”

For example, an intern table with numeric handles but a linear scan of edge pairs may reduce string storage while leaving transition lookup O(E). A string-keyed `HashMap` can index edges without exposing a `StateId`. A dense ID plus a `Vec<Vec<StateId>>` can accelerate adjacency but may make exact edge membership depend on neighbor-list search or a second edge index.

## 4. Construction and lowering

The following pipeline is a design model, not an accepted Statekit API:

```text
input string pairs
    -> validate each state name
    -> reject self-pairs; collapse duplicate logical pairs
    -> assign machine-local state slots (optional)
    -> build immutable edge/index tables
    -> expose names through the existing public API
```

### Eager lowering during `try_allow`

The builder could maintain a name map and reverse name table as edges arrive, assigning the next dense ID after endpoint validation. It could check duplicates by ID pairs. This is compatible with fail-fast validation if it validates names first and inserts no edge until the transition is valid. However:

- IDs depend on first appearance/insertion order unless the builder sorts or canonicalizes later.
- A valid source might be assigned an ID before an invalid target fails unless validation is staged first; any such work is not externally visible because the builder is consumed on error, but it can be wasted.
- A full ID index needs additional builder storage and may increase construction peak memory.
- The current builder stores a `HashSet<Transition>` and lets `build()` wrap it. Eager ID assignment changes builder internals and may help avoid repeat work at build, but no measurements show which is better.

### Deferred lowering during `build()`

Keep `try_allow` validating and storing string transitions as it does now. At `build()`, construct the machine-local name table and lower edges into a runtime form. This preserves the current public fail-fast builder behavior and puts indexing after all edge-local validation. It requires an extra pass and temporary map/table; machine construction already processes all transitions, so the incremental cost must be measured.

### Validate, deduplicate, and assign order

The logical self-transition and duplicate tests should be based on exact names. A correct name-to-ID mapping ensures equal names always map to one ID within a machine; then equal edges map to equal ID pairs. A machine built from a different input order may assign different IDs. That is safe only if:

- public names and transition sets are equivalent;
- `Machine` equality compares logical name/edge topology, not raw IDs or vector order;
- `MachineInstance` equality compares logical current state, not raw ID alone;
- serialized/debug/iteration behavior does not accidentally promise the internal assignment order.

Sorting names to make IDs deterministic is possible, but would add construction work and a determinism contract that the current API does not need. Internal IDs need not be stable across builds.

### Public transition iteration constraint

`Machine::transitions()` currently returns an iterator of references to the public `Transition` type. If internal storage becomes only `Vec<(StateId, StateId)>`, the library cannot directly yield `&Transition` values with string endpoints because no such string-backed objects are stored. Options include:

1. Retain string-backed `Transition` objects and add auxiliary ID/index structures, increasing memory.
2. Redesign `Transition` to hold shared name handles plus private IDs, while implementing equality/hash by logical names; this introduces sharing/ref-counting and more invariants.
3. Change the iterator to yield a view/value type that resolves IDs to names; this is an observable API change and may affect downstream code.
4. Keep name-backed transitions as the source of truth and add a separate private index only for hot queries.

This interface detail is a practical representation cost. BI-002 protects private storage, but not public method item types and semantics.

## 5. Runtime execution implications

### Current and possible operation costs

| Operation | Current string path | Possible internal-ID path | Caveat |
|---|---|---|---|
| `Machine::can_transition(from, to)` | Scan transitions; compare strings | Resolve both names through a name map, then query an edge index or source adjacency | IDs alone do not help unless lookup/index structure changes; the map hashes names. |
| `Machine::validate_transition` | Same scan; clone names into `InvalidTransition` on miss | Resolve names and query index; construct same string error on miss | Public error payload still requires names; failure allocation remains. |
| `Machine::instance(initial)` | Membership scan, clone initial into owned String, clone Arc | Name lookup to ID, then store ID and Arc | Name resolution still exists at initialization; per-instance current-string buffer may disappear. |
| `MachineInstance::can_transition_to(target)` | Scan using current String and target `&str` | Current ID already known; resolve target name then query outgoing/edge index | Likely better only with a target map and indexed adjacency/edge membership. |
| `MachineInstance::transition_to(target)` | Validate scan, then update owned String | Resolve target, verify ID edge, then assign ID | Failure must not update current ID. `state()` maps ID back to a stored name. |
| Current state retrieval | Borrow current String | Index name table and borrow `&str` | Still allocation-free if name table lifetime is tied to the instance/machine. |
| `is_terminal(state)` | Scan all transitions for outgoing target | Resolve name to ID, inspect adjacency length | Requires adjacency index; ID alone is not enough. |
| `targets_from(name)` | Lazy scan of all transitions | Resolve state once, enumerate neighbor IDs, map to names | Public result order remains unspecified unless newly promised otherwise. |
| `transitions()` | Borrow stored `Transition` references | Needs retained records or a new view/value API | Most significant compatibility constraint for pure ID edge storage. |
| Machine sharing | `Arc<MachineInner>` shared by instances | Same; names and indexes live inside the shared inner | Immutability allows safe shared reads, but indexes cost retained memory. |

**Repeated instance transitions:** If an application repeatedly calls `transition_to(&str)`, a numeric current-state slot can avoid rehashing/comparing the current state name, but the target name still needs resolution. If the consumer has a pre-resolved target ID and Statekit exposes an ID method, it avoids that work—but introduces public identity scope. If operations rarely transition or machines are small, conversion/index overhead may outweigh savings. No such crossover has been measured.

### Instance footprint

The v0.4 baseline reports `size_of::<MachineInstance>() = 32` bytes on the measured 64-bit target, before the owned state buffer is counted. A current instance stores an `Arc` pointer and a 24-byte `String` descriptor; an initial current-state string buffer adds heap usage depending on its length. A private `usize` state slot would conceptually reduce the inline current field to one machine word, but the new machine name/index tables add retained memory. For the benchmark’s 7-byte `state_0`, its measured per-instance heap increment was 39 bytes in a preallocated collection. These are target/workload-specific baseline facts, not a guaranteed object layout.

## 6. Structural analysis implications

The fourth investigation found no requirement yet for new Statekit graph APIs. It did find that a naive BFS built on repeated current `targets_from()` calls can scan the full edge set for every reached state (up to O(VE)). Dense internal state slots can make `visited: Vec<bool>` or `Vec<usize>` practical; adjacency arrays can make graph traversal O(V+E) with temporary output/scratch. These gains are due to the combined identity and adjacency representation, not numeric labels alone.

| Representation | Analysis consequences |
|---|---|
| Current HashSet of string transitions | Easy edge iteration and public string output; repeated `targets_from` scans; root membership also scans. An analysis can build a temporary adjacency map once, at O(V+E) expected construction plus O(V+E) BFS, with allocations proportional to the query. |
| String-keyed adjacency/index | Public roots and results map naturally; expected hash lookup by string; source/neighbor vectors may still store repeated names or references requiring ownership/lifetime design. Useful without exposing IDs. |
| Dense IDs plus adjacency vectors | Efficient visited arrays, contiguous neighbor lists, and root-to-ID resolution once; reverse mapping needed for public state names. Can support BFS/DFS/SCC without exposing IDs. |
| Petgraph conversion | Consumers can adapt topology and use broad algorithms; conversion is O(V+E) plus allocated graph storage. This is reasonable when they need a graph toolkit, but it adds little for one simple query already in Statekit. |

An internal representation can support Statekit analysis while public analysis takes/returns state names. Unknown roots must retain existing semantics where required or be distinguished explicitly in a proposed API; ID lookup naturally detects unknown names, but a boolean `false` or empty result can conflate unknown with known terminal if the API is not designed carefully.

Reachability/path queries should not become construction errors. If exposed, the root is caller-selected and output order should not depend on private ID assignment. Algorithms with filtered semantic edges (for example VPE's `AUTO_TICK` subgraph) still belong to consumers because Statekit's transitions have no action labels.

## 7. Memory and performance analysis

### Expected tradeoffs (not measured)

| Cost area | Current strings per edge | Dense internal ID + names/indexes | Consumer-owned mapping |
|---|---|---|---|
| Repeated state-label bytes | Repeated in edge endpoints | Can store each distinct name once, depending on edge/public-view design | Consumer may already own names and mapping; projection can temporarily duplicate strings. |
| Per-edge endpoint storage | Two `String` descriptors plus their buffers per stored edge | Two small integers in a pure edge record; may still retain string `Transition` or shared names for API | Statekit projection currently owns strings per edge. |
| Mapping overhead | None | Name lookup map + ID-to-name table; possibly Arc/refcount headers | Mapping already exists in consumer but translating into current Statekit still has name storage cost. |
| Adjacency | Absent; scan transitions | `Vec<Vec<Id>>` or CSR-like offsets/edge vector; O(V+E) storage | Consumer’s graph may already be indexed. |
| Exact edge lookup | Current O(E) scan | Expected O(1) with edge hash, or O(log degree) sorted adjacency / O(degree) unsorted neighbors | Depends on host representation; Statekit production calls still scan. |
| Construction | Validate, allocate endpoint strings, hash insert | Validate plus intern/resolve names, build indices and adjacency, possibly sort | Adapter conversion plus Statekit construction. |
| Instance state | Owned string per instance | One ID per instance, reverse lookup for public `state()` | Host may store ID and not use Statekit in runtime. |
| Tiny machine overhead | No name map/adjacency table beyond HashSet | Fixed map/vector/hash capacity may dominate small graphs | Conversion can dominate whole workload. |

Dense numeric adjacency generally offers low per-edge storage and direct array access once IDs exist, but the extra state map/name table and any retained `Transition` view can offset savings. Hash-based indexes can speed lookup while increasing retained memory. Sorting adjacency can permit binary-search membership and deterministic internal order but adds build sorting and does not require public deterministic iteration. A temporary analysis workspace trades repeated build time and peak memory for no persistent machine overhead.

### Required benchmark matrix

No candidate representation was implemented or timed here. A useful v0.6 experiment should compare the existing v0.4 form against realistic alternatives using the same API-level workload:

| Dimension | Cases |
|---|---|
| Transition counts | 10, 100, 1,000, 10,000, 100,000 where practical; report both input count and unique final edges. |
| Shapes | Linear chain; branching tree; high-outdegree hub; sparse disconnected graph; dense directed graph at bounded sizes; large cyclic region plus exits; many terminal states. |
| Names | Short names and long UTF-8 names; repeated high-fanout state names; unique names; whitespace-validation failures separately. |
| Input characteristics | Normal unique edges; duplicate-heavy valid input; invalid edge input measured separately from valid construction. |
| Runtime query workloads | Existing/missing `Machine::can_transition`; name-resolution-only; indexed edge lookup-only; `contains_state`; instance creation; allowed/failed `transition_to`; repeated transitions on one instance; many independent instances. |
| Structural query workloads | Full `targets_from`; `states()`/`sources()`; rooted reachability from source, sink, hub, and disconnected root if prototype exists. |
| Representations | Frozen current source; string-keyed maps/adjacency; dense private state slots + adjacency; dense slots + a direct edge index; optionally consumer conversion to Petgraph. Avoid testing a theoretical design that does not preserve public API. |
| Metrics | Construction latency, query latency, name-resolution latency, full operation latency, allocations, cumulative allocated bytes, retained machine bytes, peak construction bytes, incremental instance bytes, and implementation complexity/API changes. |

Benchmark small machines as well as large ones. If an ID design wins only for 100k edges but is slower and much larger at 10–100 edges, the decision depends on demonstrated target workload. Keep current v0.4 measurements frozen and compare equivalent workloads; do not reinterpret its linear-only graph as a complete future suite. Use the same or a clearly documented hardware/toolchain setup and run enough samples to report uncertainty. Do not combine lookup results with parsing, string formatting, graph generation, or setup unless the benchmark explicitly intends to measure those costs.

### Smallest decisive prototype

Build two non-public implementations behind the same string API:

1. **String-keyed index:** retain canonical state names and implement name-keyed outgoing/edge/state indexes.
2. **Dense private slots:** intern each exact name once, map `&str` to a dense slot, use slot adjacency and a deliberate public `Transition` view strategy.

Compare them with the frozen current implementation on a linear graph, branching graph, sparse disconnected graph, and repeated runtime transitions, at 10, 100, 1k, 10k, and 100k edges; add dense and long-name cases if either candidate is close. Measure the index-construction and name-resolution phases separately. This tests whether *numeric identity itself* adds value beyond indexing and whether the public iterator/label storage defeats expected memory savings.

## 8. VPE compiler and runtime identity

### Verified current pipeline

VPE's source model (`LawSource`, `StateSource`, `TransitionSource`) is symbolic and uses strings. Each state declares its transitions; each transition carries an action, target name, priority, guards, effects, and optional comment. The source model is Serde data.

The compiler builds a `HashMap<String, usize>` from state names in source order. It constructs `Vec<Node>`; each `Node` retains a cloned state name and its `Vec<Edge>`. An `Edge` stores `target_idx: usize` alongside its semantic action, priority, guard objects, and effects. `CompiledProcess` retains `initial_state_idx`, the nodes, and the name-to-index map. Thus the compiled form contains both symbolic names and numeric references; the mapping’s key strings and the node names are built as owned strings.

At evaluation, the request still supplies `current_state: String`. The runtime performs `process.state_index(&current_state)` to get an index, selects the node, and uses each edge’s `target_idx` to find the next node. It then clones the next node’s name into the runtime `current_state` and emitted event state. This is an indexed-target architecture with string-facing request and output boundaries, not a wholly numeric runtime API.

VPE has no Statekit dependency in its crate manifest. Its own validator handles target resolution, state uniqueness, initial-state requirements, guards/effects, `AUTO_TICK`, and other process rules. It can accept a nonempty state list with no transitions (as terminal warnings) and currently does not apply Statekit’s self-transition rejection. Multiple VPE semantic transitions can share the same source and target while differing in action, priority, guards, or effects. Statekit stores one logical endpoint pair and rejects self-transitions, so a direct projection can erase or reject VPE meaning.

### Integration choices

| Option | Potential gain | Main cost / reason it may not fit |
|---|---|---|
| **1. No integration** | VPE keeps its authoritative semantic compiled representation and indexed runtime. No adapter/dependency. | VPE retains its own structural checks. Current VPE already has names and IDs. |
| **2. Test-only projection** | An independent structural oracle can check selected flat topology fixtures or sequences. No production runtime cost. | Projection must deduplicate or exclude self/parallel edges; tests cover only structural endpoint behavior, not action/guard selection. |
| **3. Production structural validation** | Could reuse Statekit name/self/edge validation if VPE intentionally adopts exactly those rules. | It would reject VPE-accepted cases and cannot represent transition multiplicity. It may duplicate VPE’s compile-time map and check without removing meaningful code. |
| **4. Shared public StateId** | Could let VPE pass its resolved index directly to Statekit. | IDs would need shared scope/validity semantics; VPE IDs are compiler-instance positions, not Statekit logical identity. It still does not unify semantic edges. High coupling for little evidenced gain. |
| **5. Consumer-owned name/ID map** | VPE’s current pattern already resolves names to its own `usize`; Statekit could remain string-facing in test or projection code. | Translation and temporary topology construction remain if Statekit is called; not useful in VPE’s hot runtime unless Statekit itself is integrated. |

**Evidence-weighted view:** No integration is fully reasonable. The smallest experiment, if a concrete test needs one, is a test-only Statekit projection of a small VPE fixture with unique, non-self endpoint pairs, plus explicit tests for source-to-target topology and a note that semantic edge multiplicity is intentionally omitted. Do not benchmark a production integration before deciding which VPE structural code it would replace. VPE’s internal numeric representation does not make a shared public StateId necessary.

## 9. Rust ecosystem comparisons

| Library / system | Identity and purpose | Useful lesson for Statekit | What not to import by default |
|---|---|---|---|
| **petgraph** | General-purpose graph types expose node/edge indices; `Graph` and `StableGraph` have different index stability behavior, and `StableGraph` preserves indices across removals. | A graph can use opaque integer-like indexes internally while consumers retain node weights/labels. Query needs should determine graph type and index contract. | Statekit is immutable and domain-narrow; it does not need deletion stability, arbitrary graph mutation, edge weights, or petgraph’s whole algorithm surface. Converting to petgraph remains a consumer adapter when broad analysis is needed. |
| **slotmap** | Returns typed, versioned keys for values inserted in mutable maps; keys support lookup/removal. | Newtype keys prevent mixing categories more effectively than bare integers. | Statekit machines are immutable after build and do not reuse deleted state slots, so generational key machinery is not inherently needed. |
| **generational-arena** | Index plus generation protects against stale keys after slot reuse/deletion (ABA-style problem). | Generations solve a lifecycle problem, not a general numeric identity problem. | No deletion/reuse in built Statekit machines; generation counters add footprint and semantics without current need. |
| **lasso** | String interning maps strings to compact `Spur` handles; offers single-threaded and threaded interner variants, with explicit access to the interned strings. | Demonstrates that compact symbols need an interner and reverse lookup. Interner handles belong to an interner context, not a portable/global state identity. | A new dependency and interner API should be justified by measured name-storage pressure. Statekit can prototype a small per-machine map first. |
| **rustc `Symbol` pattern** | rustc uses interned UTF-8 strings represented by compact symbols and resolves them through compiler-session symbol tables. | Compiler IDs are often context-scoped; names remain recoverable through the table, and numeric symbol values are not a portable language identity. | rustc’s session globals, symbol universe, hygiene, and compiler infrastructure are far beyond Statekit’s required responsibilities. |
| **rust-fsm / statically typed FSMs** | Rust FSM libraries such as `rust-fsm` generate or use explicit enums/specifications for states; state identity is type-level or enum-like rather than arbitrary runtime text. | Typed-state libraries solve a different use case and make state identity explicit at compile time. They do not show that a data-driven runtime library should genericize its identity. | Static enums, proc-macro generation, compile-time transition exhaustiveness, and type-level state tokens conflict with Statekit’s runtime-configured string use unless offered as a separate adapter. |

**Conclusion from comparison:** Newtype and interner patterns are techniques, not requirements. The main transferable design principle is scope: a compact key is valid relative to a graph/interner/compiler context unless a system explicitly defines portable identity. Statekit’s current logical names already provide portable comparison and understandable diagnostics.

## 10. Five worked examples

### Example 1 — Equivalent machines, different insertion order

Current APIs can construct logically equal machines from different edge orders:

```rust
use statekit::{Machine, StateError};

fn equivalent_machines() -> Result<(), StateError> {
    let first = Machine::builder()
        .try_allow("queued", "running")?
        .try_allow("running", "done")?
        .build()?;

    let second = Machine::builder()
        .try_allow("running", "done")?
        .try_allow("queued", "running")?
        .build()?;

    assert_eq!(first, second);
    Ok(())
}
```

**Verified:** Current equality is based on transition-set equality, not insertion order. If a private ID is assigned at first occurrence, the IDs for `queued` and `running` can differ across these builds. That must not make `first != second`. Instance equality must likewise compare the same logical current name in equal machines, not raw private ID alone. Sorting states for a canonical ID assignment could avoid that difference, but adds construction cost and is not needed to preserve equality.

### Example 2 — Symbolic strings lowered to private IDs

Illustrative topology:

```text
queued -> running
running -> done
```

One hypothetical private lowering could be:

```rust,ignore
// Hypothetical internal representation, not Statekit source/API.
struct StateId(usize);

struct LoweredMachine {
    names_by_id: Vec<String>,
    id_by_name: HashMap<String, StateId>,
    outgoing: Vec<Vec<StateId>>,
    edges: HashSet<(StateId, StateId)>,
}
```

With one assignment, `queued -> 0`, `running -> 1`, `done -> 2`. Another build could assign different numbers while representing the same named topology. `id_by_name` resolves public strings; `names_by_id` reconstructs public output; `outgoing` supports adjacency; `edges` permits expected constant-time edge membership. This sample duplicates name content between the vector and map and stores adjacency plus edge pairs, so it is explanatory rather than a memory recommendation. An `Arc<str>` or a different edge layout could reduce copying but changes cost and complexity. A query index without a reverse name table cannot preserve public state/transition introspection.

### Example 3 — Independent instances sharing one immutable machine

**Current API:**

```rust
let machine = Machine::builder()
    .try_allow("queued", "running")?
    .try_allow("running", "done")?
    .build()?;

let mut a = machine.instance("queued")?;
let mut b = machine.instance("queued")?;

a.transition_to("running")?;
assert_eq!(a.state(), "running");
assert_eq!(b.state(), "queued");
```

Both instances share the machine’s immutable `Arc<MachineInner>`; their current strings are independent. A hypothetical internal-ID instance could store `current: StateId` and return `&names_by_id[current]` from `state()`. On `transition_to(target_name)`, it would resolve the target name, validate the indexed edge, and then assign the ID. Failed lookup must leave the old ID intact. It saves a per-instance string buffer but adds mapping/index memory to the shared machine and still performs target-name resolution through today’s public method.

### Example 4 — Cross-machine ID misuse

Suppose `Machine A` maps `0 -> queued`, `1 -> done`, while `Machine B` maps `0 -> approved`, `1 -> archived`. If a public API accepts `StateId(1)` without checking its owner, a handle from A used with B silently refers to `archived`. A wrapper type around `usize` prevents confusing state IDs with transition IDs, but alone does not prevent mixing two machines’ state IDs.

Possible public safeguards include an owner token embedded in the ID, an ID tied to a machine handle, or methods that return scoped state references. Each adds storage, lifetime, equality, and clone/handle questions; Rust lifetimes alone do not automatically prove two values came from the same machine instance. A global atomic machine serial would add synchronization/global identity; pointer-based identity has lifetime/address-reuse concerns if IDs outlive their owner. A machine-scoped ID must either be impossible to misuse by construction or detect foreign keys and return an explicit error.

**Interpretation:** Since no current consumer requires public IDs, keeping all machine-local indices private avoids this class of contract entirely. A consumer with its own IDs can keep its own map and use the existing string API for projection or testing.

### Example 5 — VPE compiled process projection

Representative VPE structure:

```text
LawSource states:
  "draft": Submit -> "review" [guards/effects/priority]
  "review": Approve -> "done" [guards/effects/priority]

CompiledProcess:
  state_index: {"draft": 0, "review": 1, "done": 2}
  nodes: Vec<Node>
  Edge { action: Submit, target_idx: 1, priority, guards, effects }
```

**Preserved by an endpoint-only Statekit projection:** the unique pairs `draft -> review` and `review -> done` can be structurally checked and traversed. **Lost:** action labels, priority, guard semantics, effects, and distinction among same-endpoint VPE edges. **Potential mismatch:** VPE can have no transitions or a self-edge that its current checks do not reject; Statekit requires at least one edge and rejects self-transitions.

VPE could project names before/after compiling, or use its own name-to-index map. The latter is already present. Production Statekit integration would require meaningful code elimination or an independently valuable structural contract; merely converting indices to names and back is not simplification. A test-only projection is safer, but should assert the expected unique topology independently so the projection does not become its own oracle.

## 11. API compatibility analysis

| Existing API/use | Can remain unchanged with private IDs? | Additional requirement |
|---|---|---|
| `Machine::builder()` | Yes | Builder can keep accepting names; lower internally during build or insertion. |
| `try_allow(from, to)` | Yes | Validate original names exactly and retain fail-fast return timing. Duplicate logical pair remains idempotent. |
| `Machine::states()` / `sources()` | Yes | Map private IDs to borrowed names; keep unspecified order unless deliberately revised. |
| `Machine::transitions() -> &Transition` | Not automatically | Retain string-backed transitions, give `Transition` shared string handles, or make a breaking iterator item/view change. |
| `Machine::can_transition(&str, &str)` | Yes | Resolve names internally and preserve exact-match behavior/errors. No need to expose IDs. |
| `Machine::instance(&str)` / `MachineInstance::state() -> &str` | Yes | Resolve initial name to ID; map ID back to a stable borrowed name for `state()`. |
| Instance equality | Yes, but custom logic may be needed | Compare logical machine plus logical state name, not raw ID if assignments differ. |
| `StateError` payloads | Yes | Continue reporting names, including unknown names; if public IDs are accepted, add foreign/unknown-ID error semantics. |
| Public `StateId` methods | No, new surface | Define ID scope, stability, conversion, equality, foreign-machine rejection, serialization, and machine-handle behavior. |
| `Machine<S>` | No, broad change | Generic bounds and error/output types propagate through most public APIs; old ergonomic string API needs adapter/compatibility route. |

**Reversibility:** Private indexing and internal interning can be changed later if public semantics and `BI-002` are respected. Public ID exposure is harder to reverse because consumers may store, serialize, compare, or rely on its scope. A generic type parameter is also a lasting public API choice. No public identity should be exposed before its lifetime and compatibility semantics are settled.

## 12. Architecture decision matrix

Qualitative ratings below are architectural expectations, **not measured benchmark results**. “Good” means a favorable fit to Statekit’s current goals; “mixed” or “costly” flags tradeoffs.

| Dimension | A. Strings remain identity | B. Private dense IDs + names | C. Public StateId | D. Generic `Machine<S>` | E. Consumer-owned mapping | F. Interned private names |
|---|---|---|---|---|---|---|
| Public API simplicity | **Good** | **Good** if hidden | Costly | Costly | Good for Statekit; adapter burden to host | Good if hidden |
| Backward compatibility | **Best** | Good if iterator/equality preserved | New additive surface, semantics hard to reverse | Broad compatibility/type change | Best inside Statekit | Good if hidden |
| Invariant preservation | Direct | Good with name-based validation and logical Eq | Requires foreign/unknown ID rules | String invariants need separate label design | Host handles mapping; Statekit unchanged | Good if interner equality remains exact |
| Logical equality | Directly implemented | Custom compare names/edges independent of slot order | Must not equate IDs across machines | Depends on `S` equality and labels | Host decides mapping; Statekit remains logical names | Compare original strings, not interner addresses |
| Construction complexity | Current baseline | Moderate/high | High | High | External conversion | Moderate |
| Runtime lookup potential | Current scans are slow at scale | High if paired with indexes; name resolution remains | Highest for resolved callers if direct-ID lookup | Depends on ID type and data structure | Good in host runtime; Statekit not necessarily involved | Moderate; still needs indexes |
| Structural analysis potential | Possible with temporary adjacency | Strong for dense vectors/scratch | Strong but API not needed for internals | Depends on generic identity | Host can analyze its own IDs | Strong if paired with adjacency |
| Retained memory | Current baseline ~8.60 MB at 100k workload | Could fall per edge but adds name maps/indexes and public-view constraints | Similar internal memory plus public scope metadata | Depends on S; no guarantee smaller | Mapping may be duplicated across host/Statekit | Can reduce repeated label buffers, adds table/handles |
| Small-machine overhead | Lowest API and map overhead | Potentially poor relative overhead | Poor | API overhead high | Conversion cost can dominate | Interner setup may dominate |
| Large-machine scalability | Current scans scale linearly per query | Likely favorable for adjacency/array work, **unmeasured** | Potentially good | Potentially good | Consumer-specific | Helps storage, not sufficient for lookup |
| VPE fit | Projection by name; no shared IDs | Could match VPE indexing internally, but duplicates representation | Attractive only if shared ID contract is sound | Can accept `usize` but major Statekit API expansion | VPE already owns this map | Could deduplicate labels but VPE already indexed |
| Dependency burden | None | None if implemented locally | None but API obligations | None | None | Could be none; `lasso` would add dependency |
| Implementation risk | Low | Moderate/high due Eq/iterator behavior | High | High | Low for test adapter | Moderate |
| Long-term maintenance | Lowest | Manageable if private and benchmarked | High | High | Consumer owns complexity | Moderate |

**Strongest alternative to current internal strings:** private dense IDs plus a name table and outgoing/edge indexes. It is the strongest because current benchmark/source evidence identifies repeated scans and endpoint storage as measurable costs, and dense slots naturally support analysis. It is not the current recommendation for implementation because no alternative has been measured and `transitions()` imposes a string-reference constraint.

## 13. Capability and architecture classification

### A. Existing design remains sufficient

- Strings are the documented logical identity, suitable for configuration and runtime definitions.
- Existing APIs let callers build machines, traverse with independent instances, query topology by names, enumerate names/transitions, and compare logical definitions.
- A consumer that already has IDs can retain its own ID mapping and use Statekit as a structural projection/reference model.
- No public StateId or generic machine is needed for these uses.

### B. Internal-only optimization

- Add private name-to-slot and slot-to-name tables if a prototype justifies them.
- Add private edge membership, outgoing adjacency, state membership, or cached projections based on benchmarked query mix.
- Store current instance state as a private slot if it reduces runtime/instance costs while `state()` remains `&str` and equality remains logical.
- Intern repeated names internally if measurements show endpoint string storage dominates.
- These are implementation changes under BI-002, but must preserve public iterator semantics and documented order guarantees.

### C. Potential v0.5 architectural decisions

- Confirm exact state names remain canonical logical identity or explicitly specify a different model.
- Decide that IDs, if any, are implementation handles and not portable identities.
- Decide whether public APIs remain string-based; current evidence favors yes.
- Decide that machine/instance equality is representation-independent.
- Decide whether exposing `StateId` is deferred pending consumer evidence; current evidence favors deferral.

### D. Potential v0.6 optimization decisions

- Compare string-keyed indexes with private dense slots.
- Choose edge membership strategy (hash pair, sorted adjacency, adjacency scan) from workload evidence.
- Evaluate current transition storage versus shared/interned names and ID edge storage.
- Measure construction/peak memory against runtime and analysis lookup benefits.
- Retain or change `Transition` storage only after resolving the public `transitions()` reference constraint.

### E. Consumer-owned responsibilities

- Map database keys, compiled symbols, or application IDs to Statekit names.
- Keep external names, aliases, serialization IDs, schema versions, and migration mappings.
- Decide whether two host labels alias the same domain entity; current Statekit treats distinct names as distinct states.
- Preserve source metadata and rich VPE edge semantics.
- Use petgraph or another graph library when broad graph operations are needed.

### F. Out of scope

- Global/distributed state identity registry.
- Deletion-safe/generational identity lifecycle for immutable built machines absent a deletion need.
- Compiler pass infrastructure or symbol resolution for consumer languages.
- General-purpose mutable graph store.
- Domain state aliases, guard/effect/action identity, process semantics, or workflow execution.
- Making internal index order or HashSet iteration portable/canonical without an evidenced requirement.

## 14. Ranked recommendations and decision gates

### Ranked recommendations

1. **Keep the current public string identity in v0.5 unless evidence changes.** The specification already states that names identify states and the existing config-driven consumer need aligns with it. Confirm that names are logical identity, case-sensitive, exact, and non-aliasing. This gives the roadmap a meaningful result even if no new ID type is introduced.
2. **Do not expose numeric IDs yet.** No prior report shows a consumer blocked by string queries. Public IDs create scope and cross-machine validity questions and are harder to retract than private indexes.
3. **Treat private dense IDs as an optimization candidate, not a required identity redesign.** Statekit can preserve public name APIs while privately lowering names if the benchmark shows a net gain. Ensure all equality and error behavior remains based on logical names.
4. **Benchmark string-keyed indexing and dense IDs side by side before v0.6 chooses storage.** The current baseline shows the need to compare, but not which alternative wins. Include `transitions()` iteration and memory—not only `can_transition`—because these can reverse the apparent benefit.
5. **Avoid `Machine<S>` unless multiple consumers prove a need for generic identity.** It changes validation semantics, errors, almost every API signature, and documentation load. First test a consumer-owned mapping or a private index.
6. **Use VPE as a compatibility/test case, not the deciding consumer.** Its own indexed runtime already exists and its semantic edges do not reduce to endpoint pairs. Any integration should name code it removes and the subset of VPE definitions it supports.

### Explicit decision gates

- **Keep strings internally:** choose if current implementation meets the scale/query goals, if indexes can be added without lowering, or if mapping costs outweigh measured query wins.
- **Use private internal IDs:** choose if same-interface benchmarks show material wins in realistic large/repeated-query workloads and preserve memory/constructor goals. IDs remain machine-local and unstable across rebuilds.
- **Expose public StateId:** consider only if consumers repeatedly possess resolved IDs and profiling shows name resolution is a material remaining cost; require unambiguous foreign-machine rejection, rebuild and stability rules, equality behavior, and transition iterator compatibility.
- **Adopt `Machine<S>`:** consider only if several consumers need different identity types and can state common validation/equality/error expectations; a generic parameter should not merely mirror one compiler’s `usize`.
- **Lower in builder or build:** decide from construction and peak-memory benchmarks. Preserve fail-fast endpoint validation regardless. Eager lowering may reuse ID maps during insertion; deferred lowering keeps builder symbol-oriented but adds a conversion pass.
- **Add persistent indexes:** choose based on repeated lookup/analysis frequency and memory budgets. v0.5 identity decisions do not require that every v0.6 index be adopted.

### v0.5 versus v0.6 boundary

| v0.5 Multiple Representations | v0.6 Optimization |
|---|---|
| Define the logical identity: exact validated symbolic name, or a changed model (no evidence currently supports changing it). | Measure and choose internal lookup/storage structures. |
| Decide whether public APIs remain string-based and whether public IDs are deferred. | Compare string indexes, dense slots, interning, adjacency and edge membership. |
| Specify equality independent of internal layout/order. | Measure construction, runtime, analysis, memory, allocation and instance costs. |
| Specify whether name-to-ID mapping is private and machine-local. | Select eager/lazy/cached indexes only if workload supports their build/retained cost. |
| Do not assume an ID implementation; preserve the option. | Do not change public identity solely because internal arrays are convenient. |

The clean separation is: v0.5 decides *what a state means to callers*; v0.6 measures *how that state is represented internally for performance*.

## 15. Risks and unresolved questions

### Risks

- Raw integer equality or machine-vector equality can violate logical equality when IDs depend on insertion order.
- Public IDs with no machine ownership can silently address the wrong state in another machine.
- Stable IDs across rebuilds would require a canonical ordering or persistent external identity and would add compatibility obligations.
- Internal dense indexing may duplicate the existing string-backed transition representation to preserve `&Transition` iteration, erasing expected memory savings.
- Changing `transitions()` to return views can be source-breaking even when private fields remain hidden.
- String-to-ID lookup can be a significant part of the path when public APIs continue to accept names; integer edge lookup does not remove this cost.
- An interner can shrink repeated label storage but does not fix O(E) scans unless paired with indexes.
- Dense arrays may waste capacity for sparse/high-ID layouts; current built machine is dense only if IDs are reassigned during compilation.
- Small machines may regress due fixed table allocations and setup costs.
- Deterministic index assignment is not necessary for logical equality and should not be accidentally promised by iteration or serialization.
- VPE's source and compiled representations already duplicate some name/index information. Using Statekit IDs could create a third identity layer rather than remove one.

### Questions to preserve

- Does the project intend the phrase “state name” to be a public label plus canonical identity, as the spec currently says, or does it want aliases in the future?
- Can an internal ID improve machine construction/retained memory while preserving public `&Transition` references without keeping duplicate strings?
- Does the 100k benchmark workload represent actual target users, and what size/query mix should define “small and fast”?
- Are users likely to retain a state handle beyond one `Machine` value, or is string addressing sufficient?
- Would VPE ever benefit from using Statekit during production compilation, given its indexed nodes and semantic parallel edges?
- Should machine equality compare explicit state sets as well as edges? Current states are inferred from endpoints, so edge topology determines represented states; declared isolated states are not representable.
- If IDs are private, should construction assign them incrementally or lower at build time? Which has better peak memory and duplicate behavior?
- Should `MachineInstance::state()` remain a name query if the current state is numeric internally? This appears necessary for compatibility but should be explicitly accepted.
- What exact error should a future public-ID method return for a foreign ID or ID invalidated by rebuild? The current design avoids this by not exposing IDs.

## 16. Suggested next experiments

1. **Representation micro-prototype:** In a scratch branch only after implementation is authorized, compare the current baseline, string-keyed indexes, and private dense slots behind the same public API. Do not change the frozen baseline.
2. **Equality conformance:** Build equal machines in different insertion orders and compare `Machine`; create instances whose logical states match across those machines and compare them. Run this before evaluating any ID-backed representation.
3. **Transition iterator prototype:** Sketch how `transitions()` can continue yielding borrowed public `Transition` values under each candidate. Include the resulting retained bytes; this may decide between storage alternatives before broad coding.
4. **VPE adapter experiment:** Use one VPE fixture to project only unique non-self topology pairs in tests. Count conversions, retained structures, and checks actually removed. If nothing is removed, keep it test-only or do not integrate.
5. **Target workload evidence:** Gather expected machine-size distribution and query rates from at least one configuration-driven consumer and one compiled consumer before choosing persistent indexing.

## 17. Primary-source references

### Statekit and supplied project documents

- [Statekit README](https://github.com/whitephoenixsoft/statekit/blob/main/README.md)
- [State name implementation](https://github.com/whitephoenixsoft/statekit/blob/main/src/state_name.rs)
- [Transition implementation](https://github.com/whitephoenixsoft/statekit/blob/main/src/transition.rs)
- [Builder implementation](https://github.com/whitephoenixsoft/statekit/blob/main/src/builder.rs)
- [Machine API](https://github.com/whitephoenixsoft/statekit/blob/main/src/machine.rs)
- [Machine instance](https://github.com/whitephoenixsoft/statekit/blob/main/src/instance.rs)
- [MachineInner](https://github.com/whitephoenixsoft/statekit/blob/main/src/inner.rs)
- [Transition collection](https://github.com/whitephoenixsoft/statekit/blob/main/src/transitions.rs)
- [Property tests](https://github.com/whitephoenixsoft/statekit/blob/main/tests/property_tests.rs)
- [Integration tests](https://github.com/whitephoenixsoft/statekit/blob/main/tests/transition_tests.rs)
- [Frozen v0.4 benchmark baseline](https://github.com/whitephoenixsoft/statekit/blob/main/docs/benchmarks/baseline-v0.4.md)
- Supplied local [Project Core](sandbox:/workspace/scratch/3524c4ebe062/project_sources/02-project-core-2-.md), [specification](sandbox:/workspace/scratch/3524c4ebe062/project_sources/01-statekit-specification.md), [README snapshot](sandbox:/workspace/scratch/3524c4ebe062/project_sources/03-README-1-.md), and [roadmap](sandbox:/workspace/scratch/3524c4ebe062/project_sources/04-roadmap-2-.md).

### VPE

- [VPE source definitions](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/source.rs)
- [VPE compiler lowering](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compile.rs)
- [VPE compiled representation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compiled.rs)
- [VPE evaluator](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/runtime/evaluate.rs)
- [VPE structural and semantic validation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/validate.rs)
- [VPE crate manifest](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/Cargo.toml)

### Rust ecosystem identity and graph references

- [petgraph `Graph`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html) and [`StableGraph`](https://docs.rs/petgraph/latest/petgraph/stable_graph/index.html)
- [slotmap documentation](https://docs.rs/slotmap/latest/slotmap/)
- [generational-arena documentation](https://docs.rs/generational-arena/latest/generational_arena/)
- [lasso documentation](https://docs.rs/lasso/latest/lasso/)
- [rustc interned `Symbol` source](https://rust.googlesource.com/rust/%2B/c96d970f2ddb4978627bb8ec23b33d64f8933274/compiler/rustc_span/src/symbol.rs)
- [rust-fsm source repository](https://github.com/eugene-babichenko/rust-fsm)
- [Statekit prior research reports](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md), [reference model](sandbox:/workspace/scratch/3524c4ebe062/statekit-reference-model-research.md), [IR and interchange](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-ir-interchange-research.md), [structural analysis](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-analysis-algorithms-research.md), and [diagnostics](sandbox:/workspace/scratch/3524c4ebe062/statekit-validation-diagnostics-research.md).

## Conclusion

Statekit’s existing string model is sufficient as the public logical identity and remains the best-supported v0.5 choice. The v0.4 baseline gives a concrete reason to investigate a more efficient private representation: transition and state queries scan edge storage, repeated names are held in edge endpoints, and the largest linear-machine construction has substantial allocation traffic. It does not tell us whether string-keyed indexes, dense IDs, interning, or a combination is best.

The smallest coherent path is to keep exact string names as the canonical identity, leave public APIs and equality semantics representation-independent, and measure private indexing/lowering alternatives in v0.6. A public `StateId` or generic `Machine<S>` should wait for consumer evidence that private optimization and consumer-owned mappings cannot meet the need. VPE validates the practicality of name-to-index lowering while also showing why sharing its IDs with Statekit would create coupling: its semantic transition model is richer and its structural invariants differ.
