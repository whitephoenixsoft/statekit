# Statekit Benchmarking Lab Notes for v0.4

These notes record the main lessons learned while extending the Statekit benchmark suite for v0.4.

They are intentionally different from the frozen public benchmark baseline.

The v0.3 benchmarking work established the general methodology for measuring immutable machine operations. The v0.4 work extended that methodology to Statekit's new stateful runtime API and added allocator-based memory measurements.

These notes focus on what was new or particularly useful during that process rather than repeating the full v0.3 benchmarking discussion.

---

## 1. Benchmarking Goals

Statekit v0.4 introduced stateful execution through `MachineInstance`.

The benchmark work therefore expanded from questions about immutable graph inspection to questions about runtime execution:

1. What does it cost to create a runtime instance?
2. What does it cost to query transitions from an instance?
3. What does it cost to perform a state transition?
4. What does it cost to determine whether an instance is terminal?
5. How much memory does a machine retain?
6. How much incremental memory does each runtime instance require?
7. Which costs belong to stateful execution itself, and which are consequences of the existing graph representation?

The goal remained the same as in v0.3:

    measure first
        ↓
    understand behavior
        ↓
    freeze the baseline
        ↓
    optimize later

This was especially important because future Statekit versions may change both runtime state identity and internal indexing.

---

## 2. Extending Rather Than Replacing the v0.3 Baseline

The v0.3 benchmark suite already covered the immutable machine definition:

    transition traversal
    transition membership
    state membership
    outgoing targets
    source projection
    state projection
    construction

Those benchmarks remained useful in v0.4.

The new work added two additional dimensions:

    stateful runtime latency
    memory behavior

This produced three benchmark areas:

    machine_queries
        immutable graph operations

    machine_runtime
        stateful runtime operations

    machine_memory
        allocator-based memory measurements

Keeping these concerns separate made the individual benchmark definitions easier to reason about.

---

## 3. Runtime Benchmarks Need Semantic Stability

The largest new benchmarking problem came from mutable state.

An immutable query can normally be repeated against the same object:

    query
    query
    query

A successful state transition cannot.

If an instance begins in:

    state_0

and the benchmark repeatedly executes:

    transition_to("state_1")

then the first iteration changes the instance to `state_1`.

Every later iteration is measuring a different operation.

The benchmark may still compile and produce convincing numbers, but the workload is invalid.

This produced an important rule:

> A benchmark for a mutating API must ensure that every measured iteration begins from an equivalent semantic state.

---

## 4. `iter_batched` for Stateful Operations

Criterion's batched iteration solved the mutable-state problem.

For `transition_to()`, setup creates a fresh instance and the measured routine performs the transition.

Conceptually:

    setup:
        create instance at state_0

    timed operation:
        transition_to(state_1)

    repeat

The instance setup is therefore excluded from the transition measurement while every measured operation begins from the same state.

This is preferable to creating the instance inside a normal timed loop because that would measure:

    instance construction
        +
    transition validation
        +
    state mutation

when the intended question is specifically the cost of the transition operation.

General lesson:

> Benchmark setup and benchmark semantics are separate concerns. Setup should establish the state required by the operation without accidentally becoming part of the measured workload.

---

## 5. Runtime Benchmarks Should Follow Public Invariants

`MachineInstance` guarantees that its current state belongs to its machine.

That invariant affected benchmark design.

For `Machine::instance()`, valid cases include:

    existing source
    terminal state
    missing state

because construction itself performs state validation.

For `MachineInstance::is_terminal()`, however, a missing current state is not a valid workload.

Such an instance cannot be created through the public API.

The useful cases are instead:

    source state
    terminal state

This was a useful reminder that benchmark matrices should not be made artificially symmetrical.

> Do not invent impossible states merely to give every benchmark the same set of cases.

The benchmark should reflect the actual public contract.

---

## 6. Negative Runtime Operations Are Strong Scaling Probes

The v0.3 work showed that successful scans over a randomized `HashSet` are position-sensitive.

The same lesson carried directly into the stateful runtime API.

Successful operations such as:

    Machine::instance(existing_source)
    can_transition_to(allowed)
    transition_to(allowed)
    is_terminal(source)

may stop when the required transition or state is encountered.

Their cost therefore depends partly on where the matching transition happens to appear in the `HashSet` iteration order.

Negative or absence-based operations are much cleaner:

    Machine::instance(missing)
    can_transition_to(disallowed)
    can_transition_to(missing)
    transition_to(disallowed)
    transition_to(missing)
    is_terminal(terminal)

These operations must establish that something does not exist.

With the v0.4 representation, that generally requires exhausting the transition collection.

The result was a useful family of runtime scaling probes rather than isolated benchmark numbers.

