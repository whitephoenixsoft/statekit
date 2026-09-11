# Statekit v0.3 Benchmark Baseline

This document records the frozen performance baseline for Statekit v0.3.

Its purpose is to provide a stable reference point for evaluating future implementation changes.

This document records only the final workloads, environment, measurements, interpretation, and known limitations used for the v0.3 baseline.

Future Statekit versions should be compared against this baseline using equivalent benchmark definitions wherever possible.

---

## 1. Purpose

The v0.3 benchmark baseline answers three primary questions:

1. How do Statekit's current public graph operations behave as machine size increases?
2. Which operations currently dominate runtime cost?
3. What reference measurements should be preserved before changing Statekit's internal storage or indexing strategy?

The baseline is intended primarily for relative comparison between Statekit versions.

It is not a performance guarantee.

---

## 2. Statekit Version

Baseline version:

    statekit 0.3.x

The benchmark captures the v0.3 architecture in which:

- transitions are immutable after machine construction
- transitions are stored internally in a `HashSet`
- several query APIs scan the transition collection
- `sources()` and `states()` construct temporary sets
- no dedicated source, target, or state indexes are maintained

The internal storage model is an implementation detail and may change in future releases.

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

The benchmark suite uses Criterion.

Benchmarks are compiled and executed using Rust's benchmark/release optimization profile.

Criterion performs repeated measurements, warmup, statistical estimation, and outlier detection.

Criterion's stored historical comparisons are not treated as part of this frozen baseline because benchmark definitions changed during development.

Only the final measurements recorded in this document constitute the v0.3 baseline.

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
    1 target-only state

The tested transition counts are:

    100
    1,000
    10,000
    100,000

The graph is logically deterministic.

However, transitions are stored internally in a `HashSet`, so physical iteration order is unspecified and may differ between program executions.

---

## 6. Benchmark Methodology

Query benchmarks construct the `Machine` before entering Criterion's timed iteration.

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

---

## 7. Benchmark Categories

The benchmark suite contains two important categories.

### 7.1 Guaranteed Full-Work Benchmarks

These operations are guaranteed to inspect or process the complete transition collection for the benchmark workload.

They are the strongest benchmarks for scaling analysis.

They include:

- `can_transition(missing)`
- `targets_from(existing)` with complete iterator consumption
- `targets_from(missing)` with complete iterator consumption
- `contains_state(missing)`
- forced `transitions()` traversal
- `sources()`
- `states()`
- `build_and_drop`

Criterion throughput is reported for these workloads where the processed transition count is well defined.

### 7.2 Position-Sensitive Successful Lookups

These operations may terminate as soon as a matching transition is encountered:

- `can_transition(existing)`
- `contains_state(existing_source)`
- `contains_state(target_only)`

Because transition iteration order is controlled by a randomized `HashSet`, these measurements can vary substantially between executions.

They are retained as representative successful-query workloads but are not used as stable complexity baselines.

No full-scan throughput is assigned to them.

---

## 8. Raw Transition Traversal

The `transitions()` benchmark forces each transition to be individually observed.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~62.1 ns | ~1.61 Gelem/s |
| 1,000 | ~631 ns | ~1.58 Gelem/s |
| 10,000 | ~6.42 µs | ~1.56 Gelem/s |
| 100,000 | ~102 µs | ~982 Melem/s |

This benchmark provides a controlled traversal reference.

It should not be interpreted as a strict theoretical lower bound because each visited transition is deliberately made observable to prevent optimization from removing the traversal.

The results show approximately proportional growth through the smaller and medium sizes, followed by lower per-element throughput at 100,000 transitions.

---

## 9. Transition Membership

### 9.1 Missing Transition

`can_transition(missing)` uses a transition guaranteed not to exist.

This forces the entire transition collection to be scanned.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~63.1 ns | ~1.58 Gelem/s |
| 1,000 | ~723 ns | ~1.38 Gelem/s |
| 10,000 | ~7.25 µs | ~1.38 Gelem/s |
| 100,000 | ~149 µs | ~673 Melem/s |

The benchmark provides strong evidence of the current linear scan behavior.

Throughput remains relatively stable through 10,000 transitions and decreases at 100,000 transitions.

### 9.2 Existing Transition

| Transitions | Time |
| ---: | ---: |
| 100 | ~4.97 ns |
| 1,000 | ~1.95 µs |
| 10,000 | ~15.2 µs |
| 100,000 | ~2.52 µs |

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
| 100 | ~179 ns | ~557 Melem/s |
| 1,000 | ~2.46 µs | ~407 Melem/s |
| 10,000 | ~25.7 µs | ~390 Melem/s |
| 100,000 | ~610 µs | ~164 Melem/s |

