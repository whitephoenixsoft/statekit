# Statekit — Structural Analysis Algorithms & Consumer Query Research

**Status:** Research proposal; no roadmap or API decision is made here  
**Investigation:** Fourth report in Statekit Consumer Research & Capability Discovery  
**Repositories reviewed:** `whitephoenixsoft/statekit` and `whitephoenixsoft/verified-process-engine`, default branch `main`  
**Research date:** 2026-10-12 (America/New_York)

## Executive summary

Statekit already has local structural queries: direct transition membership, outgoing targets, states, sources, transitions, and terminality. It has no public incoming-edge, reachability, path, cycle, component, or degree-analysis APIs. The current implementation stores transitions in a `HashSet` without persistent adjacency indexes. Consequently, iterating every transition once is linear, but repeatedly calling `targets_from` while traversing many states scans the transition set once per state and can make a naive BFS O(VE), not O(V+E).

**Smallest coherent analysis foundation:** preserve the current API as-is for now. If a consumer experiment confirms repeat demand, the first candidate is a caller-rooted reachable-state query implemented with a temporary adjacency workspace. It answers reachability and unreachable-from-root questions and can serve as the basis for a path-existence check. A read-only incoming-transition query is a smaller, independent candidate; it can be implemented as a transition scan with no persistent index. These are optional observations, never new build failures.

Cycle detection and strongly connected components (SCCs) are mathematically useful, but no Statekit consumer requirement justifies exposing them yet. SCCs reveal more than a yes/no cycle check: mutually reachable regions, regions with exits, and sink/terminal components. But they add result-shape and ordering questions; dedicated graph libraries already provide linear-time SCC and related algorithms. Keep SCCs, condensation graphs, topological sorting, all-path enumeration, and broad connectivity APIs outside Statekit until a concrete consumer shows a repeated, state-machine-specific need.

The practical-performance constraint is central. Statekit's v0.4 baseline deliberately measures 100 through 100,000 transitions, but uses only a linear graph and has no graph-analysis benchmark. The baseline records that transition membership and outgoing-target inspection scan the transition collection, `states()`/`sources()` create temporary sets, and no persistent indexes are maintained. A one-query temporary adjacency map may be a reasonable O(V+E) tradeoff. Eager indexes or caches would affect construction, memory, and possibly hot transition lookups; immutable machines make caches safe to share, but do not make their cost free.

VPE confirms both the value and the limit. It computes root-relative reachability over all declared transitions and checks cycles only in its `AUTO_TICK` subgraph. Statekit could express the first as topology reachability only if the current representational mismatch is handled. It cannot replace the second by running SCC or cycle detection over the full unlabelled topology: action-filtered edges have different meaning. VPE also already owns an indexed compiled topology, so direct adoption is not automatically simpler.

**Recommendation:** document what consumers can derive now; prototype reachability against a local adjacency map only when a second real use joins VPE; then benchmark against existing scan-based calls and an external `petgraph` conversion. Do not introduce a cache, SCC suite, or graph-library dependency on current evidence.

## Evidence and continuity

- **Verified:** stated in current Statekit/VPE source or docs, or a linked primary external source.
- **Established:** mathematical result for the stated graph model and representation assumptions.
- **Interpretation:** a conclusion drawn from verified behavior and consumer examples.
- **Hypothesis:** a possible consumer benefit without measured use.

The three prior reports are available as [configuration-driven consumer research](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md), [reference-model and model-based testing research](sandbox:/workspace/scratch/3524c4ebe062/statekit-reference-model-research.md), and [structural IR and interchange research](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-ir-interchange-research.md). Findings preserved here:

- Statekit already validates dynamic topology, supports reference-model tests, and can act as an immutable structural projection.
- Consumer serialization remains preferable to a Statekit-owned wire format.
- Structural observations should not silently become construction validity rules.
- Caller-rooted reachability and predecessor inspection recur as candidates, but none of the earlier reports established that a new API is required.
- VPE already has structural reachability and an `AUTO_TICK`-specific cycle check; its guards, actions, priorities, effects, and process rules remain semantic.

The supplied README, Project Core, specification, and roadmap were reviewed, along with current source, tests, benchmark documentation, benchmark baseline, and VPE compiler structures.

## 1. Verified current analysis capabilities

### Statekit today

`Machine` exposes:

- `can_transition(from, to)` and `validate_transition(from, to)` for a **direct edge**.
- `targets_from(from)` for direct outgoing neighbors.
- `transitions()`, `states()`, and `sources()` for enumeration.
- `contains_state(state)` and `is_terminal(state)`.
- Counts and immutable definition equality.

There is no public incoming-edge/predecessor query, rooted reachability, path existence/reconstruction, degree query, cycle query, component decomposition, or topological sort. `is_terminal(unknown)` returns true because unknown names have no outgoing targets; callers should check `contains_state` when that distinction matters. Statekit infers states from edge endpoints, so isolated states cannot be represented. Self-transitions are rejected and a built machine must have at least one transition.

