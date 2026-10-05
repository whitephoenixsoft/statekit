# Statekit v0.4 Benchmark Baseline

This document records the frozen performance and memory baseline for Statekit v0.4.

Its purpose is to provide a stable reference point for evaluating future implementation changes.

This document records only the final workloads, environment, measurements, interpretation, and known limitations used for the v0.4 baseline.

Future Statekit versions should be compared against this baseline using equivalent benchmark definitions wherever possible.

---

## 1. Purpose

The v0.4 benchmark baseline answers five primary questions:

1. How do Statekit's existing public graph operations behave as machine size increases?
2. How do the new stateful runtime operations behave as machine size increases?
3. Which operations currently dominate runtime cost?
4. What are the current memory characteristics of immutable machines and runtime instances?
5. What reference measurements should be preserved before changing Statekit's internal runtime representation or indexing strategy?

The baseline is intended primarily for relative comparison between Statekit versions.

It is not a performance or memory-usage guarantee.

---

## 2. Statekit Version

Baseline version:

    statekit 0.4.x

The benchmark captures the v0.4 architecture in which:

- transitions are immutable after machine construction
- transitions are stored internally in a `HashSet`
- several graph and runtime query APIs scan the transition collection
- `sources()` and `states()` construct temporary sets
- a `Machine` provides shared ownership of an immutable machine definition
- a `MachineInstance` stores independent mutable runtime state
- multiple instances can share the same immutable machine definition
- instance current state is represented using an owned state name
- no dedicated transition, source, target, state, or adjacency indexes are maintained

The internal storage and runtime representation are implementation details and may change in future releases.

This baseline exists so those changes can be measured objectively.

---

## 3. Benchmark Environment

All measurements were executed directly on a Google Pixel 10 Pro under Android using a native AArch64 Rust toolchain.

### 3.1 Hardware

| Component | Configuration |
| --- | --- |
| Device | Google Pixel 10 Pro |
| SoC | Google Tensor G5 |
| Architecture | AArch64 / ARMv8 |
| Logical CPUs | 8 |
| CPU frequency topology | 2 × 2.246 GHz, 5 × 3.052 GHz, 1 × 3.782 GHz |
| Memory | 15,949,248 KiB reported (~15.21 GiB) |
| Nominal memory class | 16 GB |
| Device codename | `blazer` |
| Board platform | `laguna` |

CPU topology reported by the device:

| CPUs | ARM CPU part | Maximum frequency |
| --- | ---: | ---: |
| 0-1 | `0xd80` | 2.246 GHz |
| 2-6 | `0xd87` | 3.052 GHz |
| 7 | `0xd82` | 3.782 GHz |

The ARM CPU part identifiers are recorded exactly as exposed by the device rather than being mapped to marketing core names.

The kernel exposed logical CPUs `0-7`.

### 3.2 Operating System

| Component | Configuration |
| --- | --- |
| Android version | Android 16 |
| Android API level | 36 |
| Kernel | Linux 6.6.102 |
| Kernel architecture | AArch64 |
| Execution environment | Termux |
| Build fingerprint | `google/blazer/blazer:16/CP1A.260505.005/15081906:user/release-keys` |

Kernel information reported by `uname`:

    Linux localhost 6.6.102-android15-8-g6eb5b2a8c46b-ab14739656-4k
    #1 SMP PREEMPT Mon Jan 19 02:06:09 UTC 2026 aarch64 Toybox

The `android15` component of the kernel build identifier does not indicate the Android userspace version.

Android system properties reported Android 16 and API level 36.

### 3.3 Rust Toolchain

| Component | Version |
| --- | --- |
| rustc | 1.90.0 |
| Cargo | 1.90.0 |
| LLVM | 20.1.8 |
| Rust host | `aarch64-linux-android` |
| rustc commit | `1159e78c4747b02ef996e55082b704c09b970588` |
| rustc commit date | 2025-09-14 |

Compiler details:

    rustc 1.90.0 (1159e78c4 2025-09-14)
    binary: rustc
    commit-hash: 1159e78c4747b02ef996e55082b704c09b970588
    commit-date: 2025-09-14
    host: aarch64-linux-android
    release: 1.90.0
    LLVM version: 20.1.8

