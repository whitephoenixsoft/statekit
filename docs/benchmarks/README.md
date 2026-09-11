# Statekit Benchmarks

This directory contains benchmark results and documentation for Statekit.

Statekit uses Criterion to benchmark machine construction and common query operations across different machine sizes.

## Current Baseline

The current benchmark baseline is:

- [Statekit v0.3 Benchmark Baseline](baseline-v0.3.md)

The baseline contains the benchmark environment, methodology, measurements, interpretation, and known limitations for Statekit v0.3.

## Benchmark Coverage

The benchmark suite measures:

- transition iteration
- transition membership queries
- state membership queries
- outgoing-target queries
- source projection
- state projection
- machine construction and destruction

Measurements are collected using machines containing 100, 1,000, 10,000, and 100,000 transitions.

## Benchmark Source

The benchmark implementation used for the baseline is available from the corresponding release tag:

- [Statekit benchmark source](../../benches/machine_queries.rs)

Using the release tag keeps the benchmark source associated with the implementation that produced the recorded measurements.

## Benchmarking Notes

Development notes covering the experiments and methodology behind the benchmark suite are available separately:

- [Benchmarking Lab Notes](../development/lab-notes)

## Running the Benchmarks

Run the benchmark suite with:

    cargo bench

Benchmark timings depend on hardware and execution conditions. The recorded baseline is primarily intended as a reference point for comparing Statekit performance across implementations.