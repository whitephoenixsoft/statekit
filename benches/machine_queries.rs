use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
    Throughput,
};
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

criterion_group!(
    benches,
    benchmark_can_transition_existing,
    benchmark_can_transition_missing,
    benchmark_targets_from_existing,
    benchmark_targets_from_missing
);

criterion_main!(benches);