### 10.2 Missing Source

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~181 ns | ~551 Melem/s |
| 1,000 | ~2.85 µs | ~351 Melem/s |
| 10,000 | ~26.2 µs | ~382 Melem/s |
| 100,000 | ~617 µs | ~162 Melem/s |

Existing and missing probes produce similar costs at larger sizes.

This supports the interpretation that the operation is dominated by traversal and source filtering rather than whether a match is ultimately found.

---

## 11. State Membership

### 11.1 Missing State

The missing-state workload guarantees that neither endpoint of any transition matches the probe.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~98.1 ns | ~1.02 Gelem/s |
| 1,000 | ~1.08 µs | ~930 Melem/s |
| 10,000 | ~10.7 µs | ~934 Melem/s |
| 100,000 | ~164 µs | ~609 Melem/s |

The workload performs a full scan.

For each nonmatching transition, both source and target membership comparisons must fail.

The first three graph sizes show a particularly clear approximately proportional relationship between input size and latency.

### 11.2 Existing Source

| Transitions | Time |
| ---: | ---: |
| 100 | ~110 ns |
| 1,000 | ~28.6 ns |
| 10,000 | ~3.53 µs |
| 100,000 | ~1.68 µs |

### 11.3 Target-Only Existing State

| Transitions | Time |
| ---: | ---: |
| 100 | ~71.9 ns |
| 1,000 | ~463 ns |
| 10,000 | ~2.33 µs |
| 100,000 | ~12.8 µs |

Both successful workloads are position-sensitive.

A target-only state is logically the final state in the linear graph, but the transition containing it may appear anywhere in `HashSet` iteration order.

These measurements therefore represent successful-query samples rather than worst-case scans.

---

## 12. Source Projection

`sources()` scans transitions and constructs a temporary set of unique source states.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~3.65 µs | ~27.4 Melem/s |
| 1,000 | ~49.6 µs | ~20.2 Melem/s |
| 10,000 | ~517 µs | ~19.3 Melem/s |
| 100,000 | ~8.76 ms | ~11.4 Melem/s |

The operation includes:

- transition traversal
- hashing
- temporary `HashSet` allocation
- insertion
- deduplication
- possible table growth

Its cost is substantially greater than scan-only operations.

---

## 13. State Projection

`states()` scans both transition endpoints and constructs a temporary set of unique state names.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~5.29 µs | ~18.9 Melem/s |
| 1,000 | ~70.9 µs | ~14.1 Melem/s |
| 10,000 | ~770 µs | ~13.0 Melem/s |
| 100,000 | ~20.9 ms | ~4.78 Melem/s |

For the linear benchmark graph, adjacent transitions share an endpoint.

The operation therefore performs both successful insertions and duplicate-detection work while processing two endpoint occurrences per transition.

`states()` is consistently more expensive than `sources()` in the frozen run, with the difference increasing at larger graph sizes.

---

## 14. Machine Build and Drop

The construction benchmark creates a complete machine from pre-generated transition strings and allows the resulting machine to be destroyed within the timed iteration.

| Transitions | Time | Throughput |
| ---: | ---: | ---: |
| 100 | ~15.0 µs | ~6.66 Melem/s |
| 1,000 | ~194 µs | ~5.16 Melem/s |
| 10,000 | ~2.23 ms | ~4.48 Melem/s |
| 100,000 | ~55.1 ms | ~1.81 Melem/s |

This measurement includes both construction and destruction.

The 100,000-transition workload shows a substantial drop in per-transition throughput compared with smaller machines.

The benchmark does not isolate the cause of that decrease.

Potential contributors such as allocation behavior, hash-table growth, memory hierarchy effects, or destruction cost require separate profiling before they can be stated as causes.

---

## 15. 100,000-Transition Cost Comparison

The largest benchmark size gives the following approximate cost hierarchy:

| Operation | Approximate latency |
| --- | ---: |
| Forced `transitions()` traversal | 0.102 ms |
| `can_transition(missing)` | 0.149 ms |
| `contains_state(missing)` | 0.164 ms |
| `targets_from(...).count()` | 0.61 ms |
| `sources().count()` | 8.76 ms |
| `states().count()` | 20.9 ms |
| `build_and_drop` | 55.1 ms |

