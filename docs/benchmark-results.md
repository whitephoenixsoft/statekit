# Statekit Benchmark Results

This document tracks benchmark results for `statekit` and records the reasoning behind each benchmark.

The goal is not simply to collect timing numbers. The benchmarks are intended to answer three questions:

1. How does each public query scale as the number of transitions grows?
2. Which operations are sensitive to graph shape or query distribution?
3. Which internal optimizations are justified by evidence?

Benchmarks are currently run with Criterion using optimized benchmark builds.

> Note: absolute timings depend on machine, CPU state, background load, compiler version, and hash-table layout. The most useful signals are scaling behavior, throughput trends, and controlled before/after comparisons.

---

## Benchmark Graph

Current benchmarks use a deterministic linear state machine:

```text
state_0 -> state_1
state_1 -> state_2
state_2 -> state_3
...
```

For `n` transitions, the graph contains:

```text
n transitions
n + 1 states
n unique sources
```

Current benchmark sizes:

```text
100
1,000
10,000
100,000
```

Machine construction happens outside Criterion's timed loop for query benchmarks.

---

# `can_transition`

## Existing Transition

This benchmark queries a transition known to exist.

Because the current implementation scans a `HashSet<Transition>`, successful lookup cost depends on where the matching transition appears in the hash table's unspecified iteration order.

Results:

| Transitions | Time |
|---:|---:|
| 100 | ~134 ns |
| 1,000 | ~248 ns |
| 10,000 | ~12.74 µs |
| 100,000 | ~70.97 µs |

These results do not form a clean scaling curve.

That is expected.

A successful lookup can terminate as soon as the matching transition is encountered:

```text
scan
scan
scan
MATCH
stop
```

Since `HashSet` iteration order is unspecified and affected by randomized hashing, the number of examined transitions varies between benchmark runs and graph sizes.

### Interpretation

`can_transition(existing)` is useful as a realistic successful-query workload, but it is not currently a reliable benchmark for inferring asymptotic scaling.

Its worst-case complexity is still linear with the number of transitions.

---

## Missing Transition

This benchmark queries a transition that is guaranteed not to exist.

The probe is intentionally shaped similarly to real generated state names so that string comparisons are not artificially cheap.

Because the transition does not exist, the current implementation must exhaust the transition collection before returning `false`.

Results:

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~61.07 ns | ~1.64 Gelem/s |
| 1,000 | ~690.47 ns | ~1.45 Gelem/s |
| 10,000 | ~6.86 µs | ~1.46 Gelem/s |
| 100,000 | ~143.42 µs | ~697 Melem/s |

### Scaling

From 100 to 10,000 transitions, latency scales close to linearly:

```text
100 -> 1,000
10x transitions
~11.3x time

1,000 -> 10,000
10x transitions
~9.9x time
```

At 100,000 transitions, per-element throughput drops substantially.

Approximate throughput:

```text
100        ~1.64 billion transitions/sec
1,000      ~1.45 billion transitions/sec
10,000     ~1.46 billion transitions/sec
100,000    ~0.70 billion transitions/sec
```

### Interpretation

The benchmark strongly supports the current implementation behaving approximately as:

```text
O(n)
```

for missing transition queries.

The larger graph also shows that asymptotic complexity alone does not describe real machine performance.

At 100,000 transitions, the cost per examined transition increases substantially, likely due to effects such as:

- larger working set
- CPU cache misses
- hash-table memory layout
- pointer chasing
- heap-backed strings

The implementation remains algorithmically linear, but each unit of work becomes more expensive once the data set grows.

---

# `targets_from`

`targets_from(source)` returns a lazy iterator.

Benchmarking iterator construction alone would not measure the actual traversal cost:

```rust
machine.targets_from(source)
```

Instead, the benchmark consumes the iterator:

```rust
machine.targets_from(source).count()
```

This forces traversal of the transition collection.

Two semantic cases are measured:

```text
existing source
missing source
```

Missing source probes are shaped similarly to real state names while being guaranteed not to exist as sources.

---

## Existing Source

Results:

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~176.76 ns | ~565.7 Melem/s |
| 1,000 | ~2.01 µs | ~497.3 Melem/s |
| 10,000 | ~25.30 µs | ~395.3 Melem/s |
| 100,000 | ~613.61 µs | ~163.0 Melem/s |

---

## Missing Source

Results:

| Transitions | Time | Throughput |
|---:|---:|---:|
| 100 | ~177.23 ns | ~564.3 Melem/s |
| 1,000 | ~2.02 µs | ~494.8 Melem/s |
| 10,000 | ~24.83 µs | ~402.8 Melem/s |
| 100,000 | ~604.16 µs | ~165.5 Melem/s |

