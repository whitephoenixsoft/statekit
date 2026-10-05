use statekit::{Machine, MachineInstance};
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);
static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

static LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);
static PEAK_LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };

        if !ptr.is_null() {
            let size = layout.size();

            ALLOCATED.fetch_add(size, Ordering::Relaxed);
            ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);

            let new_live = LIVE_BYTES.fetch_add(size, Ordering::Relaxed) + size;

            PEAK_LIVE_BYTES.fetch_max(new_live, Ordering::Relaxed);
        }

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();

        DEALLOCATED.fetch_add(size, Ordering::Relaxed);
        LIVE_BYTES.fetch_sub(size, Ordering::Relaxed);

        unsafe {
            System.dealloc(ptr, layout);
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };

        if !ptr.is_null() {
            let size = layout.size();

            ALLOCATED.fetch_add(size, Ordering::Relaxed);
            ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);

            let new_live = LIVE_BYTES.fetch_add(size, Ordering::Relaxed) + size;

            PEAK_LIVE_BYTES.fetch_max(new_live, Ordering::Relaxed);
        }

        ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };

        if !new_ptr.is_null() {
            let old_size = layout.size();

            DEALLOCATED.fetch_add(old_size, Ordering::Relaxed);
            ALLOCATED.fetch_add(new_size, Ordering::Relaxed);
            ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);

            LIVE_BYTES.fetch_sub(old_size, Ordering::Relaxed);

            let new_live = LIVE_BYTES.fetch_add(new_size, Ordering::Relaxed) + new_size;

            PEAK_LIVE_BYTES.fetch_max(new_live, Ordering::Relaxed);
        }

        new_ptr
    }
}

fn reset_peak() {
    let live = LIVE_BYTES.load(Ordering::Relaxed);
    PEAK_LIVE_BYTES.store(live, Ordering::Relaxed);
}

#[derive(Debug, Clone, Copy)]
struct AllocatorSnapshot {
    allocated_bytes: usize,
    allocation_count: usize,
    live_bytes: usize,
    peak_live_bytes: usize,
}