**Verified implementation detail, not a public contract:** `Transitions` stores a `HashSet<Transition>`. `targets_from`, `contains_state`, and direct transition membership filter/scan this collection. `states()` and `sources()` form temporary `HashSet`s. Public docs leave enumeration order unspecified. BI-002 keeps storage details non-contractual.

### Existing benchmark evidence

The v0.4 benchmark baseline covers linear machines with 100, 1,000, 10,000, and 100,000 edges. It has separate construction/runtime/memory suites and documents scan-based queries and the lack of persistent indexes. It contains no BFS, reachability, path, cycle, SCC, or component benchmarks, and no branching, disconnected, or dense graph shapes. The benchmark source measures scan-heavy query behavior, not the analysis candidates in this report.

**No new timing measurements were run.** There is no analysis implementation to measure, and producing timings without implementing a candidate would imply evidence that does not exist. Section 8 proposes the smallest useful follow-up benchmark set.

## 2. Structural query taxonomy

Let V be the number of represented states and E the number of unique directed transitions. Isolated states are absent under the current Statekit model.

### A1. Local topology

| Question | Existing support | Derivable cost with current API | Notes |
|---|---|---|---|
| Direct edge? | `can_transition` / `validate_transition` | Already available; current implementation can scan O(E). | Direct adjacency only; not multi-step reachability. |
| Outgoing neighbors? | `targets_from` | Already available as a lazy filter; fully consuming it scans O(E). | Per-source call is not an indexed adjacency lookup today. |
| Source states? | `sources()` | Already available; currently constructs a temporary set. | A source has out-degree > 0. |
| Terminal / zero out-degree? | `is_terminal` | Already available; can be O(E) with current scan. | It describes no outgoing edges, not success/completion. Check known-state membership separately. |
| Incoming edges / predecessors? | No API. | One pass over `transitions()`; O(E), no extra storage if streamed. | A natural dual to `targets_from`; a predecessor collection costs O(in-degree). |
| In-/out-degree? | No explicit count API. | Scan transitions for one state: O(E); or derive from neighbors and count. | Repeated degree queries may justify a temporary degree map, not necessarily a persistent index. |
| Isolated states? | Not representable. | Cannot be recovered from `Machine`; consumer's source definition may retain them separately. | Adding support would change the current endpoint-derived model and interacts with the non-empty-edge invariant. |

### A2. Reachability and reverse reachability

- **Reachable from root r:** states with a directed path from r. This is root-dependent and does not imply any other root or global connectedness.
- **Unreachable from r:** represented states not in that reachable set. Reportable fact; consumer decides whether it is a warning/error.
- **Can u reach v?:** directed path existence. A zero-length path convention makes a known state reach itself; a positive-length cycle query is a separate question. The API would need to say which convention it uses.
- **Reverse reachability to target t:** states that can reach t; traverse predecessor edges from t.
- **Can reach a terminal:** existentially, a state can reach at least one zero-outdegree state. This does not say all executions eventually reach one.
- **Disconnected regions:** for directed graphs, clarify whether “disconnected” means weak connectivity (ignore direction) or strong connectivity (each vertex reaches every other). These are different facts.

Reachability should require a caller-selected root, never introduce a canonical initial state. Since `targets_from` returns empty both for an unknown state and a known target-only state, a public query must define unknown-root behavior explicitly.

### A3. Paths

- A direct transition is exactly one edge; a multi-step path is a sequence of adjacent directed edges.
- Path existence is a BFS/DFS query and can stop at the target.
- A shortest path by transition count is an unweighted BFS with a predecessor/parent map.
- One path can be reconstructed in O(path length) after the traversal; storing parents costs O(V) in the worst case.
- Bounded exploration is caller policy: bound depth, states, or time. A general “path budget” API would add policy to Statekit.
- All-path enumeration can be exponential in output size, even for modest graphs. Leave it to test/analysis callers or dedicated libraries.

Path retrieval is useful to tools that explain “how can this state be reached?” or generate an example trace, but no previous report demonstrated enough repeated demand to put it on the public API. A one-query `can_reach` and a shortest path also have different allocations and result obligations; do not add both by convenience.

### A4. Cycles and components

- **Cycle existence:** asks whether a directed path returns to a previously visited node. Statekit rejects self-loops, so any cycle in a valid machine includes at least two states.
- **Strongly connected components (SCCs):** maximal sets where every state reaches every other. An SCC with more than one state is cyclic under Statekit's no-self-loop invariant.
- **Condensation graph:** collapse each SCC to one node. The resulting directed graph is acyclic; edges describe movement between mutually reachable regions.
- **Terminal SCC:** a sink in the condensation graph. It has no outgoing edge to another SCC. A singleton terminal SCC in this model is a terminal state; a multi-state sink SCC is a closed region where cycling is structurally possible and no terminal state is reachable from within it.
- **Weak components:** connected components after ignoring edge direction. They answer a different question than directed reachability and SCCs.
- **Topological sort:** exists only for a DAG; it is not a general machine ordering because Statekit permits cycles.

SCCs answer more than “does a cycle exist?” They distinguish a cycle with an exit path from a closed cyclic region and provide a reusable condensation DAG for component-level queries. Still, a user may need only reachability, and Petgraph already exposes SCC and condensation algorithms. Complexity alone is not a reason to place SCCs in Statekit.

