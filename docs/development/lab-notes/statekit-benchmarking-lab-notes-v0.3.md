# Statekit Benchmarking Lab Notes for v0.3

These notes record the process used to design, validate, and refine the `statekit` benchmark suite.

They are intentionally different from the public benchmark report.

The public report should present stable methodology, reproducible measurements, supported conclusions, and clearly stated limitations.

These lab notes preserve the path taken to get there:

- initial assumptions
- benchmark design decisions
- surprising results
- invalid measurements
- corrections
- hypotheses
- lessons learned

The goal is to make the reasoning recoverable for future maintainers and useful for developers learning Rust benchmarking.

---

## 1. Benchmarking Goals

The benchmark work started with three questions:

1. How do Statekit's public operations scale as the number of transitions grows?
2. Which operations are sensitive to graph shape, lookup distribution, or internal storage layout?
3. Which future optimizations are justified by evidence rather than intuition?

The intent was not to optimize immediately.

The sequence was deliberately:

    establish correctness
        ↓
    establish benchmarks
        ↓
    understand current behavior
        ↓
    freeze a baseline
        ↓
    consider optimization later

This matters because changing implementation and benchmark methodology at the same time makes before-and-after comparisons much less trustworthy.

---

## 2. Benchmark Environment

The v0.3 baseline benchmarks were executed directly on a Google Pixel 10 Pro under Android using a native AArch64 Rust toolchain.

### 2.1 Hardware

| Component | Configuration |
|---|---|
| Device | Google Pixel 10 Pro |
| SoC | Google Tensor G5 |
| Architecture | AArch64 / ARMv8 |
| Logical CPUs | 8 |
| CPU frequency topology | 2 × 2.246 GHz, 5 × 3.052 GHz, 1 × 3.782 GHz |
| Memory | 15,949,248 KiB reported (~15.21 GiB) |
| Nominal memory class | 16 GB |
| Device codename | `blazer` |
| Board platform | `laguna` |

The CPU frequency topology was obtained from Linux sysfs using each CPU's `cpuinfo_max_freq`.

The kernel exposed logical CPUs `0-7`.

CPU topology observed by `/proc/cpuinfo`:

| CPUs | ARM CPU part | Maximum frequency |
|---|---:|---:|
| 0-1 | `0xd80` | 2.246 GHz |
| 2-6 | `0xd87` | 3.052 GHz |
| 7 | `0xd82` | 3.782 GHz |

The ARM part identifiers are recorded as reported by the device rather than being mapped to marketing core names.

### 2.2 Operating System

| Component | Configuration |
|---|---|
| Android version | Android 16 |
| Android API level | 36 |
| Kernel | Linux 6.6.102 |
| Kernel architecture | AArch64 |
| Execution environment | Termux |
| Build fingerprint | `google/blazer/blazer:16/CP1A.260505.005/15081906:user/release-keys` |

Kernel information reported by `uname`:

    Linux localhost 6.6.102-android15-8-g6eb5b2a8c46b-ab14739656-4k
    #1 SMP PREEMPT Mon Jan 19 02:06:09 UTC 2026 aarch64 Toybox

The `android15` string appearing in the kernel build identifier does not represent the Android userspace release. The device reports Android 16 through Android system properties.

### 2.3 Rust Toolchain

| Component | Version |
|---|---|
| rustc | 1.90.0 |
| Cargo | 1.90.0 |
| LLVM | 20.1.8 |
| Rust host | `aarch64-linux-android` |
| rustc commit | `1159e78c4747b02ef996e55082b704c09b970588` |
| rustc commit date | 2025-09-14 |

`rustc --version --verbose` reported:

    rustc 1.90.0 (1159e78c4 2025-09-14)
    binary: rustc
    commit-hash: 1159e78c4747b02ef996e55082b704c09b970588
    commit-date: 2025-09-14
    host: aarch64-linux-android
    release: 1.90.0
    LLVM version: 20.1.8

`cargo --version` reported:

    cargo 1.90.0 (840b83a10 2025-07-30)

Both Rust and Cargo were built from source tarballs.

### 2.4 Mobile Benchmarking Caveat

These measurements were performed on a mobile device rather than a dedicated benchmark host.