Cargo:

    cargo 1.90.0 (840b83a10 2025-07-30)

Both Rust and Cargo were built from source tarballs.

---

## 4. Benchmark Framework

Runtime benchmarks use Criterion.

Benchmarks are compiled and executed using Rust's benchmark/release optimization profile.

Criterion performs repeated measurements, warmup, statistical estimation, and outlier detection.

Memory measurements use custom allocator instrumentation rather than Criterion timing.

The memory benchmark records requested allocation sizes and allocation events while controlled benchmark workloads execute.

Criterion's stored historical comparisons are not treated as part of this frozen baseline.

Several successful scan-based operations depend on randomized `HashSet` iteration order, and stored Criterion comparisons may therefore report large apparent improvements or regressions even when the underlying algorithm has not changed.

Only the final measurements recorded in this document constitute the v0.4 baseline.

---

## 5. Benchmark Graph

The benchmark suite uses a deterministic linear graph:

    state_0 -> state_1
    state_1 -> state_2
    state_2 -> state_3
    ...
    state_n-1 -> state_n

For `n` transitions, the graph contains:

    n transitions
    n + 1 states
    n source states
    1 target-only terminal state

The tested transition counts are:

    100
    1,000
    10,000
    100,000

The graph is logically deterministic.

However, transitions are stored internally in a `HashSet`, so physical iteration order is unspecified and may differ between program executions.

---

## 6. Runtime Benchmark Methodology

Graph-query benchmarks construct the `Machine` before entering Criterion's timed iteration.

Therefore query measurements exclude machine construction.

Construction inputs are also generated outside the timed iteration.

The `build_and_drop` benchmark measures Statekit construction from already-created input strings.

The benchmark includes:

- builder creation
- state validation
- ownership of state names
- transition construction
- hashing
- `HashSet` insertion and growth
- final machine construction
- destruction of the resulting machine

Benchmark fixture generation such as integer formatting is excluded.

### 6.1 Instance Construction

`Machine::instance()` benchmarks construct the machine outside the timed iteration.

The timed operation therefore measures creation of an instance from an already-existing machine definition.

Three probes are used:

- an existing source state
- the target-only terminal state
- a missing state

### 6.2 Runtime Transition Queries

`MachineInstance::can_transition_to()` benchmarks construct both the machine and instance before the timed iteration.

Three probes are used:

- an allowed transition
- a disallowed transition to an existing state
- a transition to a missing state

### 6.3 Runtime State Transitions

`MachineInstance::transition_to()` mutates instance state on success.

Reusing one instance across Criterion iterations would therefore change the workload after the first successful transition.

The benchmark uses Criterion batched iteration so that setup creates a fresh instance outside the measured routine and each timed operation receives a fresh instance in the same initial state.

Three operations are measured:

- an allowed transition
- a disallowed transition to an existing state
- a transition to a missing state

The failed operations are measured as complete public API operations, including construction of the returned error value.

### 6.4 Terminal-State Queries

`MachineInstance::is_terminal()` does not mutate the instance.

The machine and instance are therefore constructed outside the timed iteration.

Two states are measured:

- a source state with an outgoing transition
- the target-only terminal state with no outgoing transition

A missing-state case is not measured because a `MachineInstance` cannot be constructed in a state that does not belong to its machine.

---

## 7. Benchmark Categories

The benchmark suite contains two important timing categories.

### 7.1 Guaranteed Full-Work Benchmarks

These operations are guaranteed to inspect or process the complete transition collection for the benchmark workload.

They provide the strongest evidence for scaling behavior.

They include:

- `can_transition(missing)`
- `targets_from(existing)` with complete iterator consumption
- `targets_from(missing)` with complete iterator consumption
- `contains_state(missing)`
- forced `transitions()` traversal
- `sources()`
- `states()`
- `build_and_drop`
- `Machine::instance(missing)`
- `MachineInstance::can_transition_to(disallowed_existing)`
- `MachineInstance::can_transition_to(missing)`
- `MachineInstance::transition_to(disallowed_existing)`
- `MachineInstance::transition_to(missing)`
- `MachineInstance::is_terminal()` for the terminal state