### A5. Structural classifications without business meaning

Topology can identify zero-outdegree states, zero-indegree states, states reachable from one or more selected roots, states that cannot reach a selected destination, cyclic SCCs, sink SCCs, and high-degree nodes. It can also identify a graph cut or a “bottleneck” only after a precise graph property is chosen (for example, articulation in the undirected projection or dominators relative to a root).

None of these facts means a state is success, failure, deadlock, process completion, or an error. “A route exists” is existential; “every run terminates” is a different temporal/property question. Statekit should report structural facts and leave names, severity, and policy to the consumer.

## 3. Algorithms and actual complexity

The standard complexities below assume adjacency lists and expected constant-time hash membership where needed. Statekit's current public iterators do not provide indexed adjacency, so a naïve use of those iterators can be slower.

| Algorithm/query | Meaning and direction | Adjacency-list complexity and memory | Current Statekit / implementation note | Likely fit |
|---|---|---|---|---|
| Full edge scan / degree count | Inspect edges; directed for in/out-degree. | O(E) time; O(1) extra if streaming, O(V) if accumulating all degrees. | Current direct scans; use for occasional local queries. | Already reasonable. |
| BFS from root | Visit directed-reachable states in layers. | O(V+E); O(V) queue/visited. | Repeated `targets_from(v)` can scan E for every visited v: O(VE). Build temporary adjacency from `transitions()` once for expected O(V+E). | Best first analysis primitive if demand emerges. |
| DFS from root | Directed reachability, postorder, or cycle checks. | O(V+E); O(V) visited/stack. | Same adjacency caveat. Iterative DFS avoids recursion-depth risk. | Consumer can implement; internal helper may share workspace. |
| Reverse BFS/DFS | Find all predecessors or states that can reach a target. | O(V+E) after reverse adjacency exists; O(V+E) to build it, O(V) to store visited/work queue. | Streaming one target's incoming edges costs O(E). Repeated queries favor a temporary/reusable reverse map only after evidence. | Possible primitive/API candidate. |
| Unweighted shortest path | Fewest transitions from root to target. | BFS O(V+E); O(V) parents/visited; path output O(L). | No edge weights exist; Dijkstra is unnecessary for this model. | Keep as consumer utility until demonstrated. |
| Directed cycle check | Is there any directed cycle? | DFS coloring O(V+E); O(V) state. | Because self-edges are forbidden, any cycle has >=2 states. Could derive from SCC size, but a dedicated DFS is simpler if only a boolean is needed. | Defer; consumers may have subgraphs/edge filters Statekit cannot express. |
| Tarjan SCC | Partition into mutually reachable sets in one DFS pass. | O(V+E) time; O(V) auxiliary state/output. | Does not require a persistent graph index, but implementation is more intricate; recursive versions risk deep stacks. | Defer; Petgraph already provides it. |
| Kosaraju SCC | DFS finishing order, then DFS on reversed edges. | O(V+E); O(V) work plus reverse adjacency O(V+E). | Easier to explain/iterate in two passes; reverse graph has temporary memory cost. | Defer; useful as library comparison/reference. |
| Weak components | Components when direction is ignored. | O(V+E) after undirected adjacency or by scanning/building both directions. | Not the same as state-machine directed reachability. | Usually external graph analysis. |
| Topological sort | Linear ordering respecting all directed edges; only for DAGs. | O(V+E); O(V) queue/degree state. | Most valid Statekit machines can be cyclic by design; a failure to sort is not a construction error. | Defer/external unless a consumer specifically analyzes DAG-only projections. |
| All simple paths | Enumerate every simple directed path. | Output can be exponential; memory at least output/path state. | Not bounded by machine size alone. | Exclude from core API. |

The 1972 Tarjan paper gives linear time/space SCC bounds; current Petgraph API docs document Tarjan/Kosaraju O(V+E), auxiliary O(V), path-connectivity O(V+E), and topological sort behavior. Petgraph's `has_path_connecting` also accepts reusable traversal workspace, illustrating that repeated-query scratch reuse can be handled without permanent graph indexes.

## 4. Implementation architectures

### A. Consumer traversal using current APIs

A caller can scan `transitions()` once into its own adjacency/reverse-adjacency/degree maps and run whichever traversal it needs. This keeps Statekit unchanged and preserves representation independence. It also duplicates small code and makes each consumer responsible for unknown roots, output order, and allocations. Calling `targets_from` inside BFS is concise but can be O(VE) under today's scan-based implementation.

**Best for:** one-off queries, test code, specialized VPE edge subsets, algorithms that require custom semantic filters.

### B. Statekit algorithms with temporary scratch structures

Statekit could build a local adjacency map from its transition iterator once per analysis call, then run BFS/DFS/SCC. Expected scan/build plus traversal cost is O(V+E); working memory is O(V+E), freed when the call returns. Construction and instance transition behavior need not change. Internal helpers could share root validation, traversal state, and adjacency conversion without exposing representation.