---

## 7. Terminal-State Queries Are Absence Queries

`is_terminal()` initially looks like a simple state predicate.

Semantically, however, it asks:

> Does the current state have no outgoing transition?

For a source state, the query may stop as soon as one outgoing transition is found.

For a terminal state, the implementation must establish that no outgoing transition exists.

With the current scan-based representation:

    source
        potentially early exit

    terminal
        exhaustive search

The terminal workload therefore became the stronger baseline for future adjacency indexing.

This was another example of a broader benchmarking lesson:

> The positive-sounding and negative-sounding forms of an API do not necessarily correspond to the expensive and cheap workloads. What matters is what must be proven internally.

---

## 8. Exact Transition Semantics Simplify Negative Cases

`can_transition_to(target)` asks whether the exact directed edge from the current state to `target` exists.

A target can therefore be:

    an existing state without the required edge
    a completely missing state

Both cases are negative exact-transition queries.

This matters because Statekit does not need to establish target-state membership before answering the transition question.

It needs to establish whether the edge exists.

The two negative workloads therefore exercise very similar transition-search behavior.

This reinforced an API-design principle already present in Statekit:

> Exact-match queries should answer the exact question they were asked rather than performing unrelated validation first.

---

## 9. Failed Operations Are Still Complete Public Operations

For `transition_to()`, failure does more than return `false`.

It returns an error.

The benchmark therefore includes the complete failed public operation:

    transition search
        +
    failure determination
        +
    error construction

This can make failed `transition_to()` somewhat more expensive than the corresponding boolean query.

That is not benchmark contamination.

It is part of the API's actual behavior.

The benchmark should measure the operation consumers call, not an artificially stripped-down internal approximation.

---

## 10. Throughput Was Not Useful for Every Runtime Benchmark

The v0.3 suite used transition throughput where the benchmark was guaranteed to process all transitions.

For the new runtime API, the more natural unit is often:

    one public operation

A machine may contain 100,000 transitions, but calling:

    instance.can_transition_to(target)

is semantically one query.

Assigning:

    Throughput::Elements(100_000)

would imply that 100,000 transitions are necessarily processed by every call.

That is false for early-exit successful operations.

Runtime benchmarks were therefore reported primarily as latency.

General rule:

> The benchmark's input size does not automatically define its throughput unit.

---

## 11. Memory Needed Its Own Benchmarking Method

Criterion answers timing questions well.

It does not by itself answer:

    How many bytes remain live?
    How many bytes were allocated cumulatively?
    How many allocations occurred?
    What was peak live heap usage?

The v0.4 work therefore introduced a custom counting allocator benchmark.

The measured counters were:

    total allocated bytes
    allocation count
    live bytes
    peak live bytes

This made it possible to distinguish several concepts that are often casually collapsed into "memory usage."

---

## 12. Retained Memory and Allocation Traffic Are Different

One of the most useful distinctions was:

    retained memory
        bytes still live after construction

versus:

    cumulative allocated memory
        all requested allocation bytes during construction

A machine may retain substantially less memory than was cumulatively allocated while building it.

Temporary strings, collection growth, reallocations, and other short-lived allocations contribute to allocation traffic without remaining in the finished machine.

This distinction will be particularly useful when future versions introduce indexes.

An implementation might:

    retain more memory
    allocate differently during construction
    perform runtime queries much faster

No single memory number can describe that tradeoff.

---

## 13. Peak Memory Must Be Measured Relative to the Starting Point

The allocator counters are cumulative process-level instrumentation.

Resetting global totals is less useful than taking snapshots around a controlled workload.

Peak live memory also requires care.

The useful quantity is not simply the allocator's absolute peak since process start.

It is:

    peak live bytes during workload
        -
    live bytes before workload

The peak counter was therefore reset to the current live-byte level before the measured operation.

This allowed the benchmark to describe additional peak memory caused by construction rather than unrelated memory already owned by the benchmark process.

---

## 14. Allocator Measurements Are Not RSS

The custom allocator measures requested heap allocation sizes.

It does not measure exact physical process memory.

The benchmark does not include concepts such as:

    allocator metadata
    fragmentation outside requested sizes
    operating-system page accounting
    resident-set behavior
    unrelated process memory

This is a limitation, but also a strength.

For Statekit version comparisons, allocator-requested bytes provide a much cleaner signal than process RSS on Android.

The goal is not:

> How many physical bytes does Android currently attribute to this process?

The more useful Statekit question is:

> How does the allocation behavior of this workload change when Statekit's representation changes?

---

## 15. Benchmark Harness Memory Can Contaminate Results

The instance-memory experiment initially included collection behavior that could obscure the cost being investigated.