Criterion throughput is reported for graph-query workloads where the processed transition count is well defined.

Runtime-instance operations are reported primarily as operation latency because transition count is an independent property of the machine rather than the semantic number of items processed by the public operation.

### 7.2 Position-Sensitive Successful Lookups

Several operations may terminate as soon as a matching transition or state is encountered.

They include:

- `can_transition(existing)`
- `contains_state(existing_source)`
- `contains_state(target_only)`
- `Machine::instance(existing_source)`
- `Machine::instance(terminal)`
- `MachineInstance::can_transition_to(allowed)`
- `MachineInstance::transition_to(allowed)`
- `MachineInstance::is_terminal()` for a source state

Because transition iteration order is controlled by a randomized `HashSet`, these measurements can vary substantially between executions.

They are retained as representative successful-operation workloads but are not treated as stable scaling curves.

---

## 8. Raw Transition Traversal

The `transitions()` benchmark forces each transition to be individually observed.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~64.9 ns | ~1.54 Gelem/s |
| 1,000 | ~630 ns | ~1.59 Gelem/s |
| 10,000 | ~6.19 µs | ~1.61 Gelem/s |
| 100,000 | ~98.6 µs | ~1.01 Gelem/s |

This benchmark provides a controlled traversal reference.

It should not be interpreted as a strict theoretical lower bound because each visited transition is deliberately made observable to prevent optimization from removing the traversal.

---

## 9. Transition Membership

### 9.1 Missing Transition

`can_transition(missing)` uses a transition guaranteed not to exist.

This forces the entire transition collection to be scanned.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~64.9 ns | ~1.54 Gelem/s |
| 1,000 | ~696 ns | ~1.44 Gelem/s |
| 10,000 | ~7.21 µs | ~1.39 Gelem/s |
| 100,000 | ~146 µs | ~687 Melem/s |

The benchmark continues to provide strong evidence of the current linear scan behavior.

### 9.2 Existing Transition

| Transitions | Time |
| ---: | ---: |
| 100 | ~33.1 ns |
| 1,000 | ~1.55 µs |
| 10,000 | ~18.9 µs |
| 100,000 | ~26.6 µs |

These results are intentionally not interpreted as a scaling curve.

A successful lookup terminates when the matching transition is encountered.

Because `HashSet` iteration order is randomized, the number of transitions inspected can vary substantially between executions.

---

## 10. Outgoing Target Queries

`targets_from(source)` returns a lazy iterator.

The benchmark consumes the iterator completely using `.count()`, ensuring that every transition is examined.

### 10.1 Existing Source

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~183 ns | ~547 Melem/s |
| 1,000 | ~2.97 µs | ~337 Melem/s |
| 10,000 | ~26.6 µs | ~375 Melem/s |
| 100,000 | ~645 µs | ~155 Melem/s |

### 10.2 Missing Source

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~186 ns | ~538 Melem/s |
| 1,000 | ~2.99 µs | ~334 Melem/s |
| 10,000 | ~26.5 µs | ~378 Melem/s |
| 100,000 | ~659 µs | ~152 Melem/s |

Existing and missing probes produce similar costs at larger sizes.

This supports the interpretation that the operation is dominated by traversal and source filtering rather than whether a match is ultimately found.

---

## 11. State Membership

### 11.1 Missing State

The missing-state workload guarantees that neither endpoint of any transition matches the probe.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~135 ns | ~742 Melem/s |
| 1,000 | ~1.24 µs | ~808 Melem/s |
| 10,000 | ~12.0 µs | ~835 Melem/s |
| 100,000 | ~187 µs | ~535 Melem/s |

The workload performs a full scan.

For each nonmatching transition, both source and target membership comparisons must fail.

### 11.2 Existing Source

| Transitions | Time |
| ---: | ---: |
| 100 | ~11.2 ns |
| 1,000 | ~661 ns |
| 10,000 | ~9.71 µs |
| 100,000 | ~136 µs |

### 11.3 Target-Only Existing State

| Transitions | Time |
| ---: | ---: |
| 100 | ~69.4 ns |
| 1,000 | ~542 ns |
| 10,000 | ~7.61 µs |
| 100,000 | ~46.1 µs |