Mobile systems can dynamically change CPU frequency and scheduling behavior because of thermal state, battery state, background activity, and operating-system power-management decisions.

The measurements should therefore be interpreted primarily as a reproducible Statekit v0.3 baseline on this specific environment and as a basis for relative before-and-after comparisons, rather than as universal absolute performance figures.

---

## 3. Benchmark Graph

Most query benchmarks use a deterministic linear machine:

    state_0 -> state_1
    state_1 -> state_2
    state_2 -> state_3
    ...

For `n` transitions, this produces:

    n transitions
    n + 1 states
    n unique source states

Benchmark sizes were standardized at:

    100
    1,000
    10,000
    100,000

The linear topology was chosen because it is simple, deterministic at the logical level, and easy to reason about.

The internal transition collection is currently backed by a `HashSet`, so physical iteration order remains unspecified and randomized even though the logical graph is deterministic.

---

## 4. Query Benchmarks and Construction Benchmarks

Query benchmarks construct the machine outside Criterion's timed loop.

This prevents machine construction from contaminating query timing.

Conceptually:

    build fixture
        ↓
    begin timed iteration
        ↓
    query existing machine
        ↓
    end timed iteration

Construction benchmarks are different.

For those, transition input strings are generated before the timed loop, but `MachineBuilder` work happens inside the timed loop.

This isolates Statekit construction from unrelated benchmark-data generation such as integer formatting.

---

## 5. Pre-Generating Construction Inputs

Generating states inside the timed loop would measure:

    integer formatting
    String allocation for benchmark data
    Statekit validation
    StateName allocation
    Transition construction
    hashing
    HashSet insertion
    Machine construction

That would answer a broader application-level question, but not the intended Statekit construction question.

Instead, inputs are generated before timing.

The timed workload therefore asks:

> Given an already-parsed collection of valid transition strings, what does it cost Statekit to construct a machine from them?

This makes the construction benchmark substantially easier to interpret.

---

## 6. Why `black_box` Matters

Benchmark code is still optimized Rust code.

The compiler is allowed to remove work when it can prove that the result does not matter.

Criterion's `black_box` is used to make relevant values opaque enough that the optimizer cannot trivially eliminate the operation being measured.

However, an important lesson emerged:

> Applying `black_box` only to the final result does not necessarily force all intermediate work to occur.

This became particularly clear in the transition-iteration benchmark.

---

## 7. The `transitions().count()` Mistake

The first attempt to benchmark raw transition traversal used:

    machine.transitions().count()

The reported times were approximately:

    100 transitions       sub-nanosecond
    1,000 transitions     sub-nanosecond
    10,000 transitions    sub-nanosecond
    100,000 transitions   about 1 nanosecond

At first glance, this could look like extraordinary iterator performance.

Physically, however, it made no sense.

Traversing 100,000 heap-backed transition records cannot plausibly happen in around one nanosecond.

The explanation was iterator metadata and optimization.

The iterator can expose its exact remaining length, so `.count()` does not necessarily need to visit every item.

The benchmark was therefore measuring something close to retrieving a known length, not traversing the transition collection.

This was corrected by forcing each transition to be observed individually:

    machine
        .transitions()
        .for_each(|transition| {
            black_box(transition);
        });

The resulting timings became realistic.

This produced one of the most important lessons from the benchmarking work:

> An iterator expression that looks like traversal does not guarantee that traversal actually occurs.

More generally:

> A benchmark producing numbers does not mean it is measuring what its author intended.

---

## 8. Lazy Iterators Must Be Consumed

`targets_from(source)` returns a lazy iterator.

Benchmarking only:

    machine.targets_from(source)

would mostly measure iterator construction.

That says little about the cost of actually finding outgoing targets.

The benchmark therefore consumes the iterator with:

    machine.targets_from(source).count()

Unlike the raw transition iterator, this count cannot generally collapse to a stored length because the number of matching targets is not known without evaluating the filter.

This forces traversal and predicate evaluation.

---

## 9. Probe Design Matters

One of the strongest lessons was that lookup values are part of the benchmark workload.

An early missing-source probe looked substantially different from real generated states.

Rust string equality can reject mismatched strings quickly, including based on length.

As a result, `targets_from(missing)` initially appeared much faster than `targets_from(existing)` even though both traversed the entire transition collection.