The tradeoff is that an algorithm advertised as “small and fast” may allocate substantially for each call. This can still be a better fit for infrequent config checks than paying memory for every Machine. Returning owned `Vec<&str>` results adds O(V) output memory; a lazy iterator over temporary traversal state would need a more complex public iterator type.

**Best for:** occasional root analysis, as a candidate after repeated consumer evidence and a benchmark.

### C. Persistent indexes or cached analysis

An eager adjacency/reverse-adjacency index can make repeated neighbor queries and traversals faster, with O(V+E) additional construction work and memory. A reachability cache is root-specific and can grow with queried roots; eager all-pairs reachability requires O(V²) bits/entries in common representations and does not fit a small general default. Lazy caches require synchronization/interior mutability even though the logical machine is immutable; that can add complexity and contention. Immutable topology makes cache results valid, but not necessarily deterministic in iteration order.

A caller-owned reusable analysis context is another middle ground: one local graph/visit workspace can serve many queries and be discarded when the consumer is done. That API itself needs justification and lifecycle/compatibility design.

**Best for:** only a demonstrated high-query workload where allocation or repeated scans dominates, supported by Statekit's existing runtime and memory benchmark discipline.

### D. Petgraph use or adapter

Options include:

1. Consumers convert `Machine::transitions()` to `petgraph::DiGraphMap` themselves.
2. Statekit builds a temporary Petgraph graph internally for selected analyses.
3. Statekit stores a Petgraph graph persistently.
4. Statekit implements one or two simple analyses directly.

The first option adds no dependency to Statekit and serves advanced consumers well. A temporary graph adds conversion allocations and dependency cost to Statekit, but can delegate algorithm maintenance. Persistent Petgraph storage duplicates or replaces Statekit's storage, ties internal representation/index choices to another crate, and likely adds memory. Direct simple BFS/predecessor logic has low dependency cost but Statekit then owns algorithm correctness and tests. Statekit's own `HashSet` means neither an in-house nor Petgraph algorithm automatically gets fast adjacency without building an adapter/index.

**Assessment:** Do not add Petgraph as a dependency for the currently supported query set. Consumers needing SCCs, path algorithms, dominators, or broad graph inspection can convert the logical edges. Reconsider only if Statekit adopts several analysis features and a trait-based adapter has a measured maintenance/performance advantage.

## 5. Consumer-driven queries and examples

### Example 1 — Configuration reachability

**Structural question:** From configured root `draft`, which represented states can be reached by directed edges?

```text
draft -> review -> approved
              \-> hold -> review
legacy -> archived
```

Expected topological observation: `{draft, review, approved, hold}` is reachable from `draft`; `legacy` and `archived` are not. The loop `review -> hold -> review` is permitted. Statekit must not reject the machine simply because a region is unreachable from the selected root. A deployment host may warn or reject according to its own policy.

Current Statekit offers `transitions()` and `contains_state`, so consumers can build a scratch adjacency map and BFS now. Example Rust (current APIs plus consumer-owned code):

```rust
use statekit::Machine;
use std::collections::{HashMap, HashSet, VecDeque};

fn reachable_from<'m>(machine: &'m Machine, root: &'m str)
    -> Option<HashSet<&'m str>>
{
    if !machine.contains_state(root) {
        return None;
    }

    let mut outgoing: HashMap<&str, Vec<&str>> = HashMap::new();
    for edge in machine.transitions() {
        outgoing.entry(edge.source()).or_default().push(edge.target());
    }

    let mut seen = HashSet::from([root]);
    let mut queue = VecDeque::from([root]);
    while let Some(state) = queue.pop_front() {
        if let Some(targets) = outgoing.get(state) {
            for &target in targets {
                if seen.insert(target) {
                    queue.push_back(target);
                }
            }
        }
    }
    Some(seen)
}
```

`None` for an unknown root and `Some({root})` for a known terminal root are consumer choices in this example, not Statekit contracts. This code builds adjacency once and is expected O(V+E) in time/memory; using `machine.targets_from(state)` for each popped state would rescan edges and can be O(VE). Return order is set-like/unspecified. The consumer still selects the root, interprets the output, and maps unknown roots to its diagnostic format.

### Example 2 — Cycles, SCCs, and components

**Structural question:** Which states can mutually reach each other, and which cyclic regions have exits or are closed?

```text
A <-> B -> C <-> D -> E
F -------------> G
```

SCCs are `{A,B}`, `{C,D}`, `{E}`, `{F}`, `{G}`. The condensation edges form `{A,B} -> {C,D} -> {E}` and `{F} -> {G}`. The first two SCCs are cyclic but have an exit route toward a terminal state. `{E}` and `{G}` are sink/terminal components. Both cyclic regions still permit indefinite cycling by repeatedly taking their internal cycle; existence of an exit does not prove eventual exit.

Statekit currently provides no SCC API. A consumer can convert the exposed edge set to Petgraph:

```rust
use petgraph::algo::kosaraju_scc;
use petgraph::graphmap::DiGraphMap;
use statekit::Machine;

fn components(machine: &Machine) -> Vec<Vec<&str>> {
    let mut graph = DiGraphMap::<&str, ()>::new();
    for edge in machine.transitions() {
        graph.add_edge(edge.source(), edge.target(), ());
    }
    kosaraju_scc(&graph)
}
```

