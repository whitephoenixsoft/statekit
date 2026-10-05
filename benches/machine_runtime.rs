use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, BatchSize};
use statekit::Machine;
use std::hint::black_box;

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

                    black_box(instance)
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

                    black_box(instance)
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
                    let instance = machine.instance(black_box("definitely_missing"));

                    black_box(instance)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_can_transition_to_allowed(c: &mut Criterion) {
    let mut group = c.benchmark_group("can_transition_to");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);
        let instance = machine.instance("state_0")
            .expect("benchmark state exists");

        group.bench_with_input(
            BenchmarkId::new("allowed", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        instance.can_transition_to(black_box("state_1"))
                    );
                });
            },
        );
    }

    group.finish();
}


fn benchmark_can_transition_to_disallowed_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("can_transition_to");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);
        let instance = machine.instance("state_0")
            .expect("benchmark state exists");

        group.bench_with_input(
            BenchmarkId::new("disallowed_existing", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        instance.can_transition_to(black_box("state_2"))
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_can_transition_to_missing(c: &mut Criterion) {
    let mut group = c.benchmark_group("can_transition_to");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);
        let instance = machine.instance("state_0")
            .expect("benchmark state exists");

        group.bench_with_input(
            BenchmarkId::new("missing", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        instance.can_transition_to(black_box("definitely_missing"))
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_transition_to_allowed(c: &mut Criterion) {
    let mut group = c.benchmark_group("transition_to");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);

        group.bench_with_input(
            BenchmarkId::new("allowed", transition_count),
            &transition_count,
            |b, _| {
                b.iter_batched(
                    || {
                        machine.instance(black_box("state_0"))
                            .expect("benchmark state exists")
                    },
                    |mut instance| {
                        black_box(
                            instance.transition_to(black_box("state_1"))
                        )
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn benchmark_transition_to_disallowed(c: &mut Criterion) {
    let mut group = c.benchmark_group("transition_to");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);

        group.bench_with_input(
            BenchmarkId::new("disallowed_existing", transition_count),
            &transition_count,
            |b, _| {
                b.iter_batched(
                    || {
                        machine.instance(black_box("state_0"))
                            .expect("benchmark state exists")
                    },
                    |mut instance| {
                        black_box(
                            instance.transition_to(black_box("state_2"))
                        )
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn benchmark_transition_to_missing(c: &mut Criterion) {
    let mut group = c.benchmark_group("transition_to");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);

        group.bench_with_input(
            BenchmarkId::new("missing", transition_count),
            &transition_count,
            |b, _| {
                b.iter_batched(
                    || {
                        machine.instance(black_box("state_0"))
                            .expect("benchmark state exists")
                    },
                    |mut instance| {
                        black_box(
                            instance.transition_to(black_box("definitely_missing"))
                        )
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn benchmark_is_terminal_source(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_terminal");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);
        let instance = machine.instance("state_0")
            .expect("benchmark state exists");

        group.bench_with_input(
            BenchmarkId::new("source", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        instance.is_terminal()
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_is_terminal_terminal(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_terminal");

    for transition_count in SIZES {
        let machine = build_linear_machine(transition_count);
        let terminal = format!("state_{transition_count}");
        let instance = machine.instance(terminal.as_str())
            .expect("benchmark state exists");

        group.bench_with_input(
            BenchmarkId::new("terminal", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        instance.is_terminal()
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
    benchmark_instance_construction_missing,
    benchmark_can_transition_to_allowed,
    benchmark_can_transition_to_disallowed_existing,
    benchmark_can_transition_to_missing,
    benchmark_transition_to_allowed,
    benchmark_transition_to_disallowed,
    benchmark_transition_to_missing,
    benchmark_is_terminal_source,
    benchmark_is_terminal_terminal
);

criterion_main!(instance_benches);