The missing probe was changed to resemble real state names while still being guaranteed absent.

After controlling probe shape, existing and missing `targets_from` benchmarks converged closely.

This led to the rule:

> Control semantic meaning and physical input shape independently.

A missing query should be missing because of its semantic value, not because it is structurally trivial to reject.

---

## 10. Short-Circuit Evaluation Changes Workload Cost

Some Statekit queries use compound predicates.

Transition membership is effectively shaped like:

    transition.source() == source
        && transition.target() == target

If the source comparison is false, the target comparison is not evaluated.

By contrast, state membership is effectively shaped like:

    transition.source() == state
        || transition.target() == state

For a missing state, the source comparison is false and the target comparison must also be evaluated.

Two algorithms may therefore both be O(n) while performing different amounts of string comparison per element.

The benchmarks made this visible.

> Big-O describes scaling structure, not the complete per-element cost.

---

## 11. Existing and Missing Lookup Workloads

Benchmarks were intentionally divided into semantic categories.

For transition membership:

    existing transition
    missing transition

For state membership:

    existing source
    target-only existing state
    missing state

For source projection:

    existing source
    missing source

These cases serve different purposes.

Missing lookups are especially useful for complexity analysis because they guarantee a full scan.

Successful lookups may terminate early.

---

## 12. HashSet Iteration Order and Successful Lookups

The current internal transition store is a `HashSet`.

`HashSet` iteration order is unspecified and affected by randomized hashing.

That means a successful scan-based lookup does not have a stable logical position.

A transition that is logically "last" in the generated graph is not necessarily examined last.

This produced dramatic run-to-run differences.

For example, successful lookups at 100,000 transitions sometimes took tens of microseconds and on later runs only a few microseconds.

No algorithmic improvement had occurred.

The matching item had simply appeared much earlier in that run's `HashSet` traversal.

This led to a key classification:

> Successful lookup benchmarks are useful for representing real workloads, but they are poor complexity baselines when iteration order is randomized.

Missing lookups are cleaner because they are guaranteed to scan the entire collection.

---

## 13. Target-Only States Are Not Worst-Case Lookups

The linear benchmark graph has one state that is target-only:

    state_n

where `n` is the number of transitions.

It might be tempting to think this creates a worst-case state lookup because the state appears at the logical end of the graph.

That is not true with `HashSet` storage.

The transition containing that target can appear anywhere in iteration order.

Therefore:

    contains_state(target_only)

is still an early-exit successful workload.

It should not be assigned throughput metadata claiming all `n` transitions were examined.

---

## 14. Throughput Must Describe Guaranteed Work

Criterion throughput using:

    Throughput::Elements(transition_count as u64)

was retained only when the timed operation was guaranteed to process all transitions.

This is appropriate for workloads such as:

    can_transition(missing)
    targets_from(existing) with full iterator consumption
    targets_from(missing) with full iterator consumption
    contains_state(missing)
    raw transitions traversal
    sources()
    states()
    build_and_drop

It is not appropriate for successful early-exit lookups.

Input size is not the same thing as work performed.

> Throughput metadata should describe work that is guaranteed to occur, not merely the size of the input data structure.

---

## 15. `targets_from` Existing vs Missing

After controlling probe shape, the existing-source and missing-source `targets_from` results became very similar.

This was expected because consuming the iterator with `.count()` forces every transition to be examined in both cases.

Conceptually:

    for every transition:
        compare transition.source() to query source

The source may match zero times, once, or multiple times, but the entire transition collection still has to be visited.

This provided strong evidence that the operation is currently dominated by full traversal plus string comparison.

---

## 16. Raw Transition Traversal as a Reference

After fixing the `.count()` mistake, a raw traversal benchmark was retained as a controlled reference workload.

Its role is not to establish a perfect theoretical lower bound.

Each element is explicitly passed through `black_box`, which itself creates observable per-element work.

Instead, it answers:

> What does a forced traversal of the current transition collection cost under Criterion?

This makes it useful for comparing heavier operations.

    raw traversal
        ↓
    source filtering
        ↓
    state membership predicates
        ↓
    temporary set construction

---

## 17. Raw Traversal Is Not a Strict Lower Bound