The example identifies components, not their business labels. Consumers can classify SCC size > 1 as cyclic for Statekit machines because self-edges are forbidden, then inspect inter-component edges to identify sink SCCs. Petgraph reports component members/order according to its own API; consumers must not treat ordering as Statekit order. Conversion and SCC work are O(V+E) expected, with temporary graph and analysis memory O(V+E). A configuration editor may display cyclic regions; a process engine may issue a warning; a game may treat a cycle as an intentional loop. Statekit should not decide which interpretation applies.

### Example 3 — Model-based test exploration

**Structural question:** Can a selected test destination be reached, and what is one short edge sequence that gets there?

A test generator can use BFS from its initial state and parent links to produce one shortest-by-edge-count sequence. Existing `MachineInstance` can replay that sequence one direct move at a time. The generator, sequence shrinking, event mapping, execution, assertions, and coverage goals remain with Proptest or the host test framework.

With current APIs, consumer code can build adjacency from `transitions()` and run BFS. A hypothetical `reachable_states_from(root)` could supply candidate targets; a hypothetical `shortest_path(from, to)` would need to specify zero-length `from == to`, unknown endpoints, ordering/tie behavior, and result allocation. Neither is needed for basic reference testing, as investigation 2 found. Structural reachability also does not prove an application event or guard can make each edge execute.

**Performance:** Building a scratch adjacency map once makes a test exploration O(V+E). Doing one full reachability pass separately per target may repeat O(V+E); prefer one set of reachable states/one traversal when exploring several destinations. For small randomized machines, local test helpers may be simpler than public Statekit APIs.

### Example 4 — VPE compiler validation

**Verified VPE topology:** `CompiledProcess` contains indexed `Node`s and `Edge`s. Each edge has `action`, `target_idx`, `priority`, guards, and effects. VPE's source-law validator runs BFS from `initial_state` over all declared transition endpoints and warns on structurally unreachable states. It separately detects cycles among edges whose action is `AUTO_TICK`; that cycle result is specific to this edge filter.

**Potential Statekit contribution:** A caller-rooted reachability query could express the generic all-edge topology fact, after converting source/target names. It could be a cross-check or possibly replace VPE's BFS if VPE's projection constraints are acceptable. But VPE currently has indexed nodes and its own validator; a local BFS over `Vec<Node>` may be simpler and avoid building another machine.

**Why SCC on the full topology cannot replace AUTO_TICK validation:** `Draft --Submit--> Review` and `Review --AUTO_TICK--> Draft` form a cycle in the full graph, but not in the AUTO_TICK-only subgraph. Statekit edges have no labels, so VPE would have to build a filtered graph separately. Guards and priorities also mean structural cycle existence is not proof that a particular execution can loop. VPE's own semantics must interpret that observation.

**Compatibility constraints:** Statekit rejects self-edges and empty transition sets, infers states from edges, validates names more strictly, and collapses same-endpoint transitions. VPE allows terminal states and uses parallel semantic edges; its validator currently checks non-empty names but does not impose Statekit's self-edge/name rules. Some compiled laws therefore cannot be projected without explicit handling. No direct Statekit replacement is recommended on source evidence alone.

**Performance note:** VPE's validation source uses a `VecDeque`/`HashSet` reachability search and resolves each popped source by scanning `law.states`. The current control flow also invokes transient-state validation and AUTO_TICK structure validation inside the loop over states. This is a concrete place to measure/refactor in VPE itself before attributing cost to missing Statekit APIs. Statekit can help only if adopting it removes repeated work or supplies an independently useful contract; it cannot make a filtered semantic graph disappear.

## 6. Analysis dependency map

The smallest conceptual dependency chain is:

1. `transitions()` gives one complete edge scan.
2. One temporary outgoing adjacency map supports directed BFS/DFS; one reverse map supports predecessor and reverse-reachability queries.
3. Forward BFS supports reachable states, unreachable-from-root differences, path existence (stop early), and shortest path by edge count (record parents).
4. Reverse BFS from a target supports “which states can reach this target?” Reverse BFS from all terminal states supports “which states can reach some terminal?”—existentially.
5. SCC decomposition supports cyclic-region classification and condensation-DAG observations. A DAG algorithm on the condensation graph can identify sink components; it does not replace the SCC algorithm.
6. Degree maps and direct predecessor lists can be accumulated during the same edge scan if a consumer needs several local facts at once.

This argues for a reusable *internal traversal workspace* if multiple methods are eventually implemented, not necessarily for public `AnalysisContext`/graph types. Public conveniences such as `has_path`, `can_reach_terminal`, `toposort`, or `terminal_components` can be derived from the primitives, but each still needs a consumer justification and API semantics.

## 7. Benchmark and performance plan

### Current evidence

The v0.4 suite measures construction, transition queries, outgoing-target scans, state/source projections, instances, and memory at 100/1k/10k/100k transitions. The baseline graph is linear only. It documents that current target/state membership work may scan transitions, successful scan timings vary with randomized HashSet order, and no index is retained. These are useful controls but do not establish actual BFS/SCC costs.