Both successful workloads are position-sensitive.

A target-only state is logically the final state in the linear graph, but the transition containing it may appear anywhere in `HashSet` iteration order.

These measurements therefore represent successful-query samples rather than worst-case scans.

---

## 12. Source Projection

`sources()` scans transitions and constructs a temporary set of unique source states.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~3.85 µs | ~26.0 Melem/s |
| 1,000 | ~52.2 µs | ~19.2 Melem/s |
| 10,000 | ~499 µs | ~20.1 Melem/s |
| 100,000 | ~8.52 ms | ~11.7 Melem/s |

The operation includes:

- transition traversal
- hashing
- temporary `HashSet` allocation
- insertion
- deduplication
- possible table growth

Its cost remains substantially greater than scan-only operations.

---

## 13. State Projection

`states()` scans both transition endpoints and constructs a temporary set of unique state names.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~5.63 µs | ~17.8 Melem/s |
| 1,000 | ~78.7 µs | ~12.7 Melem/s |
| 10,000 | ~844 µs | ~11.8 Melem/s |
| 100,000 | ~25.9 ms | ~3.86 Melem/s |

For the linear benchmark graph, adjacent transitions share an endpoint.

The operation therefore performs both successful insertions and duplicate-detection work while processing two endpoint occurrences per transition.

`states()` remains consistently more expensive than `sources()` in the frozen run.

---

## 14. Machine Build and Drop

The construction benchmark creates a complete machine from pre-generated transition strings and allows the resulting machine to be destroyed within the timed iteration.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~19.8 µs | ~5.06 Melem/s |
| 1,000 | ~217 µs | ~4.61 Melem/s |
| 10,000 | ~2.62 ms | ~3.82 Melem/s |
| 100,000 | ~62.1 ms | ~1.61 Melem/s |

This measurement includes both construction and destruction.

The 100,000-transition workload shows a substantial decrease in per-transition throughput compared with smaller machines.

The benchmark does not isolate the cause of that decrease.

Potential contributors such as allocation behavior, hash-table growth, memory hierarchy effects, or destruction cost require separate profiling before they can be stated as causes.

---

## 15. Machine Instance Construction

`Machine::instance()` validates the requested initial state and creates an independently mutable runtime instance sharing the immutable machine definition.

### 15.1 Existing Source

| Transitions | Time |
| ---: | ---: |
| 100 | ~139 ns |
| 1,000 | ~1.05 µs |
| 10,000 | ~7.51 µs |
| 100,000 | ~57.7 µs |

### 15.2 Terminal State

| Transitions | Time |
| ---: | ---: |
| 100 | ~108 ns |
| 1,000 | ~720 ns |
| 10,000 | ~486 ns |
| 100,000 | ~168 µs |

### 15.3 Missing State

| Transitions | Time |
| ---: | ---: |
| 100 | ~153 ns |
| 1,000 | ~1.22 µs |
| 10,000 | ~14.4 µs |
| 100,000 | ~178 µs |

The missing-state workload provides the cleanest scaling signal because validation must establish that the state does not occur in the machine.

Successful initial-state validation is position-sensitive because a matching transition endpoint may be encountered at different positions in `HashSet` iteration order.

The terminal state is valid even though it has no outgoing transitions.

---

## 16. Instance Transition Queries

`MachineInstance::can_transition_to()` asks whether the current instance state may transition to a specified target without mutating the instance.

### 16.1 Allowed Transition

| Transitions | Time |
| ---: | ---: |
| 100 | ~68.4 ns |
| 1,000 | ~963 ns |
| 10,000 | ~2.49 µs |
| 100,000 | ~25.6 µs |

The successful case may terminate as soon as the matching edge is encountered and is therefore position-sensitive.

### 16.2 Disallowed Existing Target

A valid machine state is used as the target, but the exact transition from the current state does not exist.

The workload therefore tests exact transition membership rather than general target-state membership.

This case requires an exhaustive transition-membership search in the benchmark graph.

### 16.3 Missing Target

The target does not exist in the machine.

Like the disallowed-existing case, the exact transition is absent and the transition collection must be exhausted.

The similarity between the two negative workloads reflects Statekit's exact-match transition semantics: the query asks whether the specific directed edge exists.