The corresponding conceptual cost ladder is:

    raw transition traversal
        ↓
    transition membership predicate
        ↓
    two-endpoint state membership predicate
        ↓
    full traversal with source filtering
        ↓
    temporary unique-source set construction
        ↓
    temporary unique-state set construction
        ↓
    complete machine construction and destruction

This comparison is one of the most useful results of the v0.3 baseline.

It identifies where the current implementation spends substantially more time without assuming what optimization should follow.

---

## 16. Scaling Interpretation

Several current Statekit operations are implemented using full transition scans.

Their algorithmic structure is therefore linear in the number of stored transitions.

The benchmark measurements generally support that structure at smaller and medium graph sizes.

However, observed latency does not scale with a perfectly constant cost per transition.

At larger working-set sizes, throughput decreases for several workloads.

The measurements establish that the per-element cost changes.

They do not establish the hardware-level cause.

Possible cache, allocator, memory-access, scheduler, or hash-layout effects remain hypotheses unless confirmed through profiling.

Algorithmic complexity and observed machine throughput should therefore be treated as separate concepts.

---

## 17. Position-Sensitive Lookup Variability

Successful scan-based lookup results vary significantly because the internal `HashSet` does not preserve logical insertion order.

For example, a successful query over a 100,000-transition machine can terminate after examining relatively few transitions in one process and many more in another.

The v0.3 baseline therefore uses guaranteed-missing workloads as the primary evidence for scan complexity.

Successful lookup results are retained because they represent real API usage, but they should not be used alone to evaluate regression or scaling.

---

## 18. Mobile Benchmark Environment Limitations

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

---

## 19. Baseline Conclusions

The v0.3 benchmark suite supports the following conclusions.

### 19.1 Scan-Based Queries

Several query operations currently scale with transition collection size because they scan the transition set.

Missing-query workloads provide the clearest evidence of this behavior.

### 19.2 Successful Queries Are Position Sensitive

Successful scan-based queries can vary substantially because `HashSet` iteration order determines when a matching transition is encountered.

### 19.3 `targets_from` Performs Full Filtering

Fully consuming `targets_from()` requires scanning the transition collection regardless of whether the queried source exists.

### 19.4 Projection APIs Are Relatively Expensive

`sources()` and especially `states()` are substantially more expensive than simple scans because they construct temporary unique-state collections.

### 19.5 Construction Dominates Individual Queries

For large machines, complete machine construction and destruction is considerably more expensive than individual query operations.

### 19.6 Larger Working Sets Reduce Throughput

Several workloads show lower per-transition throughput at 100,000 transitions.

The benchmark establishes the observation but does not identify the underlying hardware cause.

### 19.7 Optimization Should Be Evidence Driven

Potential indexes or cached projections could improve repeated query performance, but they may also increase memory usage, construction cost, and implementation complexity.

Any such change should be compared directly against this baseline.

---

## 20. Future Comparison Rules

When comparing a later Statekit version against this baseline:

1. Use the same benchmark graph topology where possible.
2. Use the same transition counts.
3. Preserve benchmark probe semantics.
4. Preserve realistic missing-probe structure.
5. Fully consume lazy iterators where traversal cost is intended.
6. Do not assign full-scan throughput to early-exit workloads.
7. Use the same benchmark host where practical.
8. Record Rust, Cargo, LLVM, Android, kernel, and Criterion changes.
9. Treat successful `HashSet` lookup measurements as position-sensitive.
10. Distinguish benchmark methodology changes from implementation performance changes.

If a benchmark definition must change, the new result should not be presented as a direct regression comparison with this baseline without explaining the methodological difference.

---

## 21. Baseline Status

This document freezes the initial Statekit v0.3 benchmark baseline.

The measurements should remain unchanged as a historical record.

Future benchmark results belong in either:

    the current benchmark report

or:

    a new versioned baseline

For example:

    baseline-v0.4.md

The v0.3 baseline should only be corrected if a factual or transcription error is discovered, and such corrections should be explicitly documented.

---

## 22. Next Phase

The v0.3 correctness and performance baselines are now established.

Future internal optimization work can therefore proceed against measurable evidence.

Potential optimization areas include:

- transition membership indexing
- source-based transition indexing
- cached state membership
- cached source projection
- cached state projection

These are candidate directions, not commitments.

Any optimization should preserve Statekit's public semantics and invariants while being evaluated against construction cost, query latency, memory usage, and implementation complexity.

The purpose of this baseline is not to prove that Statekit v0.3 is fast.

Its purpose is to make future claims of improvement measurable.