Using:

    Vec::new()

allows the vector to grow geometrically while instances are pushed.

That means the measured workload includes temporary capacity growth and reallocations belonging to the benchmark harness.

Changing the benchmark to:

    Vec::with_capacity(instance_count)

removed that noise.

The collection storage was still real memory required by the benchmark, but its size became predictable:

    instance_count × size_of::<MachineInstance>()

This led to another useful rule:

> Memory benchmark infrastructure allocates memory too. Control it deliberately or it becomes part of the result.

---

## 16. The Instance Memory Result Explained the Representation

The final instance-memory workload produced a particularly clean result.

On the benchmark platform:

    size_of::<MachineInstance>() = 32 bytes

Each instance was initialized to:

    "state_0"

which requires seven UTF-8 bytes.

With the instance collection preallocated, measured retained memory became:

    32 bytes
        MachineInstance collection storage

    + 7 bytes
        owned current-state string

    = 39 bytes per instance

The result was identical across machines containing:

    100
    1,000
    10,000
    100,000 transitions

This was useful for more than documenting memory consumption.

It experimentally confirmed the intended ownership architecture:

> Runtime instances share the immutable machine definition rather than duplicating it.

---

## 17. Allocation Count Can Reveal Ownership Behavior

The instance benchmark also produced:

    1 instance       2 allocation events
    100 instances    101 allocation events
    1000 instances   1001 allocation events

With exact vector preallocation, this pattern is consistent with:

    one Vec backing allocation
        +
    one owned state-name allocation per instance

The shared machine definition did not introduce one machine allocation per instance.

This demonstrates an important use of memory benchmarks:

> Allocation measurements can help validate architectural expectations, not just optimize byte counts.

---

## 18. Do Not Overgeneralize Exact Memory Numbers

The 39-byte result is exact for the benchmark workload, but it is not a universal statement that:

> A MachineInstance uses 39 bytes.

The result depends on:

    platform representation
    collection context
    current-state representation
    state-name length

A longer current state requires a larger owned string buffer.

Future versions may replace the owned runtime state name with an ID-like representation.

The correct value of the v0.4 result is therefore as a baseline:

> Instances initialized to `"state_0"` added 39 measured heap bytes per instance in the controlled benchmark.

That gives future runtime representations something concrete to compare against.

---

## 19. Construction Allocation Counts Include Benchmark Inputs

Machine construction approached approximately four allocation or successful reallocation events per transition in the measured workload.

It would be easy to conclude:

> Statekit requires four allocations per transition.

That conclusion would be wrong.

The benchmark constructs formatted source and target strings while exercising the public builder workflow.

Those allocations are observed by the global allocator instrumentation too.

The result therefore describes:

    the measured public-API construction workload

not:

    Statekit's internal representation in isolation

This distinction is important whenever instrumentation surrounds code that includes both library behavior and benchmark fixture behavior.

---

## 20. Runtime Measurements Pointed to One Common Representation Cost

At 100,000 transitions, several absence-based runtime operations landed in roughly the same latency region:

    transition membership failure
    state membership failure
    missing instance state
    failed runtime transition
    terminal-state detection

These APIs have different semantics.

However, they share one important implementation characteristic:

    proving absence by scanning transitions

The measurements therefore suggested that the main runtime issue is not:

    Arc cloning
    MachineInstance size
    method-call overhead
    mutable state itself

The common cost is the graph-access representation.

This sharpened the future optimization question considerably.

---

## 21. Stateful Execution Did Not Reveal a New Raw-Speed Problem

The v0.4 benchmark work changed the performance question.

The issue is not that Statekit's basic operations are inherently slow.

The issue is that several repeated runtime operations currently have work proportional to the number of transitions.

That distinction matters.

A few nanoseconds of method overhead are not the interesting optimization target if a 100,000-transition machine requires a full graph scan to answer a common runtime question.

The more useful conclusion is:

> Statekit has a runtime access-complexity opportunity, not evidence of a general low-level execution problem.

---

## 22. Memory Makes Future Indexing a Tradeoff

The v0.3 timing baseline already suggested possible indexes.

The v0.4 memory baseline made the cost side measurable.

Future structures might include:

    transition membership index
    state membership index
    source adjacency index
    cached source projection
    cached state projection

These could dramatically reduce repeated runtime scans.

But they may also increase:

    retained machine memory
    construction latency
    peak construction memory
    allocation traffic
    implementation complexity

Future optimization therefore needs to be evaluated in several dimensions at once:

    query latency
    runtime transition latency
    construction latency
    retained memory
    peak memory
    allocation behavior
    implementation complexity

This is a stronger optimization framework than timing alone.

---

## 23. Runtime State Identity Now Has a Baseline