---

## 17. Instance State Transitions

`MachineInstance::transition_to()` validates a transition and mutates the current state only when that transition is allowed.

A fresh instance is supplied to each timed successful operation so every iteration measures the same transition from the same initial state.

### 17.1 Allowed Transition

The operation measures both transition validation and the successful runtime-state update.

Successful lookup remains position-sensitive because validation may terminate when the matching edge is encountered.

### 17.2 Disallowed Existing Target

The requested target exists elsewhere in the machine, but the exact edge from the current state does not.

The operation returns an error and leaves instance state unchanged.

### 17.3 Missing Target

The target does not exist in the machine.

| Transitions | Time |
| ---: | ---: |
| 100 | ~257 ns |
| 1,000 | ~987 ns |
| 10,000 | ~9.27 µs |
| 100,000 | ~240 µs |

Failed `transition_to()` measurements represent the complete public operation, including production of the returned error value.

---

## 18. Terminal-State Queries

`MachineInstance::is_terminal()` reports whether the instance's current state has no outgoing transitions.

### 18.1 Source State

A source state has an outgoing transition.

The query may terminate when an outgoing transition is encountered and is therefore position-sensitive.

### 18.2 Terminal State

The benchmark's final target-only state has no outgoing transitions.

Determining that it is terminal requires establishing that no transition uses the state as a source.

The terminal workload therefore provides the stronger scaling baseline for the current implementation.

At 100,000 transitions, the final frozen run measured the terminal-state query at approximately 238 µs.

This workload provides a direct future comparison point for any source or adjacency indexing strategy.

---

## 19. Memory Measurement Methodology

Memory measurements use allocator instrumentation around controlled benchmark workloads.

The benchmark records:

- retained bytes
- cumulative allocated bytes
- allocation count
- peak additional live bytes

These metrics describe allocator requests made by the measured workload.

They should not be interpreted as exact process memory consumption or resident-set size.

Allocator bookkeeping, allocator implementation overhead, operating-system memory accounting, fragmentation, and unrelated process memory are outside the measurement.

### 19.1 Retained Bytes

Retained bytes represent measured heap bytes still live after the workload completes while the resulting machine or instances remain alive.

This metric is particularly useful for estimating the persistent heap cost of the resulting Statekit data structures.

### 19.2 Cumulative Allocated Bytes

Cumulative allocated bytes represent the total number of requested allocation bytes during the workload.

Memory that is allocated and later freed still contributes to this metric.

It therefore reflects allocation traffic rather than final retained memory.

### 19.3 Allocation Count

Allocation count records allocation and successful reallocation events observed during the measured workload.

For machine construction, the benchmark workload includes temporary formatted state-name strings used to exercise the public builder API.

The resulting allocation count must therefore not be interpreted as the number of allocations performed solely by Statekit's internal machine representation.

### 19.4 Peak Additional Live Bytes

Peak additional live bytes record the greatest increase in measured live heap bytes above the workload's starting live-byte level.

This provides a view of temporary construction pressure in addition to final retained memory.

The allocator benchmark is controlled and single-threaded while peak measurements are collected.

---

## 20. Machine Memory

### 20.1 Retained Memory

| Transitions | Retained bytes | Bytes / transition |
| ---: | ---: | ---: |
| 100 | 7,926 | 79.26 |
| 1,000 | 118,207 | 118.21 |
| 10,000 | 1,000,672 | 100.07 |
| 100,000 | 8,600,385 | 86.00 |

At 100,000 transitions, the finished machine retained approximately 8.60 MB of measured heap allocations.

The per-transition value is not constant across machine sizes because the representation includes collection capacity, string storage, and other allocation behavior that does not scale as a fixed exact number of bytes per transition.

### 20.2 Cumulative Allocated Memory

| Transitions | Allocated bytes | Bytes / transition |
| ---: | ---: | ---: |
| 100 | 16,442 | 164.42 |
| 1,000 | 242,435 | 242.44 |
| 10,000 | 2,043,388 | 204.34 |
| 100,000 | 17,422,837 | 174.23 |

At 100,000 transitions, the construction workload requested approximately 17.42 MB cumulatively.

