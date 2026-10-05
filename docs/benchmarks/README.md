# Statekit Benchmarks

This directory contains benchmark results and documentation for Statekit.

Statekit benchmarks runtime performance with Criterion and uses allocator instrumentation to measure memory behavior across different machine and instance sizes.

## Current Baseline

The current benchmark baseline is:

- [Statekit v0.4 Benchmark Baseline](baseline-v0.4.md)

The baseline contains the benchmark environment, methodology, measurements, interpretation, and known limitations for Statekit v0.4.

## Benchmark Coverage

The benchmark suite measures:

- transition iteration
- transition membership queries
- state membership queries
- outgoing-target queries
- source projection
- state projection
- machine construction and destruction
- machine instance creation
- instance transition queries
- instance transitions
- instance terminal-state queries
- machine construction memory
- machine instance memory

Measurements are collected using machines containing 100, 1,000, 10,000, and 100,000 transitions.

Instance memory measurements additionally vary the number of live instances to measure incremental per-instance memory behavior.

## Benchmark Source

The benchmark implementation used for the baseline is available here:

- [Statekit Machine benchmark source](../../benches/machine_queries.rs)
- [Statekit Instance benchmark source](../../benches/machine_runtime.rs)
- [Statekit memory benchmark source](../../benches/machine_memory.rs)

When viewing a release tag, the benchmark source and baseline documentation correspond to that version of Statekit.

## Running the Benchmarks

Run the benchmark suite with:

    cargo bench

Run individual benchmarks with:
```sh
cargo bench --bench machine_queries
cargo bench --bench machine_runtime
cargo bench --bench machine_memory
```

Benchmark timings depend on hardware and execution conditions. The recorded baseline is primarily intended as a reference point for comparing Statekit performance across implementations.