v0.4 stores instance current state as an owned state name.

That has observable consequences:

    instance memory includes the state-name buffer
    successful mutation changes owned runtime state
    instance construction requires runtime state ownership

A future version may represent runtime state differently.

For example, a machine-owned state identity could potentially allow instances to store a compact identifier rather than an owned string.

The v0.4 measurements do not prove that such a change is desirable.

They do provide the baseline needed to measure it.

This is exactly why representation changes should follow measurement rather than precede it.

---

## 24. Benchmark Names Are Part of the Experiment

During the runtime benchmark work, one benchmark case was discovered to have the wrong label even though its actual workload was correct.

The case labeled as a second disallowed-existing transition was actually the missing-target workload.

The fix was to rename the benchmark rather than change its implementation.

This was minor mechanically but important methodologically.

Benchmark names are data.

A correct workload with an incorrect label can lead to an incorrect interpretation months later.

> Verify benchmark identity from the code that constructs the workload, not only from the printed benchmark name.

---

## 25. The v0.4 Benchmark Suite

The completed v0.4 benchmark work now covers three layers.

### Immutable Machine Queries

    can_transition
    targets_from
    sources
    states
    transitions
    contains_state
    build_and_drop

### Stateful Runtime Operations

    Machine::instance

    MachineInstance::can_transition_to
    MachineInstance::transition_to
    MachineInstance::is_terminal

with successful, negative, missing, source, and terminal probes where semantically valid.

### Memory

    machine retained memory
    machine cumulative allocation
    machine allocation count
    machine peak construction memory

    instance retained memory
    instance cumulative allocation
    instance allocation count
    instance peak construction memory

This gives future versions a baseline across both execution time and representation cost.

---

## 26. Main Lessons from v0.4

The most important lessons from this round were:

1. Mutating APIs require every benchmark iteration to begin from equivalent semantic state.
2. Criterion batching is useful for separating mutable setup from the operation being timed.
3. Benchmark workloads should obey the same invariants as real public API usage.
4. Negative and absence-based operations remain the strongest scaling probes for randomized scan-based storage.
5. Terminal-state detection is fundamentally an absence query in the current representation.
6. Failed public operations should include legitimate error-construction cost.
7. Throughput should not be attached to runtime operations merely because the containing machine has \`n\` transitions.
8. Timing and memory require different measurement tools.
9. Retained memory, cumulative allocation traffic, allocation count, and peak memory answer different questions.
10. Allocator-requested bytes should not be confused with RSS or exact physical memory consumption.
11. Memory used by benchmark infrastructure must be controlled deliberately.
12. Exact collection preallocation can make per-instance memory experiments much easier to interpret.
13. Allocation patterns can validate ownership architecture as well as expose performance costs.
14. Exact memory results should be described in terms of the measured workload rather than generalized to every possible instance.
15. Construction allocation counts must account for allocations performed by benchmark inputs and fixture code.
16. Stateful execution did not reveal machine-definition duplication per instance.
17. The strongest runtime costs still come from scan-based graph access.
18. Future indexing decisions must be evaluated against memory and construction tradeoffs.
19. The owned runtime state name now has a concrete v0.4 memory and latency baseline.
20. Benchmark labels are part of the experimental record and must accurately describe the workload.

---

## 27. What the v0.4 Work Changed

Before the v0.4 benchmark work, the main performance observation was:

> Several immutable Statekit queries scan the transition collection.

After adding stateful execution and memory measurements, the picture is clearer:

> Statekit's current runtime representation is small and shares immutable machine definitions efficiently, but several important runtime operations inherit linear transition-scan costs from the underlying graph representation.

That distinction gives future optimization work a much more precise target.

The next representation should not merely aim to be "faster."

It should be evaluated against specific questions:

    How much runtime latency was removed?
    How much retained memory was added?
    How much construction work was added?
    Did instance memory change?
    Did runtime state mutation become cheaper?
    Which complexity classes actually changed?

The v0.4 baseline now makes those questions measurable.

---

## 28. Closing Note

The v0.3 benchmarking work was largely about learning how to construct trustworthy Rust microbenchmarks.

The v0.4 work was more about learning how benchmark design interacts with API semantics, ownership, mutable state, and memory representation.

That progression mirrors Statekit itself.

The library moved from:

    immutable definition and inspection

to:

    immutable definition
        +
    shared runtime execution state

The benchmark suite had to make the same transition.

The most useful result is not any individual nanosecond or byte count.

It is that Statekit now has enough evidence to distinguish:

    execution semantics
    ownership cost
    runtime state cost
    graph-access complexity
    memory tradeoffs

Future representation changes can therefore be evaluated against a measured system rather than intuition.