At some sizes, `can_transition(missing)` measured slightly faster than the forced raw traversal benchmark.

That is not a contradiction.

The raw traversal benchmark performs:

    black_box(transition)

for every item.

A real query loop can be optimized as a tight predicate scan without the same forced observable operation.

Therefore raw traversal should be treated as:

    a controlled traversal reference

rather than:

    an absolute physical minimum

---

## 18. `sources()` and `states()` Are Different Kinds of Work

Unlike simple scans, `sources()` and `states()` construct temporary sets.

Their cost includes:

    traversal
    hashing
    temporary HashSet allocation
    insertion
    duplicate detection
    possible reallocation

For the linear graph, `sources()` processes one source endpoint per transition and produces `n` unique source states.

`states()` processes both endpoints and produces `n + 1` unique states.

The endpoint pattern for `states()` also creates repeated insert attempts:

    state_0   new
    state_1   new

    state_1   duplicate
    state_2   new

    state_2   duplicate
    state_3   new

    ...

This explains why `states()` performs more temporary-set work than `sources()`.

---

## 19. Repeated Runs Matter

Early `states()` measurements showed substantial variation between runs.

For example, the 10,000-transition case changed by nearly a factor of two between two benchmark runs.

That result was not discarded simply because one run better matched expectations.

Instead, it was treated as evidence that the benchmark had meaningful run-to-run variability.

Possible contributors include:

    randomized HashSet layout
    iteration order
    temporary HashSet insertion order
    allocator behavior
    cache state
    system noise

Those are hypotheses, not proven causes.

> Do not cherry-pick the benchmark run that best matches the hypothesis.

Repeated measurements are useful both for confirming stable behavior and for discovering unstable workloads.

---

## 20. Graph Topology Can Affect Allocation-Heavy Benchmarks

The current benchmark graph is linear.

For scan-only operations, topology may matter less than transition count and query distribution.

For operations that build temporary sets, topology can matter more because it changes:

    number of unique endpoints
    duplicate insertion rate
    hash-table growth behavior
    output cardinality

Future benchmark suites could compare:

    linear
    fan-out
    fan-in
    duplicate-heavy definitions

These were intentionally deferred.

The goal of the v0.3 baseline was to establish one trustworthy reference workload before expanding the benchmark matrix.

---

## 21. Criterion Change Reports Are Contextual

Criterion prints messages such as:

    Performance has improved.
    Performance has regressed.
    No change in performance detected.

These compare the current result with Criterion's stored previous measurement for the same benchmark identifier.

That does not automatically mean the Statekit implementation improved or regressed.

During benchmark development, several things changed:

    probe values
    workload definitions
    iterator-consumption methodology
    benchmark naming
    measurement settings

If the workload changes while retaining the same benchmark ID, Criterion is comparing two different experiments.

Randomized `HashSet` layout can also cause large changes in successful early-exit scans without any source-code change.

> Criterion change reports are only meaningful as regression evidence when the benchmark definition and environment are sufficiently stable.

This produced another rule:

> Treat benchmark definitions like APIs once they are used for regression tracking.

---

## 22. Outliers Are Not Automatically Problems

Criterion frequently reported outliers, sometimes at noticeable percentages.

Outliers alone do not invalidate a benchmark.

For extremely fast operations, tiny scheduling or hardware disturbances can be large relative to the operation itself.

For allocation-heavy benchmarks, allocator and system behavior can also introduce variation.

The correct response is not automatically to remove outliers or rerun until they disappear.

Instead, look for:

    stable central estimates
    coherent scaling
    repeatability
    suspicious discontinuities
    methodology mistakes

Outliers become important when they are part of a larger pattern suggesting unstable measurement.

---

## 23. Criterion Measurement-Time Warnings

Expensive workloads such as 100,000-transition construction sometimes produced warnings that Criterion could not complete the requested sample count within the default measurement window.

This is expected when one iteration itself is expensive.

The lesson was not simply to silence the warning.

Instead, expensive benchmark groups should eventually use a deliberate, documented policy such as:

    longer measurement time
    lower sample count
    possibly different sampling configuration

The benchmark configuration should be standardized rather than adjusted opportunistically benchmark by benchmark.

---

## 24. Construction Benchmark Design

Construction benchmarking introduced a different ownership problem from query benchmarks.

