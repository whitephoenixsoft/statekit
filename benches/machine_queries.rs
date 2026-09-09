use criterion::{
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
    Throughput,
};
use std::hint::black_box;
use std::time::Duration;
use statekit::Machine;

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

fn missing_state_like_existing(transition_count: usize) -> String {
    let digits = transition_count.to_string().len();

    format!("state_{}x", "9".repeat(digits.saturating_sub(1)))
}

fn missing_source_like_existing(transition_count: usize) -> String {
    let digits = (transition_count - 1).to_string().len();

    format!(
        "state_{}x",
        "9".repeat(digits.saturating_sub(1)),
    )
}

fn build_linear_inputs(transition_count: usize) -> Vec<(String, String)> {
    (0..transition_count)
        .map(|index| {
            (
                format!("state_{index}"),
                format!("state_{}", index + 1),
            )
        })
        .collect()
}

fn benchmark_can_transition_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("can_transition");

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);

        let source = format!("state_{}", transition_count - 1);
        let target = format!("state_{transition_count}");

        group.bench_with_input(
            BenchmarkId::new("existing", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.can_transition(
                            black_box(&source),
                            black_box(&target),
                        )
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_can_transition_missing(c: &mut Criterion) {
    let mut group = c.benchmark_group("can_transition");

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);

        let source = missing_state_like_existing(transition_count);
        let target = format!("{source}x");

        group.throughput(
            Throughput::Elements(transition_count as u64)
        );
        
        group.bench_with_input(
            BenchmarkId::new("missing", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.can_transition(
                            black_box(&source),
                            black_box(&target),
                        )
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_targets_from_existing(c: &mut Criterion) {
    let mut group = c.benchmark_group("targets_from");
    group.measurement_time(Duration::from_secs(10));
    
    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);

        let source = format!("state_{}", transition_count - 1);

        group.throughput(
            Throughput::Elements(transition_count as u64)
        );
        
        group.bench_with_input(
            BenchmarkId::new("existing", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.targets_from(
                            black_box(&source),
                        ).count()
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_targets_from_missing(c: &mut Criterion) {
    let mut group = c.benchmark_group("targets_from");
    group.measurement_time(Duration::from_secs(10));

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);

        let source = missing_source_like_existing(transition_count);        
        
        group.throughput(
            Throughput::Elements(transition_count as u64)
        );
        
        group.bench_with_input(
            BenchmarkId::new("missing", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.targets_from(
                            black_box(&source),
                        ).count()
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_sources(c: &mut Criterion) {
    let mut group = c.benchmark_group("sources");
    group.measurement_time(Duration::from_secs(10));

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);      
        
        group.throughput(
            Throughput::Elements(transition_count as u64)
        );
        
        group.bench_with_input(
            BenchmarkId::from_parameter(transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.sources().count()
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_states(c: &mut Criterion) {
    let mut group = c.benchmark_group("states");
    group.measurement_time(Duration::from_secs(10));

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);      
        
        group.throughput(
            Throughput::Elements(transition_count as u64)
        );
        
        group.bench_with_input(
            BenchmarkId::from_parameter(transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.states().count()
                    );
                });
            },
        );
    }

    group.finish();
} 

fn benchmark_transitions(c: &mut Criterion) {
    let mut group = c.benchmark_group("transitions");

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);

        group.throughput(
            Throughput::Elements(transition_count as u64)
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    machine
                        .transitions()
                        .for_each(|transition| {
                            black_box(transition);
                        });
                });
            },
        );
    }

    group.finish();
}

fn benchmark_contains_state_existing_source(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_state");

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);

        let state = "state_0";

        group.bench_with_input(
            BenchmarkId::new("existing_source", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.contains_state(
                            black_box(&state),
                        )
                    );
                });
            },
        );
    }

    group.finish();
} 

fn benchmark_contains_state_target_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_state");

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);

        let state = format!("state_{}", transition_count);

        group.bench_with_input(
            BenchmarkId::new("target_only", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.contains_state(
                            black_box(&state),
                        )
                    );
                });
            },
        );
    }

    group.finish();
} 

fn benchmark_contains_state_missing(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_state");

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let machine = build_linear_machine(transition_count);

        let state = missing_state_like_existing(transition_count);

        group.throughput(
            Throughput::Elements(transition_count as u64)
        );
        group.bench_with_input(
            BenchmarkId::new("missing", transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    black_box(
                        machine.contains_state(
                            black_box(&state),
                        )
                    );
                });
            },
        );
    }

    group.finish();
} 

fn benchmark_build_and_drop(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_and_drop");
    //group.measurement_time(Duration::from_secs(10));

    for transition_count in [100, 1_000, 10_000, 100_000] {
        let inputs = build_linear_inputs(transition_count);      
        
        group.throughput(
            Throughput::Elements(transition_count as u64)
        );
        
        group.bench_with_input(
            BenchmarkId::from_parameter(transition_count),
            &transition_count,
            |b, _| {
                b.iter(|| {
                    let mut builder = Machine::builder();

                    for (source, target) in &inputs {
                        builder = builder
                            .try_allow(
                                black_box(source),
                                black_box(target),
                            )
                            .expect("benchmark inputs should be valid");
                    }

                    let machine = builder
                        .build()
                        .expect("benchmark machine should contain transitions");

                    black_box(machine);
                });
            },
        );
    }

    group.finish();
} 

criterion_group!(
    benches,
    benchmark_can_transition_existing,
    benchmark_can_transition_missing,
    benchmark_targets_from_existing,
    benchmark_targets_from_missing,
    benchmark_sources,
    benchmark_states,
    benchmark_transitions,
    benchmark_contains_state_existing_source,
    benchmark_contains_state_target_only,
    benchmark_contains_state_missing,
    benchmark_build_and_drop
);

criterion_main!(benches);