This is allocation traffic, not simultaneous live memory.

### 20.3 Allocation Count

| Transitions | Allocation events | Events / transition |
| ---: | ---: | ---: |
| 100 | 407 | 4.07 |
| 1,000 | 4,011 | 4.01 |
| 10,000 | 40,014 | 4.00 |
| 100,000 | 400,017 | 4.00 |

The benchmark approaches approximately four allocation or successful reallocation events per transition.

However, two formatted state-name strings are created per transition inside the measured public-API construction workload.

The result therefore describes the complete benchmark construction workload and must not be interpreted as evidence that Statekit's internal representation itself requires exactly four allocations per transition.

### 20.4 Peak Construction Memory

| Transitions | Peak additional bytes | Bytes / transition |
| ---: | ---: | ---: |
| 100 | 10,317 | 103.17 |
| 1,000 | 166,472 | 166.47 |
| 10,000 | 1,345,403 | 134.54 |
| 100,000 | 10,873,182 | 108.73 |

At 100,000 transitions, peak additional measured live memory during construction was approximately 10.87 MB.

This is moderately above the approximately 8.60 MB retained by the finished machine, indicating temporary construction memory without an extreme peak relative to the retained representation.

---

## 21. Machine Instance Memory

On the benchmark platform:

    size_of::<MachineInstance>() = 32 bytes

The memory benchmark initializes each instance to:

    state_0

The UTF-8 state name contains seven bytes.

The benchmark collection is preallocated to the exact instance count to minimize geometric collection-growth noise.

### 21.1 Retained Memory

For every tested machine size, the results were:

| Instances | Retained bytes | Bytes / instance |
| ---: | ---: | ---: |
| 1 | 39 | 39.00 |
| 100 | 3,900 | 39.00 |
| 1,000 | 39,000 | 39.00 |

The same values were observed for machines containing 100, 1,000, 10,000, and 100,000 transitions.

For this benchmark workload, each additional instance therefore added exactly 39 measured heap bytes:

    32 bytes of preallocated collection storage
    + 7 bytes for the owned "state_0" state-name buffer
    = 39 bytes