Every timed iteration creates a fresh builder and inserts all transitions.

The source inputs are borrowed from pre-generated `String`s.

Statekit then performs the ownership work required by its own data structures.

The timed work includes:

    MachineBuilder creation
    state validation
    StateName allocation and ownership
    Transition construction
    transition hashing
    HashSet insertion
    possible HashSet growth
    MachineBuilder::build
    resulting Machine destruction after the iteration

This is therefore an end-to-end build-and-drop workload.

---

## 25. Why `build_and_drop` Is a Better Name

The construction benchmark was initially called `build_linear`.

That mixed two independent concepts:

    operation
        build

    graph topology
        linear

It was renamed conceptually to:

    build_and_drop

while retaining a helper such as:

    build_linear_inputs()

for workload generation.

This separates the measured operation from the workload topology and leaves room for future benchmark organization such as:

    build_and_drop/linear
    build_and_drop/fan_out
    build_and_drop/fan_in

---

## 26. Construction Currently Includes Destruction

An important detail of the current benchmark is that the resulting `Machine` is dropped before the Criterion iteration completes.

So the measurement is not pure construction.

It is:

    construct
        +
    destroy

This is acceptable as long as the benchmark is named and documented accordingly.

A construction-only benchmark could theoretically defer destruction, but retaining many large Machines can introduce significant memory pressure and distort the workload.

For the v0.3 baseline, build-and-drop was retained as the simpler and more honest lifecycle measurement.

---

## 27. Construction Throughput

Construction uses:

    Throughput::Elements(transition_count)

because every timed iteration attempts exactly `n` transition additions.

The benchmark therefore reports transitions processed per second during the complete build-and-drop workload.

The measurements showed relatively similar throughput through moderate graph sizes and a noticeable decline at 100,000 transitions.

Possible contributors include:

    allocation
    HashSet resizing
    cache pressure
    allocator behavior
    larger working set
    destruction cost

These remain hypotheses until separately isolated.

---

## 28. Complexity and Hardware Effects

Several full-scan benchmarks showed approximately linear scaling at smaller and medium graph sizes, followed by reduced throughput at 100,000 transitions.

This does not imply that the algorithm changed complexity.

For an O(n) traversal:

    total time = n × cost per examined element

Big-O discusses how the number of operations grows.

It does not require the cost of each operation to remain constant across all working-set sizes.

As data grows, effects such as cache behavior, memory access patterns, allocator behavior, and hash-table layout can change the constant factor.

The benchmark report should therefore distinguish:

    algorithmic complexity

from:

    observed hardware-level throughput

---

## 29. Do Not Overstate Hardware Causes

It is tempting to explain every 100,000-transition slowdown as cache misses.

That is plausible but not proven by Criterion timing alone.

Without profiling or hardware performance counters, the defensible observation is:

> Per-element throughput decreases at larger graph sizes.

Potential explanations can be listed as hypotheses, but they should not be presented as established causes.

This distinction matters between lab notes and the public report.

The lab notes may preserve hypotheses.

The public benchmark report should state only what the measurements support directly.

---

## 30. Final Full-Suite Run

The final v0.3 benchmark run covered:

    can_transition(existing)
    can_transition(missing)

    targets_from(existing)
    targets_from(missing)

    sources()

    states()

    transitions()

    contains_state(existing_source)
    contains_state(target_only)
    contains_state(missing)

    build_and_drop()

The benchmark sizes remained:

    100
    1,000
    10,000
    100,000 transitions

This completed the initial API-performance baseline.

---

## 31. Final Full-Scan Measurements

### `can_transition(missing)`

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~63.1 ns | ~1.58 Gelem/s |
| 1,000 | ~723 ns | ~1.38 Gelem/s |
| 10,000 | ~7.25 µs | ~1.38 Gelem/s |
| 100,000 | ~149 µs | ~673 Melem/s |

The first three sizes show a particularly clear approximately linear relationship between input size and latency.

At 100,000 transitions, per-element throughput decreases.

### `targets_from(existing)`

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~179 ns | ~557 Melem/s |
| 1,000 | ~2.46 µs | ~407 Melem/s |
| 10,000 | ~25.7 µs | ~390 Melem/s |
| 100,000 | ~610 µs | ~164 Melem/s |