fn allocator_snapshot() -> AllocatorSnapshot {
    AllocatorSnapshot {
        allocated_bytes: ALLOCATED.load(Ordering::Relaxed),
        allocation_count: ALLOCATION_COUNT.load(Ordering::Relaxed),
        live_bytes: LIVE_BYTES.load(Ordering::Relaxed),
        peak_live_bytes: PEAK_LIVE_BYTES.load(Ordering::Relaxed),
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

struct MemoryMeasurement {
    instance_count: usize,
    retained_bytes: usize,
    allocated_bytes: usize,
    allocation_count: usize,
    peak_live_bytes: usize,
}

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

fn build_instances(machine: &Machine, instance_count: usize) -> Vec<MachineInstance> {
    let mut instances = Vec::with_capacity(instance_count);

    for _index in 0..instance_count {
        let instance = machine.instance("state_0").expect("valid instance");

        instances.push(instance);
    }

    instances
}

fn measure_machine(size: usize) -> MemoryMeasurement {
    let before = allocator_snapshot();

    reset_peak();

    let machine = build_linear_machine(size);

    let after = allocator_snapshot();

    let measurement = MemoryMeasurement {
        instance_count: 0,
        retained_bytes: after.live_bytes - before.live_bytes,
        allocated_bytes: after.allocated_bytes - before.allocated_bytes,
        allocation_count: after.allocation_count - before.allocation_count,
        peak_live_bytes: after.peak_live_bytes - before.live_bytes,
    };

    drop(machine);

    measurement
}

fn measure_instance(machine_size: usize, instance_count: usize) -> MemoryMeasurement {
    let machine = build_linear_machine(machine_size);

    let before = allocator_snapshot();

    reset_peak();

    let instances = build_instances(&machine, instance_count);

    let after = allocator_snapshot();

    let measurement = MemoryMeasurement {
        instance_count,
        retained_bytes: after.live_bytes - before.live_bytes,
        allocated_bytes: after.allocated_bytes - before.allocated_bytes,
        allocation_count: after.allocation_count - before.allocation_count,
        peak_live_bytes: after.peak_live_bytes - before.live_bytes,
    };

    drop(instances);
    drop(machine);

    measurement
}

fn report_retained_machine_memory(map: &BTreeMap<usize, MemoryMeasurement>) {
    println!("\n=== Machine Retained Memory ===\n");
    println!("transitions\tretained\tbytes/transition");
    for (count, m) in map.iter() {
        println!(
            "{count:>11}\t{:<15}\t{:<.2}",
            m.retained_bytes,
            m.retained_bytes as f64 / *count as f64,
        );
    }
}

fn report_retained_instance_memory(map: &BTreeMap<usize, BTreeMap<usize, MemoryMeasurement>>) {
    println!("\n=== Instance Retained Memory ===\n");
    println!("transitions\tinstances\tretained\tbytes/instance");
    for (count, map2) in map.iter() {
        for (_, m) in map2.iter() {
            println!(
                "{count:>11}\t{:<11}\t{:<15}\t{:<.2}",
                m.instance_count,
                m.retained_bytes,
                m.retained_bytes as f64 / m.instance_count as f64,
            );
        }
        println!();
    }
}

fn report_machine_construction_allocations(map: &BTreeMap<usize, MemoryMeasurement>) {
    println!("\n=== Machine Allocated Memory ===\n");
    println!("transitions\tallocated\tbytes/transition");
    for (count, m) in map.iter() {
        println!(
            "{count:>11}\t{:<15}\t{:<.2}",
            m.allocated_bytes,
            m.allocated_bytes as f64 / *count as f64,
        );
    }
}

fn report_instance_construction_allocations(
    map: &BTreeMap<usize, BTreeMap<usize, MemoryMeasurement>>,
) {
    println!("\n=== Instance Allocated Memory ===\n");
    println!("transitions\tinstances\tallocated\tbytes/instance");
    for (count, map2) in map.iter() {
        for (_, m) in map2.iter() {
            println!(
                "{count:>11}\t{:<11}\t{:<15}\t{:<.2}",
                m.instance_count,
                m.allocated_bytes,
                m.allocated_bytes as f64 / m.instance_count as f64,
            );
        }
        println!();
    }
}

fn report_machine_allocation_count(map: &BTreeMap<usize, MemoryMeasurement>) {
    println!("\n=== Machine Allocation Count ===\n");
    println!("transitions\tcount\tallocations/transition");
    for (count, m) in map.iter() {
        println!(
            "{count:>11}\t{:<15}\t{:<.2}",
            m.allocation_count,
            m.allocation_count as f64 / *count as f64,
        );
    }
}

fn report_instance_allocation_count(map: &BTreeMap<usize, BTreeMap<usize, MemoryMeasurement>>) {
    println!("\n=== Instance Allocation Count ===\n");
    println!("transitions\tinstances\tcount\tallocations/instance");
    for (count, map2) in map.iter() {
        for (_, m) in map2.iter() {
            println!(
                "{count:>11}\t{:<11}\t{:<15}\t{:<.2}",
                m.instance_count,
                m.allocation_count,
                m.allocation_count as f64 / m.instance_count as f64,
            );
        }
        println!();
    }
}

fn report_machine_peak_memory(map: &BTreeMap<usize, MemoryMeasurement>) {
    println!("\n=== Machine Peak Construction Memory ===\n");
    println!("transitions\tpeak additional\tbytes/transition");
    for (count, m) in map.iter() {
        println!(
            "{count:>11}\t{:<15}\t{:<.2}",
            m.peak_live_bytes,
            m.peak_live_bytes as f64 / *count as f64,
        );
    }
}

fn report_instance_peak_memory(map: &BTreeMap<usize, BTreeMap<usize, MemoryMeasurement>>) {
    println!("\n=== Instance Peak Construction Memory ===\n");
    println!("transitions\tinstances\tpeak additional\tbytes/instance");
    for (count, map2) in map.iter() {
        for (_, m) in map2.iter() {
            println!(
                "{count:>11}\t{:<11}\t{:<15}\t{:<.2}",
                m.instance_count,
                m.peak_live_bytes,
                m.peak_live_bytes as f64 / m.instance_count as f64,
            );
        }
        println!();
    }
}

fn benchmark_machine() {
    let mut map = BTreeMap::new();

    for size in [100, 1_000, 10_000, 100_000] {
        let measurement = measure_machine(size);

        map.insert(size, measurement);
    }

    report_retained_machine_memory(&map);
    report_machine_construction_allocations(&map);
    report_machine_allocation_count(&map);
    report_machine_peak_memory(&map);

    println!("\n");
}

fn benchmark_instance() {
    let mut map: BTreeMap<usize, BTreeMap<usize, MemoryMeasurement>> = BTreeMap::new();

    for machine_size in [100, 1_000, 10_000, 100_000] {
        for instance_count in [1, 100, 1_000] {
            let measurement = measure_instance(machine_size, instance_count);

            map.entry(machine_size)
                .or_default()
                .insert(instance_count, measurement);
        }
    }

    println!(
        "MachineInstance size: {} bytes",
        std::mem::size_of::<MachineInstance>()
    );

    report_retained_instance_memory(&map);
    report_instance_construction_allocations(&map);
    report_instance_allocation_count(&map);
    report_instance_peak_memory(&map);

    println!("\n");
}

fn main() {
    println!("Statekit memory benchmark");

    benchmark_machine();
    benchmark_instance();
}