### Smallest experiment that would inform an API decision

Compare three implementations without changing public Statekit storage:

1. **Naïve current-API traversal:** queue plus `targets_from` at every node.
2. **Temporary adjacency:** one pass over `transitions()` to build outgoing map, then BFS/DFS.
3. **External graph adapter:** convert to a Petgraph graph and call its corresponding analysis.

Include persistent indexing only as a fourth variant if repeated queries are a real expected workload. Preserve the existing `build_and_drop`, `can_transition`, and successful/failed instance-transition benchmarks as regression controls.

Use representative shapes rather than only edge count:

- Linear chain: long path/depth, V≈E+1.
- Broad branching tree: many shallow reachable states.
- Sparse disconnected regions: V/E similar to a tree but multiple roots/components.
- Large SCC plus exit tail and terminal sinks: cycle/component queries.
- Dense directed graph at smaller bounded sizes: high neighbor count and worst-case conversion cost.
- Many terminal states and repeated reverse queries: terminal-target analysis and reverse-index workload.

For each shape, record construction time, one analysis latency, repeated-query latency (for example 1, 10, 100 roots), temporary allocations/peak bytes, retained Machine bytes, and runtime transition lookup latency. Vary graph sizes using existing 100/1k/10k scales, with 100k only for sparse shapes; dense graphs need smaller limits. Use Criterion for timing and the existing allocator instrumentation or an equivalent isolated harness for memory. Record toolchain/device and do not call one machine's timing a universal guarantee.

**Decision gate:** A scratch workspace is acceptable if occasional analysis latency and peak memory are bounded at expected sizes and hot transition behavior stays unchanged. Persistent indexes need clear wins for repeated analysis after accounting for build time and retained memory. No numerical threshold can be set without consumer workload evidence.

## 8. Capability classification

### A. Already supported

- Direct-edge existence, outgoing neighbors, state/source/transition enumeration, terminality, counts, and immutable logical equality.
- Statekit can be projected from dynamic edge data and used as a structural oracle without API additions.
- A consumer can traverse `transitions()` once and derive arbitrary state-machine-specific analyses in its own representation.

### B. Consumer-implemented

- In-degree/out-degree and predecessor lists from a one-pass edge scan.
- Rooted BFS/DFS, reverse reachability, path existence, shortest unweighted path, and root-relative unreachable sets.
- SCC, weak components, condensation graph, topological sorting, and specialized edge-filter analyses through Petgraph.
- Test sequences and coverage using reachability/path outputs.

These remain easy to implement externally for one-off use and preserve Statekit's API stability.

### C. Small reusable improvements (conditional proposals)

1. **Incoming transition/predecessor iterator.** Scope is a direct dual to `targets_from`; can be a scan with O(E) query time and no persistent index. Value is clearer reverse inspection and repeated consumer use. It does not require changing construction rules.
2. **Caller-rooted reachable-state result.** Highest potential reusable analysis because it supports unreachable reporting and reachability questions while remaining domain-neutral. It needs explicit unknown-root semantics and allocation/result-order contracts. Internal implementation should scan transitions once into temporary adjacency rather than repeatedly use O(E) `targets_from` calls.

Neither is proven necessary. A pure documentation example may be enough until repeated use is shown.

### D. Future research

- SCC/cycle/condensation APIs after a consumer demonstrates a state-machine-specific need beyond Petgraph.
- Scratch-buffer reuse or a separate analysis context for repeated queries.
- Persistent outgoing/reverse indexes only after construction/query/memory benchmarks.
- Result types, deterministic output policy, and unknown-name behavior if analysis methods are actually proposed.
- Root-relative “all runs terminate” or liveness questions, which need precise quantifiers and should not be conflated with reachability.
- Support for isolated states and analysis of declared states not connected to edges; separate model change from algorithm APIs.

### E. Out of scope or better delegated

- All-path enumeration, weighted shortest path without edge costs, broad graph algorithms, visualization, graph storage/index types, and universal analysis workspaces without consumer evidence.
- Business labels and validity policies for reachable/unreachable/terminal/cyclic states.
- Event/action/guard-filter interpretation, simulation, migration, test generation, test coverage policy, compiler pass infrastructure.
- General graph API parity with Petgraph.

## 9. Ranked shortlist