### `targets_from(missing)`

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~181 ns | ~551 Melem/s |
| 1,000 | ~2.85 µs | ~351 Melem/s |
| 10,000 | ~26.2 µs | ~382 Melem/s |
| 100,000 | ~617 µs | ~162 Melem/s |

Existing and realistic missing probes converge closely at larger sizes, supporting the interpretation that both workloads are dominated by full traversal and source comparison.

### `transitions()`

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~62.1 ns | ~1.61 Gelem/s |
| 1,000 | ~631 ns | ~1.58 Gelem/s |
| 10,000 | ~6.42 µs | ~1.56 Gelem/s |
| 100,000 | ~102 µs | ~982 Melem/s |

This benchmark forces every transition to be observed and acts as the controlled raw-traversal reference.

### `contains_state(missing)`

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~98.1 ns | ~1.02 Gelem/s |
| 1,000 | ~1.08 µs | ~930 Melem/s |
| 10,000 | ~10.7 µs | ~934 Melem/s |
| 100,000 | ~164 µs | ~609 Melem/s |

The first three sizes show a particularly clean scaling relationship.

The missing-state workload requires the full transition collection to be examined and both source and target membership comparisons to fail.

---

## 32. Projection Measurements

### `sources()`

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~3.65 µs | ~27.4 Melem/s |
| 1,000 | ~49.6 µs | ~20.2 Melem/s |
| 10,000 | ~517 µs | ~19.3 Melem/s |
| 100,000 | ~8.76 ms | ~11.4 Melem/s |

### `states()`

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~5.29 µs | ~18.9 Melem/s |
| 1,000 | ~70.9 µs | ~14.1 Melem/s |
| 10,000 | ~770 µs | ~13.0 Melem/s |
| 100,000 | ~20.9 ms | ~4.78 Melem/s |

The final run gives a coherent picture in which `states()` is more expensive than `sources()`, with the difference becoming increasingly significant at larger graph sizes.

Both operations are substantially more expensive than scan-only queries because they construct and populate temporary `HashSet`s.

---

## 33. Build-and-Drop Measurements

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~15.0 µs | ~6.66 Melem/s |
| 1,000 | ~194 µs | ~5.16 Melem/s |
| 10,000 | ~2.23 ms | ~4.48 Melem/s |
| 100,000 | ~55.1 ms | ~1.81 Melem/s |

The 100,000-transition case shows a substantial reduction in throughput compared with the smaller machines.

The benchmark includes both construction and destruction.

The exact cause of the larger-size throughput decline has not been isolated.

---

## 34. Successful Lookup Measurements

Successful lookup benchmarks remain useful as representative workloads, but they should not be interpreted as stable complexity curves.

### `can_transition(existing)`

The final run produced approximately:

| Transitions | Time |
|---:|---:|
| 100 | ~4.97 ns |
| 1,000 | ~1.95 µs |
| 10,000 | ~15.2 µs |
| 100,000 | ~2.52 µs |

The non-monotonic shape is expected from a successful scan over randomized `HashSet` iteration order.

### `contains_state(existing_source)`

| Transitions | Time |
|---:|---:|
| 100 | ~110 ns |
| 1,000 | ~28.6 ns |
| 10,000 | ~3.53 µs |
| 100,000 | ~1.68 µs |

### `contains_state(target_only)`

| Transitions | Time |
|---:|---:|
| 100 | ~71.9 ns |
| 1,000 | ~463 ns |
| 10,000 | ~2.33 µs |
| 100,000 | ~12.8 µs |

These successful workloads demonstrate why collection size alone does not determine work performed.

No full-scan throughput is assigned to these cases.

---

## 35. Final 100,000-Transition Cost Snapshot

The final run produced approximately:

    transitions()                ~0.102 ms
    can_transition(missing)      ~0.149 ms
    contains_state(missing)      ~0.164 ms
    targets_from(...)            ~0.61 ms
    sources()                    ~8.76 ms
    states()                     ~20.9 ms
    build_and_drop               ~55.1 ms

This gives a useful summary of the relative cost of the current API families.

It should not be interpreted as a universal performance guarantee.