---

## Existing vs Missing Source

After controlling the shape of the missing probe, the two workloads became nearly identical:

```text
                   existing        missing

100                176.8 ns        177.2 ns
1,000                2.01 µs         2.02 µs
10,000               25.30 µs        24.83 µs
100,000             613.6 µs        604.2 µs
```

This confirms that the dominant cost is scanning the full transition collection rather than whether the queried source exists.

The iterator is effectively equivalent to:

```rust
transitions
    .iter()
    .filter(|transition| transition.source() == source)
    .map(Transition::target)
```

When `.count()` consumes this iterator, every transition must be examined because additional matching targets may appear later.

### Interpretation

Both existing-source and missing-source workloads exhibit approximately linear traversal behavior.

The similarity between the two cases provides stronger evidence than either benchmark alone.

The throughput trend also shows increasing per-transition cost as the graph grows:

```text
~565 Melem/s at 100
~500 Melem/s at 1,000
~400 Melem/s at 10,000
~164 Melem/s at 100,000
```

This again suggests that cache and memory effects become significant for larger machines.

---

# Probe Design

Benchmark query values matter.

An earlier missing-source probe looked substantially different from real state names.

For example:

```text
missing_sou torce
```

while real states looked like:

```text
state_1234
```

This made negative string comparisons unusually cheap.

After changing the missing probe to resemble real state names while still guaranteeing that it could not collide with them, `targets_from(existing)` and `targets_from(missing)` converged closely.

This demonstrates an important benchmarking principle:

> Control the semantic workload and the shape of benchmark inputs independently.

For compound predicates such as:

```rust
transition.source() == source
    && transition.target() == target
```

short-circuit evaluation also matters.

If the source comparison fails, the target comparison is never evaluated.

Therefore benchmark input distribution can materially affect measured cost even when asymptotic complexity is unchanged.

---

# Throughput

Criterion throughput is currently reported as:

```rust
Throughput::Elements(transition_count as u64)
```

for benchmarks that are guaranteed to scan the full transition collection.

This is appropriate for:

```text
can_transition/missing
targets_from/existing + full iterator consumption
targets_from/missing + full iterator consumption
```

It is not used for successful `can_transition` queries because those may terminate early and therefore do not necessarily examine `transition_count` elements.

Benchmark throughput metadata should describe work that is guaranteed to occur, not merely the size of the input collection.

---

# Criterion Change Reports

Criterion may report messages such as:

```text
Performance has improved.
Performance has regressed.
No change in performance detected.
```

These compare the current benchmark against previously stored results with the same benchmark identifier.

They must be interpreted carefully.

If the benchmark workload itself changes, the comparison is no longer measuring only a code-performance change.

For example, changing a missing probe from an obviously unrelated string to a realistic state-shaped string produced large reported regressions even though the Statekit implementation had not changed.

Therefore:

> Benchmark definitions should remain stable once they are used for performance-regression tracking.

---

# Current Performance Model

Based on the benchmarks so far:

## `can_transition(existing)`

```text
Behavior:
    scan until matching edge

Worst-case complexity:
    O(n)

Observed characteristics:
    sensitive to HashSet iteration position
    highly variable across graph sizes and runs
```

## `can_transition(missing)`

```text
Behavior:
    scan entire transition collection

Complexity:
    O(n)

Observed characteristics:
    clean scaling signal
    strong throughput drop at larger graph sizes
```

## `targets_from(source).count()`

```text
Behavior:
    scan entire transition collection
    evaluate source predicate for every transition

Complexity:
    O(n)

Observed characteristics:
    existing and missing sources have nearly identical cost
    throughput decreases substantially as graph size increases
```

---

# Current Conclusions

The current implementation is performant for small machines, but several query operations scale linearly with transition count.

The benchmarks also show that real performance is influenced by more than Big-O complexity.

Important effects observed so far include:

- successful lookup position in `HashSet`
- randomized hash-table iteration order
- query-string comparison cost
- short-circuit predicate behavior
- CPU cache and working-set effects
- reduced per-element throughput at larger graph sizes

No storage optimization has been made yet.

The current goal is to establish a trustworthy baseline before considering alternate internal representations or indexes.

---

# Next Benchmarks

Planned next measurements:

```text
sources()
states()
```

These are expected to differ from the existing query benchmarks because they do more than scan transitions.

They also construct temporary sets and deduplicate values, introducing:

```text
allocation
hashing
deduplication
```

The working hypothesis is:

> `sources()` and `states()` will scale approximately linearly but will be substantially more expensive per transition than `targets_from()`, with `states()` likely costing more because both endpoints are processed.