This should not be generalized as a statement that every \`MachineInstance\` always uses 39 bytes.

The owned state-name allocation depends on state-name length, and collection/storage context also matters.

### 21.2 Cumulative Allocated Memory

Cumulative allocated memory was identical to retained memory for this workload:

| Instances | Allocated bytes | Bytes / instance |
| ---: | ---: | ---: |
| 1 | 39 | 39.00 |
| 100 | 3,900 | 39.00 |
| 1,000 | 39,000 | 39.00 |

No additional temporary allocation traffic was observed beyond the memory retained by the instance collection and current-state strings.

### 21.3 Allocation Count

For every tested machine size:

| Instances | Allocation events | Events / instance |
| ---: | ---: | ---: |
| 1 | 2 | 2.00 |
| 100 | 101 | 1.01 |
| 1,000 | 1,001 | 1.00 |

The pattern is consistent with:

- one allocation for the preallocated instance collection
- one owned current-state string allocation per instance

Sharing the immutable machine definition does not allocate another machine definition for each instance.

### 21.4 Peak Instance Construction Memory

Peak additional live memory was also identical to retained memory:

| Instances | Peak additional bytes | Bytes / instance |
| ---: | ---: | ---: |
| 1 | 39 | 39.00 |
| 100 | 3,900 | 39.00 |
| 1,000 | 39,000 | 39.00 |

No temporary peak above the final retained instance memory was observed for this workload.

### 21.5 Independence from Machine Size

The incremental instance measurements were identical across machines containing:

    100
    1,000
    10,000
    100,000

transitions.

For the tested implementation and workload, incremental instance memory therefore did not increase with machine transition count.

This supports the intended shared-definition architecture: each instance carries its own runtime state while referring to an already-existing immutable machine definition.

---

## 22. Runtime Cost at 100,000 Transitions

The largest benchmark size exposes the main current runtime-access costs.

Representative exhaustive or negative operations fall into a similar latency region:

    can_transition(missing)              ~0.146 ms
    contains_state(missing)              ~0.187 ms
    Machine::instance(missing)           ~0.178 ms
    transition_to(missing)               ~0.240 ms
    is_terminal(terminal)                ~0.238 ms

These operations have different public semantics, but each currently depends on determining absence through transition-set traversal.

The clustering does not imply identical implementation work.

For example, failed state transitions additionally produce an error value.

It does show that graph-access representation is a significant component of runtime cost.

Projection and construction operations remain substantially more expensive:

    targets_from(...).count()            ~0.65 ms
    sources().count()                    ~8.52 ms
    states().count()                     ~25.9 ms
    build_and_drop                       ~62.1 ms

The broad conceptual cost ladder remains:

    raw transition traversal
        ↓
    scan-based membership/runtime predicates
        ↓
    full traversal with filtering
        ↓
    temporary unique-source construction
        ↓
    temporary unique-state construction
        ↓
    complete machine construction and destruction

---

## 23. Scaling Interpretation

Several current Statekit operations are implemented using transition scans.

Their algorithmic structure is therefore linear in the number of stored transitions for workloads that require exhausting the collection.

The v0.4 runtime API exposes this characteristic in additional places.

In particular:

- validating a missing initial instance state requires proving state absence
- rejecting an absent exact transition requires proving edge absence
- determining that a state is terminal requires proving absence of outgoing transitions

The benchmark measurements support this structure.

Observed latency does not, however, scale with a perfectly constant cost per transition.

At larger working-set sizes, throughput changes for several workloads.

The measurements establish the observed cost.

They do not establish the hardware-level cause.

Possible cache, allocator, memory-access, scheduler, thermal, or hash-layout effects remain hypotheses unless confirmed through profiling.

Algorithmic complexity and observed machine throughput should therefore be treated as separate concepts.

---

## 24. Position-Sensitive Lookup Variability

Successful scan-based lookup results vary significantly because the internal \`HashSet\` does not preserve logical insertion order.

This affects both the original immutable query API and the v0.4 runtime API.

For example, an allowed transition may be found near the beginning of iteration in one process and much later in another.

Similarly, an existing initial state or a source state's outgoing transition may be encountered at different physical positions.

The v0.4 baseline therefore uses guaranteed-negative and terminal workloads as the strongest evidence for scan complexity.

Successful lookup results are retained because they represent real API usage, but they should not be used alone to evaluate regression or scaling.

Criterion's historical “improved” or “regressed” messages for these workloads should not be interpreted as controlled version-to-version conclusions.

---

## 25. Mobile Benchmark Environment Limitations

These measurements were collected on a mobile device.

Android may dynamically alter:

- CPU frequency
- CPU scheduling
- thermal policy
- background activity
- power-management behavior

The benchmark host is therefore not equivalent to a dedicated, frequency-controlled benchmarking workstation.

Absolute timings should not be generalized to other hardware.

The baseline is most useful for:

- understanding relative operation costs
- detecting large performance changes
- comparing future Statekit versions under equivalent conditions
- evaluating proposed internal optimizations on the same device

Small percentage differences should be interpreted conservatively.

Allocator measurements avoid several problems associated with process-level resident-memory measurements, but they still describe requested heap allocations rather than exact physical memory consumption.

---

## 26. Baseline Conclusions

The v0.4 benchmark suite supports the following conclusions.

### 26.1 Scan-Based Graph Access Remains the Main Runtime Scaling Characteristic

Several graph and runtime operations currently scale with transition collection size because they scan the transition set.

Negative and terminal workloads provide the clearest evidence of this behavior.

### 26.2 Stateful Execution Does Not Duplicate the Machine Definition

Multiple \`MachineInstance\` values share an immutable machine definition rather than copying it.

The instance memory benchmark showed identical incremental instance memory across all tested machine sizes.

### 26.3 Instance Memory Is Dominated by Runtime State, Not Graph Size

For instances initialized to \`"state_0"\`, the benchmark measured 39 incremental heap bytes per instance in a preallocated collection.

The result consisted of 32 bytes of collection storage per \`MachineInstance\` plus the seven-byte owned state-name buffer.

The exact value depends on representation and state-name length.

### 26.4 Successful Runtime Queries Are Position Sensitive

Successful initial-state validation, allowed transition queries, successful state transitions, and nonterminal source queries can terminate early.

Their timing therefore depends on randomized \`HashSet\` iteration order.

### 26.5 Negative Runtime Queries Expose Transition-Scan Cost

Missing initial states, absent exact transitions, and terminal-state checks require proving absence.

These workloads provide cleaner scaling references for future representation changes.

### 26.6 Runtime State Mutation Has a Measurable Representation Cost

A successful \`transition_to()\` performs both transition validation and current-state mutation.

In v0.4 the current state is represented using an owned state name.

This provides a baseline for evaluating future runtime-state representations.

### 26.7 Machine Construction Has Significant Allocation Traffic

At 100,000 transitions, the public-API construction workload retained approximately 8.60 MB while requesting approximately 17.42 MB cumulatively and reaching approximately 10.87 MB of additional peak live memory.

The construction workload includes temporary formatted input strings, so cumulative allocation counts and bytes should not be attributed solely to Statekit internals.

### 26.8 Projection APIs Remain Relatively Expensive

\`sources()\` and especially \`states()\` remain substantially more expensive than simple scans because they construct temporary unique-state collections.

### 26.9 Optimization Should Target Runtime Access Complexity

The measurements do not indicate that small runtime objects or shared ownership are the dominant cost.

The strongest repeated runtime costs come from operations that must search the transition representation.

Future optimization should therefore be evaluated primarily in terms of runtime access complexity rather than micro-optimizing \`MachineInstance\` itself.

### 26.10 Optimization Must Include Memory Tradeoffs

Indexes, adjacency structures, cached state sets, or lowered runtime identities may improve repeated runtime operations while increasing retained machine memory and construction cost.

Future implementations should therefore be evaluated across at least:

    query latency
    runtime transition latency
    construction latency
    retained memory
    peak construction memory
    allocation behavior
    implementation complexity

---

## 27. Future Comparison Rules

When comparing a later Statekit version against this baseline:

1. Use the same benchmark graph topology where possible.
2. Use the same transition counts.
3. Preserve benchmark probe semantics.
4. Preserve realistic missing-probe structure.
5. Fully consume lazy iterators where traversal cost is intended.
6. Do not assign full-scan throughput to early-exit workloads.
7. Use fresh equivalent instance state for mutating runtime benchmarks.
8. Exclude instance setup from \`transition_to()\` timing when transition latency is the intended measurement.
9. Do not benchmark impossible instance states that violate public construction invariants.
10. Use the same benchmark host where practical.
11. Record Rust, Cargo, LLVM, Android, kernel, and Criterion changes.
12. Treat successful \`HashSet\` lookup measurements as position-sensitive.
13. Preserve allocator measurement definitions when comparing memory results.
14. Distinguish retained memory from cumulative allocation traffic.
15. Distinguish allocator-requested bytes from process resident memory.
16. Record state-name lengths when comparing instance memory.
17. Distinguish benchmark methodology changes from implementation performance changes.

If a benchmark definition must change, the new result should not be presented as a direct regression comparison with this baseline without explaining the methodological difference.

---

## 28. Baseline Status

This document freezes the Statekit v0.4 benchmark baseline.

The measurements should remain unchanged as a historical record.

Future benchmark results should be recorded in a new versioned baseline.

For example:

    baseline-v0.5.md

The benchmark documentation index may be updated to point to the newest baseline, while this document remains the frozen historical record for v0.4.

---

## 29. Next Phase

The v0.4 correctness, runtime, and memory baselines are now established.

Statekit now has measurements covering both immutable machine definition operations and stateful runtime execution.

Future work can therefore evaluate changes against evidence in multiple dimensions.

Potential future investigation areas include:

- runtime state identity and lowered state representations
- transition membership indexing
- source-based adjacency indexing
- cached state membership
- cached source projection
- cached state projection

These are candidate directions, not commitments.

A future runtime identity representation may reduce per-instance state storage or mutation costs.

A future indexing strategy may substantially reduce repeated scan-based query latency while increasing machine construction work and retained memory.

Those tradeoffs should be measured rather than assumed.

The purpose of this baseline is not to prove that Statekit v0.4 is fast or memory efficient.

Its purpose is to make future claims of improvement measurable.