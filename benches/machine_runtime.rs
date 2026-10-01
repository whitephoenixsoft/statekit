use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use statekit::Machine;
use std::hint::black_box;
use std::time::Duration;

fn build_linear_machine(transition_count: usize) -> Machine {
    let mut builder = Machine::builder();

    for index in 0..transition_count {
        let source = format!("state_{index}");
        let target = format!("state_{}", index + 1);

        builder = builder
            .try_allow(source, target)
            .expect("generated transition is valid");
    }

    builder
        .build()
        .expect("benchmark machine contains transitions")
}

const SIZES: [usize; 4] = [100, 1_000, 10_000, 100_000];

fn benchmark_instance_construction_existing_source(c: &mut Criterion) {
    let mut group = c.benchmark_group("instance_construction");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);

        group.bench_with_input(
            BenchmarkId::new("existing_source", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    let instance = machine
                        .instance(black_box("state_0"))
                        .expect("benchmark state exists");

                    black_box(instance);
                });
            },
        );
    }

    group.finish();
}

fn benchmark_instance_construction_terminal(c: &mut Criterion) {
    let mut group = c.benchmark_group("instance_construction");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);
        let state = format!("state_{transition_count}");
        
        group.bench_with_input(
            BenchmarkId::new("terminal", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    let instance = machine
                        .instance(black_box(state.as_str()))
                        .expect("benchmark state exists");

                    black_box(instance);
                });
            },
        );
    }

    group.finish();
}


fn benchmark_instance_construction_missing(c: &mut Criterion) {
    let mut group = c.benchmark_group("instance_construction");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);

        group.bench_with_input(
            BenchmarkId::new("missing", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                                    machine.instance(black_box("definitely_missing"))
                    );
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    instance_benches,
    benchmark_instance_construction_existing_source,
benchmark_instance_construction_terminal,
benchmark_instance_construction_missing
);

criterion_main!(instance_benches);