| Rank | Candidate | Answers / evidence | Complexity and implementation | Recommendation |
|---:|---|---|---|---|
| 1 | **Rooted reachable-state analysis** | Config/pre-deploy reachability and VPE's existing root BFS; prior reports found repeated interest but no requirement. Primitive analysis. | O(V+E) expected with one temporary adjacency build; O(V+E) peak scratch/output. Naïve public-iterator BFS can be O(VE). Root, unknown-name, output/order, and allocation contract need definition. | **Prototype**, not immediately implement. If another independent consumer repeats the need and benchmarks are acceptable, consider a caller-rooted observation API. |
| 2 | **Incoming edges / predecessors** | Reverse inspection in configuration tools; supports reverse-analysis groundwork. Prior reports identified it. Local query. | O(E) scan and no additional memory; returned collection O(in-degree). Persistent reverse index costs O(E) extra storage. | **Document/consider small API** only if a consumer needs this directly more than once; no index initially. |
| 3 | **Path existence** | Explain whether a selected target can be reached; derived from reachability. | Early-exit BFS O(V+E), O(V); may use reusable scratch. | **Defer.** Can be consumer code; adds unknown/root and zero-length-path decisions. |
| 4 | **One shortest path** | Trace generation/explanation, model-based testing. | BFS plus parent map O(V+E), O(V); result O(L). Tie path depends on neighbor enumeration order unless normalized. | **Defer / external.** Useful but not shown as common Statekit responsibility. |
| 5 | **Cycle existence / SCCs / terminal SCCs** | Detect structurally cycling/closed regions; useful richer analysis. | Tarjan or Kosaraju O(V+E), O(V), plus result. More complex implementation and public result semantics. Petgraph provides these algorithms. | **Defer; prefer Petgraph conversion for consumers needing them.** Revisit after a concrete repeated use case. |
| 6 | **Weak components, topological sort, cut/bottleneck analyses** | Specialized structural views; topological order only for DAGs. | Most are O(V+E) but require a choice of directed/undirected interpretation and/or extra projections. | **Exclude from core for now.** Use dedicated graph library or consumer code. |
| 7 | **All paths / all-pairs reachability / eager caches** | Broad query families. | All paths can be exponential; all-pairs is typically O(V²) storage; caches add synchronization/lifecycle. | **Exclude/defer.** Poor fit without narrow measured workloads. |

The ranked list is a research order, not a roadmap commitment. The most reusable API candidate is rooted reachability, but even it should pass an evidence gate because consumers can implement it today.

## 10. Project Core and invariant review

| Existing intent/invariant | Implication for structural analysis |
|---|---|
| **I-001/I-005, INV-006 runtime/data-driven definition** | Root is supplied per query. No canonical start field should be added to `Machine`. Analyses work on runtime topology. |
| **I-002 small/fast, I-003 ease of use, D-011 performance tradeoff** | Benchmark analysis allocation and repeated cost. Do not add eager index/cache without memory/runtime evidence. Preserve hot transition-query performance. |
| **BI-001 building block** | Queries should be structural and limited. Do not add pathfinding policy, test/coverage tools, or graph-framework abstractions. |
| **BI-002 hidden representation** | Return logical state/edge references or consumer collections; do not expose HashSet order, internal node IDs, or adjacency layout. |
| **INV-001 immutability, INV-009 shared immutable definition** | Read-only analyses fit. Temporary scratch state should be per call/caller. Shared cached data is possible but requires measurement and synchronization policy. |
| **INV-002 self-transitions rejected** | Cycle classification may use SCC size > 1 for cycles. Do not weaken this invariant for algorithm convenience. |
| **INV-003 non-empty transition set** | Rooted queries only operate on built machines; a future empty/isolated-state model needs a separate invariant decision. Do not relax as a side effect. |
| **INV-004 duplicate edge identity, INV-005 case-sensitive names** | Analysis treats each directed endpoint pair once and preserves exact names. No normalization or edge multiplicity should appear in analysis results. |
| **INV-007 earliest detectable error, INV-008 typed errors** | Do not change builder validation timing. For query roots, define unknown-state behavior separately; a new analysis error must not turn observations into construction errors. |
| **INV-013 machine equality / INV-014 instance equality** | Analysis does not alter logical equality. Results may use sets because enumeration order is unspecified; do not bake result order into equality. |

Only proposed method behavior/result semantics would require spec/API documentation. No existing invariant requires reachability, acyclicity, connectedness, terminal reachability, or a terminal component. Do not make these `build()` failures.

## 11. Risks and unresolved design questions

- **Scan complexity:** Reusing `targets_from` for each BFS node is not linear on current storage. Public docs should not imply adjacency-list complexity unless implementation builds/indexes adjacency.
- **Existential vs universal meaning:** “Can reach a terminal” does not mean “will terminate”; a reachable cycle allows an infinite walk even when an exit exists.
- **Unknown root:** Should an analysis return an error, `Option`, empty result, or a typed outcome? `targets_from(unknown)` and `targets_from(known terminal)` currently both return empty.
- **Self-reachability:** Should `can_reach(x,x)` include the zero-edge path or require a non-empty cycle? Graph libraries commonly choose the former; Statekit's no-self-edge rule does not settle it.
- **Output order:** Existing order is unspecified. SCC membership and reachable sets are naturally unordered. Should APIs return vectors, iterators, sets, or a result wrapper? No stable order should be promised without a consumer need.
- **Scratch reuse:** Is one adjacency materialization per query acceptable, or do consumers need an explicit analysis workspace? Repeated-query benchmark should decide.
- **SCC behavior:** Does a consumer need SCC members, only a cycle flag, or terminal SCCs? These have different complexity/result API obligations and may be available externally.
- **Declared isolated states:** Configuration and VPE may declare states with no edges; Statekit cannot analyze them. Do these need to enter Statekit's domain, or should consumers retain them separately?
- **VPE-specific filtering:** Statekit has no event label or edge predicate query; a topology SCC cannot answer `AUTO_TICK` cycle questions without VPE building a filtered graph.
- **VPE duplication:** The current validator's reachability is a concrete comparison point, but its source-level lookup/control-flow costs may be improved locally. Measure which implementation is simpler before dependency integration.
- **Algorithm implementation choice:** Recursive Tarjan has compact state but call-stack depth risks; iterative Kosaraju requires reverse edges. No Statekit-specific benchmark currently selects one.