Absolute timings depend on factors including:

    hardware
    compiler version
    Rust toolchain
    allocator
    system load
    thermal state
    CPU frequency policy
    hash-table layout
    benchmark configuration

Its primary value is comparative.

---

## 36. Performance Cost Ladder

The API operations form a useful conceptual hierarchy:

    transitions()
        controlled raw traversal reference

    can_transition(missing)
        traversal + transition membership predicate

    contains_state(missing)
        traversal + two-endpoint state predicate

    targets_from(...).count()
        traversal + source filtering across all transitions

    sources().count()
        traversal + hashing + temporary set + deduplication

    states().count()
        traversal + hashing both endpoints + larger temporary-set workload

    build_and_drop()
        validation + ownership + allocation + hashing + insertion
        + machine construction + destruction

This is more informative than isolated benchmark numbers because it explains how additional layers of work accumulate.

---

## 37. Stable and Position-Sensitive Benchmark Classes

The final suite naturally divides into two categories.

### Stronger Scaling Baselines

These workloads perform guaranteed full work and are therefore better suited to complexity and throughput analysis:

    can_transition(missing)
    targets_from(existing) with full iterator consumption
    targets_from(missing) with full iterator consumption
    contains_state(missing)
    transitions() forced traversal
    sources()
    states()
    build_and_drop()

### Position-Sensitive Workload Samples

These can terminate early and depend strongly on `HashSet` iteration order:

    can_transition(existing)
    contains_state(existing_source)
    contains_state(target_only)

These remain useful because successful lookup is a real application workload.

However, they should not be treated as stable complexity curves.

---

## 38. Current Optimization Implications

The benchmarks show that several query APIs currently scan the transition collection.

They also show that projection APIs using temporary `HashSet`s are much more expensive than scan-only operations.

This suggests possible future optimizations such as:

    source index
    target index
    state index
    cached source set
    cached state set

However, no optimization should be chosen solely because it appears obvious.

Indexes would introduce tradeoffs:

    higher construction cost
    additional memory
    more complex internal invariants
    potentially faster repeated queries

The existing immutable `Machine` design makes build-once/query-many indexing attractive, but benchmark evidence should drive the decision.

The v0.3 baseline exists specifically so those future tradeoffs can be measured.

---

## 39. Benchmarking Lessons

The main lessons from this work were:

1. Benchmark code must be validated just like production code.
2. A benchmark can execute successfully while measuring the wrong thing.
3. Lazy iterators must be deliberately consumed.
4. Exact-size iterator metadata can eliminate apparent traversal.
5. `black_box` is useful, but placing it only around a final result does not guarantee every expected intermediate operation occurs.
6. Input values are part of the benchmark workload.
7. Missing probes should resemble realistic values when comparison cost matters.
8. Short-circuit evaluation can materially change per-element work.
9. Successful `HashSet` scans are sensitive to randomized iteration position.
10. Missing queries are often better full-scan complexity benchmarks.
11. Throughput metadata should represent guaranteed work, not input size alone.
12. Big-O and real throughput describe different aspects of performance.
13. Allocation-heavy operations can behave differently from scan-only queries.
14. Graph topology can affect hashing and deduplication workloads.
15. Repeated runs can reveal unstable benchmarks.
16. Do not cherry-pick the run that matches expectations.
17. Criterion's "improved" and "regressed" messages require a stable benchmark definition to be meaningful.
18. Benchmark definitions should be treated as versioned experimental contracts.
19. Hardware explanations should remain hypotheses unless profiling supports them.
20. Establish a baseline before optimizing.

---

## 40. Documentation Strategy

The benchmarking documentation is intentionally split into three artifacts.

### Benchmarking Lab Notes

Purpose:

    preserve experiments
    preserve mistakes
    preserve reasoning
    preserve methodology lessons
    preserve hypotheses

This document is expected to evolve as new benchmark work is performed.

### Current Benchmark Report

Purpose:

    give users the current performance picture
    summarize methodology
    present current measurements
    explain supported conclusions
    identify limitations

This document can be updated as Statekit evolves.

### Versioned Baselines

Purpose:

    freeze historical benchmark snapshots

For example:

    baseline-v0.3.md
    baseline-v0.4.md

A frozen baseline should not be silently rewritten when later versions are benchmarked.

This provides an auditable performance history.
 