## 12. Suggested next investigations

1. **One-query reachability spike outside the repository:** Use a temporary adjacency map from `Machine::transitions()` and compare it with repeated `targets_from` calls. Validate unknown-root, terminal-root, cycle, and disconnected cases.
2. **Consumer recurrence check:** Ask whether VPE plus one independent config or IR consumer need the same root query. If only VPE needs it, its indexed `Node` representation may be simpler.
3. **Minimal benchmark suite:** Add no repository code yet; run the proposed three-way comparison on linear/tree/disconnected/sparse-SCC shapes and retain current v0.4 runtime query controls.
4. **API design note only after evidence:** Compare `reachable_states_from(root) -> Vec<&str>`, a result view, and `has_path(from,to)`; define unknown states, zero-length paths, allocation, ordering, and complexity honestly.
5. **SCC follow-up only if needed:** Compare Petgraph conversion, iterative Kosaraju, and iterative Tarjan for a real consumer query; decide whether Statekit would add unique value beyond wrapper/conversion convenience.
6. Keep other future tracks—portable representations, isolated-state support, diagnostics, migration/simulation, and semantic graph filters—separate unless a concrete algorithm depends on them.

## 13. Primary sources

### Statekit and VPE

- [Statekit README](https://github.com/whitephoenixsoft/statekit/blob/main/README.md), [Project Core](https://github.com/whitephoenixsoft/statekit/blob/main/docs/project-core.md), [specification](https://github.com/whitephoenixsoft/statekit/blob/main/docs/statekit-specification.md), [roadmap](https://github.com/whitephoenixsoft/statekit/blob/main/docs/roadmap.md)
- Statekit source: [machine queries](https://github.com/whitephoenixsoft/statekit/blob/main/src/machine.rs), [transition storage/query implementation](https://github.com/whitephoenixsoft/statekit/blob/main/src/transitions.rs), [builder](https://github.com/whitephoenixsoft/statekit/blob/main/src/builder.rs)
- Statekit performance source: [benchmark overview](https://github.com/whitephoenixsoft/statekit/blob/main/docs/benchmarks/README.md), [v0.4 baseline](https://github.com/whitephoenixsoft/statekit/blob/main/docs/benchmarks/baseline-v0.4.md), [query benchmarks](https://github.com/whitephoenixsoft/statekit/blob/main/benches/machine_queries.rs), [runtime benchmarks](https://github.com/whitephoenixsoft/statekit/blob/main/benches/machine_runtime.rs), [memory benchmarks](https://github.com/whitephoenixsoft/statekit/blob/main/benches/machine_memory.rs)
- VPE [compiled representation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compiled.rs), [compiler lowering](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compile.rs), and [compiler validation](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/validate.rs)
- Earlier investigations: [consumer research](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md), [reference-model research](sandbox:/workspace/scratch/3524c4ebe062/statekit-reference-model-research.md), [IR/interchange research](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-ir-interchange-research.md)

### Algorithms and libraries

- Petgraph [BFS visitor](https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html), [`has_path_connecting`](https://docs.rs/petgraph/latest/petgraph/algo/fn.has_path_connecting.html), [Tarjan SCC](https://docs.rs/petgraph/latest/petgraph/algo/scc/tarjan_scc/fn.tarjan_scc.html), [Kosaraju SCC](https://docs.rs/petgraph/latest/petgraph/algo/scc/kosaraju_scc/fn.kosaraju_scc.html), and [`toposort`](https://docs.rs/petgraph/latest/petgraph/algo/fn.toposort.html)
- Robert Tarjan, [“Depth-First Search and Linear Graph Algorithms,” SIAM Journal on Computing (1972)](https://doi.org/10.1137/0201010)
- Rust standard collection guidance for [`VecDeque`, `HashMap`, and `HashSet`](https://doc.rust-lang.org/stable/std/collections/)

---

## Conclusion

Statekit already exposes the edge facts needed to build consumer-owned analyses. The first analysis API worth evaluating is caller-rooted reachability, because it answers several repeated structural questions without imposing an initial state or validity policy. Incoming-edge inspection is a small secondary candidate. Both should be implemented without persistent indexes first if evidence warrants them, with algorithm complexity and allocation behavior measured against the current scan-based representation.

SCCs and richer path/component algorithms are valuable graph tools, but current consumer evidence does not justify adding them to Statekit. Petgraph is a better fit when consumers need a broad analysis suite. The current best answer is therefore: **no immediate API change; document and prototype reachability, then require consumer recurrence and benchmark evidence before expanding the surface.**
