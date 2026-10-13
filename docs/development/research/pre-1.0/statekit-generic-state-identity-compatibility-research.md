# Statekit — Generic State Identity & API Compatibility Feasibility Research

**Status:** Research proposal; no design decision or implementation is accepted here  
**Investigation:** Seventh report in Statekit Consumer Research & Capability Discovery  
**Research date:** 2026-10-13

## Executive summary

**Finding:** Generic state identity is technically feasible in Rust, but the proposed <code>pub struct Machine&lt;S = StateName&gt;</code> is not a drop-in compatibility change for Statekit. <code>StateName</code> is currently crate-private; a public generic default must be publicly nameable. Making it public would expose a currently hidden type and require decisions about its construction and conversion API. Meaningful generic identity also propagates through <code>MachineBuilder</code>, <code>Transition</code>, <code>MachineInstance</code>, public iterators, and likely error payloads. A default type parameter does not erase those design and inference requirements.

**Recommendation:** Do not add public generic identity in v0.5 on current evidence. Preserve <code>StateName</code> as Statekit's sole public logical identity and its string-oriented API. Keep private numeric lowering/indexing as a separate v0.6 implementation experiment. Use consumer-owned mappings or structural projections where a compiler already has IDs. This best protects I-002 (small and fast), I-003 (ease of use), BI-002 (hidden storage), current error matching, and borrowed iteration while leaving implementation optimization open.

**Strongest alternative:** If a second independent consumer demonstrates that converting its stable identity to a name is a repeated, material integration cost, prototype a separately named generic structural type or generic core behind the existing façade. Do not first genericize <code>Machine</code>, <code>Transition</code>, and <code>StateError</code> and assume defaults make the change invisible. A separate API can protect ordinary callers, but duplicated concepts and maintenance must be measured against a small consumer adapter.

**Key consumer evidence:** VPE resolves source names to <code>usize</code> indices and stores compiled edges by target index, but it retains node names and its semantics permit cases Statekit rejects or collapses: empty-edge processes, self-transitions, and parallel semantic transitions. Generic Statekit IDs could remove some representation translation in a test projection, but would not make the two transition models equivalent or replace VPE's compiler/runtime. No inspected consumer demonstrates that this benefit outweighs public API expansion.

**Experiment limit:** This environment has no <code>rustc</code> or <code>cargo</code>; no disposable code was compiled. The report separates source-verified facts, language-documentation findings, and hypotheses. It provides a runnable compatibility suite for a Rust-enabled environment rather than presenting unverified snippets as compiler results.

## Evidence labels and continuity

- **Verified:** observed in supplied project files, current public repositories, or earlier reports' source-backed findings.
- **Language rule:** supported by Rust's official language/library documentation.
- **Interpretation:** architectural consequence drawn from those facts.
- **Hypothesis:** plausible benefit needing a prototype or evidence.
- **Proposed API:** illustrative only; not implemented.

The six prior reports established Statekit's niche as a small topology validator/runtime for runtime-defined machines; its value for configuration validation and reference-model tests; <code>Machine</code> as a validated structural projection; consumer-owned serialization; descriptive rather than prescriptive analysis; and no proven need for public state IDs. The previous identity report recommended keeping strings as logical public identity while separately benchmarking private indexing. This report revisits generic identity against a new compatibility question, but does not change those findings without new evidence.

The supplied roadmap lists generic <code>Machine&lt;S&gt;</code> as an alternative for investigation, not an accepted direction. It separates v0.5 representation semantics from v0.6 measurement of indexes. The post-1.0 synthesis describes Statekit as a structural foundation and recommends integrating consumers before broadening its interface.

## 1. Verified current Statekit API and representation

### 1.1 Public API inventory

Current <code>src/lib.rs</code> publicly re-exports <code>MachineBuilder</code>, <code>StateError</code>, <code>MachineInstance</code>, <code>Machine</code>, and <code>Transition</code>. <code>StateName</code>, <code>MachineInner</code>, and <code>Transitions</code> are crate-private. All public types are non-generic.

| Public item | Current contract | Generic-identity impact |
|---|---|---|
| <code>Machine</code> | Non-generic struct holding <code>Arc&lt;MachineInner&gt;</code>; derives <code>Debug, PartialEq, Eq</code> | Would change type identity, bounds, equality, and APIs. |
| <code>Machine::builder()</code> | Returns <code>MachineBuilder</code> | Must choose a generic builder type; inference/default behavior must be proven. |
| <code>Machine::instance(&str)</code> | Returns <code>Result&lt;MachineInstance, StateError&gt;</code> | Must accept/resolve S and decide ownership/borrowing. |
| Lookups | <code>can_transition(&str,&str)</code>, <code>validate_transition(&str,&str)</code>, <code>contains_state(&str)</code>, <code>is_terminal(&str)</code> | Inputs and errors become custom-identity-aware or remain string-only. |
| Topology | <code>states()/sources() -&gt; impl Iterator&lt;Item=&amp;str&gt;</code>; <code>targets_from(&str)</code>; <code>transitions() -&gt; impl Iterator&lt;Item=&amp;Transition&gt;</code> | Generic transition storage changes iterator item types and bounds. |
| <code>MachineBuilder</code> | Non-generic; <code>try_allow(self, from: impl AsRef&lt;str&gt;, to: impl AsRef&lt;str&gt;) -&gt; Result&lt;Self, StateError&gt;</code>; <code>build(self) -&gt; Result&lt;Machine, StateError&gt;</code> | Needs an identity conversion/ownership contract. It consumes the builder on failure as well as success. |
| <code>Transition</code> | Stores private validated <code>StateName</code> endpoints; <code>source()/target() -&gt; &amp;str</code>; derives <code>Debug, Clone, PartialEq, Eq, Hash</code> | Likely becomes <code>Transition&lt;S&gt;</code> with <code>&amp;S</code> accessors. |
| <code>MachineInstance</code> | Holds <code>Arc&lt;MachineInner&gt;</code> and current <code>String</code>; <code>state() -&gt; &amp;str</code>; <code>transition_to(&amp;str)</code> | Generic state storage; borrowed transition may need S: Clone. |
| <code>StateError</code> | Non-generic <code>thiserror::Error + Debug + PartialEq + Eq</code>; string payloads for invalid name, self-edge, invalid edge, and unknown initial state | Custom IDs require generic payloads, lossy formatting, or different error variants. |
| Deprecated APIs | <code>MachineBuilder::allow</code> and <code>Machine::targets</code> remain public | Their signatures also need compatibility review. |

### 1.2 Verified behavior

- <code>StateName(String)</code> is a private validated wrapper. It rejects empty/whitespace-only values and leading/trailing Unicode whitespace; it preserves exact text and case.
- <code>Transition::try_new</code> validates each endpoint, rejects equal endpoints, then stores owned <code>StateName</code>s. The transition set is a <code>HashSet</code>; duplicate endpoint pairs collapse.
- Documented construction is <code>Machine::builder().try_allow("idle", "running")?.build()?</code>. Query inputs are exact and not normalized.
- Public state/transition iteration is borrowed; order is unspecified.
- <code>Machine</code> shares immutable inner storage through <code>Arc</code>. Each <code>MachineInstance</code> has an independent current string. Equality is logical, not pointer identity; a failed transition validates before mutation.
- These are current contracts; generic support must not weaken them for easier generic signatures.

### 1.3 Frozen performance baseline

The v0.4 baseline measures only the current string representation. For a 100,000-edge linear machine it reports approximately 62.1 ms build-and-drop, 146 µs for a missing direct-transition check, 645 µs for enumerating outgoing targets, and 25.9 ms for <code>states()</code>. The measured machine retained about 8.60 MB; the workload requested about 17.42 MB cumulatively during construction. These measurements justify representation/index experiments; they do not show that generic identity or numeric IDs improve performance. No new benchmark was run.

## 2. Rust feasibility: defaults, visibility, and inference

### 2.1 The exact proposed default cannot use today's private StateName

Rust rejects private types that leak through a public interface (E0446). <code>StateName</code> is currently crate-private. Therefore <code>pub struct Machine&lt;S = StateName&gt;</code> cannot be adopted exactly as written without making <code>StateName</code> publicly visible. Publicizing it requires a nameable exported type and decisions about whether external callers can construct it, convert from strings, inspect it, and use it in annotations. A private module does not give consumers a useful public generic argument.

Using <code>Machine&lt;S = String&gt;</code> avoids the private-type issue, but changes the default identity type and does not automatically retain the validated-wrapper model.

### 2.2 Defaults help, but do not guarantee compatibility

Rust permits default type parameters and uses type context to infer generic parameters. This can preserve declarations such as <code>let m: Machine = …</code> in some designs. It does not prove that every associated-function chain selects the intended specialization, or that a generic builder accepts existing literals through chosen conversion bounds. Compile exact chains externally against a prototype.

Illustrative only:

<pre><code>pub struct Machine&lt;S = StateName&gt; { /* ... */ }
pub struct MachineBuilder&lt;S = StateName&gt; { /* ... */ }
pub struct Transition&lt;S = StateName&gt; { /* ... */ }
pub struct MachineInstance&lt;S = StateName&gt; { /* ... */ }</code></pre>

This model still has unresolved inference. A generic <code>try_allow</code> may infer S from custom arguments, while strings may depend on the default fallback. A <code>TryInto&lt;S&gt;</code> signature introduces conversion-error typing. Explicit <code>Machine::&lt;MyId&gt;::builder()</code> is clear but makes custom use less transparent.

It is not possible to retain a generic <code>builder</code> method in both <code>impl Machine&lt;StateName&gt;</code> and <code>impl&lt;S&gt; Machine&lt;S&gt;</code> with identical names and overlapping inherent impls. A distinct generic constructor name or separate type is needed; duplicate associated items are rejected.

### 2.3 Compatibility classification

| Existing usage | Likely result under generic defaults | Caveat |
|---|---|---|
| <code>Machine::builder()</code> chain with strings | Could remain compatible if the default is public and conversions preserve &str input; unverified | Exact current <code>StateName</code> is private; chain needs compilation. |
| <code>Machine::&lt;MyId&gt;::builder()</code> | New explicit path | Requires generic builder and conversion/ownership contract. |
| <code>let m: Machine = …</code>; <code>fn f(m: Machine)</code> | Likely selects the default specialization | Defaults do not settle signatures of return values, errors, or iterators. |
| <code>machine.instance("idle")</code> | Potentially compatible only for default specialization | Custom identity requires different arguments/conversion. |
| <code>machine.states()</code> | May remain usable with default type | Item changes from <code>&amp;str</code> to <code>&amp;S</code> or wrapper; explicit annotations may break. |
| <code>machine.transitions()</code> / <code>&amp;Transition</code> | Default arguments may help | Public item type changes to <code>&amp;Transition&lt;S&gt;</code>. |
| <code>StateError</code> annotations/matches | Compatible only if error remains non-generic and represents custom IDs | Generic payloads expand signatures; formatting loses typed values. |

A default parameter may preserve some *spelling* of type paths. It does not make a generic public API source-neutral when return types, errors, bounds, or identity representation change.

## 3. Generic identity alternatives

| Approach | Strength | Cost / fit |
|---|---|---|
| **A. StateName-only public model** | Best string configuration ergonomics; current validation/errors | Advanced consumers adapt IDs; lowest complexity and recommended default. |
| **B. Public <code>Machine&lt;S=…&gt;</code>** | Direct enum/newtype IDs | Genericizes most public types and errors; inference/bounds/API complexity; require multiple consumer evidence. |
| **C. Generic structural core + string façade** | Protects the old public API | Duplicate concepts, errors, equality, docs, tests; may diverge. |
| **D. Additive generic constructor/adapter** | Existing API untouched; custom IDs possible through a separate route | Parallel API and conversion complexity; a separate name/type is safer than overlapping constructors. |
| **E. Consumer-owned mapping** | Domain identity stays with consumer; no Statekit change | Some boundary conversion/lookup cost; easiest to explain and maintain today. |
| **F. Private numeric lowering** | Allows internal indexed analysis/execution while preserving names | Does not remove custom-ID adapter cost; v0.6 benchmark question. |

Generic identity and internal indexing solve different problems. A generic <code>u32</code> model may still scan a set of edges. A private dense index may speed adjacency while public APIs remain string-based.

### Conversion and borrowing contract if reconsidered

- <code>From</code>/<code>Into</code> are for infallible conversion. Statekit name validation is fallible, so retain it through <code>TryFrom</code>/<code>TryInto</code> or a string-specific builder path.
- Construction needs owned S or fallible conversion to owned S. Don't bypass current validation to get a convenient signature.
- Lookup should accept borrowed identities to avoid clones. <code>Borrow&lt;Q&gt;</code> is appropriate for hash lookup only when Eq/Hash semantics match between S and Q; <code>AsRef&lt;Q&gt;</code> only exposes a view.
- Returning <code>&amp;S</code> preserves allocation-free iteration but changes current <code>&amp;str</code> contracts. Returning owned values adds clone cost.

## 4. Trait bounds, validation, and errors

| Need | Candidate bound | Assessment |
|---|---|---|
| Logical edge uniqueness/equality | <code>Eq</code>; <code>Hash</code> if using hashed storage | Likely required for a generic HashSet representation. |
| Debug output | Conditional <code>Debug</code> impl | Should not constrain all operations. |
| Human-readable error text | <code>Display</code> if formatting S | Avoid a global Display requirement only for diagnostics. |
| Stateful transition from borrowed target | <code>Clone</code> to store new current S | Could consume owned S but changes ownership ergonomics. |
| Thread-safe sharing | <code>Send + Sync</code> | Without these bounds some <code>Machine&lt;S&gt;</code> values lose current thread-sharing expectations. |
| Borrowed hash lookup | <code>Borrow&lt;Q&gt;</code>, <code>Q: Eq + Hash</code> | Requires equivalent hash/equality semantics. |

A custom Statekit identity trait is not justified by evidence. A further challenge is identity stability: interior mutability can change a stored value's Eq/Hash behavior, which generic bounds cannot prevent.

### Error implications

Current <code>StateError</code> stores strings for self-transition, invalid transition, unknown initial state, and invalid names. Custom identities force a choice:

1. Generic <code>StateError&lt;S&gt;</code> carrying S; this propagates generic errors and conditional std::error::Error bounds.
2. Format S to String; loses typed identity and requires Display/Debug.
3. Remove/change payloads; breaks useful matching.
4. Separate string-specific validation errors from generic topology errors; creates two public error contracts.

No option is a small signature-only change. Preserve INV-008 and existing useful payloads as a design constraint.

### Validation and equality semantics

A plausible generic meaning is that values equal under <code>S::Eq</code> identify the same logical state within a machine. Self-transition and duplicate-edge checks then use this equality. This preserves the *shape* of INV-002/004 but does not generalize the string-specific meaning of INV-005. The latter should remain a default string rule, not be imposed on numbers/enums.

Logical machine equality should compare edge sets under S equality, never insertion order or private slot order. This only works across machines when consumers provide IDs with the same logical meaning. If VPE assigns local integer slots by declaration order, equivalent graphs can compare unequal when state order changes. Instance equality must compare equal definitions and equal logical current identities. No cross-type comparison has evidence.

## 5. Project Core impact assessment

### Invariants

| Invariant | Generic model preservation requirement / tension |
|---|---|
| INV-001 Immutability | Preserve owned immutable S values; interior mutability remains consumer-controlled. |
| INV-002 No self transitions | Reject when S equality says endpoints match; retain earliest error. |
| INV-003 Nonempty transitions | Independent of S; preserve. |
| INV-004 Logical uniqueness | Use stable S Eq/Hash; no ID assignment/insertion order semantics. |
| INV-005 Case-sensitive identity | Keep for default string names; not meaningful for arbitrary S. Clarify Core if generic API is first-class. |
| INV-006 Runtime configuration | Preserve string construction as obvious default; custom identities require consumer parsing/resolution. |
| INV-007 Earliest error | Conversion/name/self errors should still occur in try_allow where detectable. |
| INV-008 Standard errors | Preserve typed matchability without imposing excessive bounds or losing identity payloads. |
| INV-009 Shared immutable machine | Generic S may need Send+Sync to keep cross-thread sharing. |
| INV-010 Valid traversal | All transitions must be checked using the same identity relation. |
| INV-011 Failed move atomicity | Validate before clone/replace of current S. |
| INV-012 Independent instances | Each instance has independent current S, shared immutable topology. |
| INV-013 Machine equality | Logical edge-set equality, independent of private slots/Arc. |
| INV-014 Instance equality | Equal machine definitions plus equal logical state; do not compare local slot indexes raw. |

No invariant needs to be weakened. If genericity proceeds, INV-005 needs clarification that its case-sensitive property applies to the default string identity; custom S identity follows its own Eq/Hash semantics. Also specify custom ID equality and stability separately.

### Intents, boundaries, and decisions

| Core element | Assessment |
|---|---|
| I-001 Runtime Use | Generic S can be runtime-defined but parsing/storage remain consumer responsibilities. |
| I-002 Small and Fast | Genericity does not automatically slow a fixed specialization, but expands docs, instantiations, and code paths; benefit unmeasured. |
| I-003 Ease of Use | Default construction must remain natural; inference and error typing risk friction. |
| I-004 Engineering Reference | Could be a design exercise, not sufficient reason by itself. |
| I-005 Data-Driven Definition | Strings fit current source/config use; generic IDs help already-resolved data only. |
| BI-001 Building Block | Fits if genericity remains topology only. |
| BI-002 Hidden Internal Storage | Public S/StateId is a semantic commitment, unlike private index. |
| BI-003 Concurrency | Generic values alter auto-traits; preserve Send/Sync contract explicitly or document scope. |
| D-002 Builder/Machine APIs | Can remain primary, but genericizing both broadens the primary interface. |
| D-005 External API testing | Must test unchanged old code from an external crate. |
| D-006 Fail during construction | Keep fallible default string conversion and immediate topology errors. |
| D-007 proptest | Test string and custom identity invariants/property model if introduced. |
| D-008 Criterion / D-009 memory | Compare with the frozen baseline; no measurements currently establish a benefit. |
| D-010 Mutable instances | Preserve independent S current values over shared immutable topology. |
| D-011 Performance tradeoff | Genericity is not proof of runtime improvement; construction/runtime/memory need measurements. |
| D-012 Runtime failures | Generic errors remain for invalid operations, not normal terminal states. |

## 6. Construction, iterators, and consumers

### Construction and runtime

A generic builder could own S and check topology as edges are added. The string specialization must still validate names before storage. <code>TryInto&lt;S&gt;</code> may need distinct error types for source and target conversions; generic custom IDs have no name-validation rule unless the consumer supplies one. Deferring lowering until build risks delaying known errors and retaining more input. Keep the current fail-fast consuming builder; generic identity is not a reason to redesign it.

A generic instance storing S and accepting <code>&amp;S</code> in <code>transition_to</code> needs to clone after validation. Taking S by value avoids a clone but changes the current call contract. A generic machine containing non-Send/non-Sync S may lose cross-thread sharing despite current documentation. These are public semantics questions.

### Borrowed transition iteration

Current <code>transitions()</code> yields <code>&amp;Transition</code>; accessors yield <code>&amp;str</code>. Generic storage naturally yields <code>&amp;Transition&lt;S&gt;</code> and <code>&amp;S</code>. Calls that only format/compare may still work with a suitable default type, but explicit annotations and helper APIs can break. Reconstructing references from numeric endpoint pairs needs retained name/transition storage. A separate generic model protects the current iterator but duplicates the abstraction.

### Consumer findings

- **Configuration-driven applications:** best-evidenced users already hold textual names; current validation and errors fit. Genericity adds no clear value unless they resolve names into custom IDs first.
- **Testing/model-based testing:** a test harness may use enum keys, but prior research found string projections sufficient. Test generation, shrinking, coverage, and execution remain the test framework's job.
- **VPE:** current compiler's <code>CompiledProcess</code> stores <code>initial_state_idx: usize</code>, <code>nodes: Vec&lt;Node&gt;</code>, and <code>state_index: HashMap&lt;String, usize&gt;</code>. Nodes retain names and vectors of edges; edges hold target index, action, priority, guards, effects. The compiler resolves names to indices; runtime request still supplies current state by string. Generic IDs could reduce some test-projection conversion, but VPE owns semantics.
- **VPE incompatibilities:** multiple semantic transitions can share endpoints and differ in action/guard/priority/effect; Statekit collapses endpoint duplicates. Statekit rejects self edges and requires at least one edge. Structural-only reachability cannot equal guard/priority-filtered or AUTO_TICK behavior. Genericity solves none of these.

A test-only VPE projection could resolve each compiled edge's target index to its Node name and call current string APIs. It must pre-handle empty topologies and self edges; a projection that omits them is intentionally partial. Retain VPE's semantic edges and test those independently. Do not add production dependency because a shared integer type exists.

## 7. Five worked examples

No example below was compiled in this environment. Current API examples are based on source; generic APIs are hypothetical.

### Example 1 — Equivalent machines, different insertion order

<pre><code>let left = Machine::builder()
    .try_allow("draft", "review")?
    .try_allow("review", "approved")?
    .build()?;
let right = Machine::builder()
    .try_allow("review", "approved")?
    .try_allow("draft", "review")?
    .build()?;
assert_eq!(left, right);</code></pre>

**Current:** logical set equality ignores insertion order. **Generic:** should compare logical S edge sets, not index slots. Declaration-order IDs can make equivalent graphs unequal unless consumer IDs are stable.

### Example 2 — String-to-ID lowering

<pre><code>"queued" -> "running" -> "done"
name map: queued=0, running=1, done=2
runtime pairs: (0,1), (1,2)</code></pre>

**Current API:** retains public names and borrowed transitions. **Hypothetical private lowering:** names remain public identity; internal slots serve indexes. **Hypothetical generic public API:** accepts consumer IDs but needs a separate label map for config, errors, display, and interchange. This is a v0.6 storage issue distinct from v0.5 identity.

### Example 3 — Independent instances

<pre><code>let machine = Machine::builder()
    .try_allow("ready", "running")?
    .try_allow("running", "done")?
    .build()?;
let mut a = machine.instance("ready")?;
let b = machine.instance("ready")?;
a.transition_to("running")?;
assert_eq!(a.state(), "running");
assert_eq!(b.state(), "ready");</code></pre>

**Current:** instances store separate current strings and share immutable topology. **Hypothetical generic:** each instance stores S; borrowed target transitions likely need Clone, while consuming S changes ownership ergonomics. If S is not Send+Sync, concurrent sharing may differ.

### Example 4 — Cross-machine ID misuse

<pre><code>Machine A order: [Draft, Approved] => 0=Draft, 1=Approved
Machine B order: [Approved, Draft] => 0=Approved, 1=Draft</code></pre>

A bare <code>StateId(0)</code> silently denotes different states across builds. Current string identity avoids this ambiguity. A public StateId must define machine scope, foreign-ID behavior, rebuild validity, and stability. A generation/token wrapper detects misuse but adds API/runtime cost. Generic S alone cannot make local indices globally stable.

### Example 5 — VPE structural projection

<pre><code>Node(0, "Draft") -- Submit, priority 10, guard G, effect E --> Node(1, "Approved")
Node(0, "Draft") -- Submit, priority  5, guard H, effect F --> Node(1, "Approved")</code></pre>

The Statekit projection has one edge <code>Draft -&gt; Approved</code>; VPE must retain both semantic edges. A VPE self-edge is rejected by Statekit, and a no-edge process cannot build. A <code>Machine&lt;VpeStateId&gt;</code> could reduce label conversion but cannot encode semantic edge identity or VPE rules. An oracle is valid only for compatible structural subsets.

## 8. Adoption friction and acceptance criteria

| Goal | Current API | Generic public model |
|---|---|---|
| No setup for string config | Strong | Must preserve default inference and fallible validation. |
| No conversion for advanced IDs | Host adapter required | Strong in principle if S flows through all APIs. |
| No representation knowledge | Strong | Weaker if IDs become public domain contract. |
| Clear typed errors | Strong string payloads | Requires generic error or lossy formatting. |
| Minimal trait burden | String only | Likely Eq+Hash, possibly Clone, Debug, Send+Sync, Borrow per method. |
| Borrowed iteration | &str / &Transition | Becomes &S / &Transition<S>, a public change. |
| One construction route | Machine::builder() | May need turbofish or a second constructor. |
| Equality across builds | Exact names | Requires stable consumer identity; local indices insufficient. |

Before accepting generic identity, require: unchanged external README examples compile; public type signatures have a minimal migration; custom enum/newtype works without unnecessary bounds; default string validation/errors remain identical; machine and instance equality ignore insertion/private layout; failure atomicity remains; Send/Sync expectations are preserved or scoped; ordinary workloads do not regress; and one concrete non-synthetic consumer materially benefits.

## 9. Capability classification

| Category | Findings |
|---|---|
| **A. Existing design remains sufficient** | Dynamic string definitions, early validation, immutable machine, equality, independent instances, borrowed topology iteration, reference-model projection. |
| **B. Internal-only optimization** | Private string-to-slot indexes/dense adjacency can be benchmarked without public genericity, subject to preserving public transition references/equality. |
| **C. Small reusable improvement** | Document consumer-owned enum/numeric mapping and external-crate API examples; no new API justified today. |
| **D. Future research** | Generic identity after another consumer; error and thread-safety contracts; duplicate generic-core maintenance; benchmark generic hashing vs strings. |
| **E. Consumer-owned** | Parsing, symbol resolution, ID-label mapping, stable identity across builds, semantic transitions and VPE behavior. |
| **F. Out of scope** | Global/distributed Statekit IDs, identity registry, compiler framework, arbitrary graph mutation, business semantics. |

## 10. Architecture decision matrix

Qualitative expectations only; no generic implementation was measured.

| Criterion | Current string API | Public generic Machine | Generic core + façade | Additive generic type | Consumer map | Private lowering |
|---|---|---|---|---|---|---|
| Source compatibility | High | Low–medium | High for old facade | High for old API | High | High |
| Ordinary ergonomics | High | Medium if defaults work | High | High for strings | High | High |
| Advanced ID ergonomics | Low | High | High | Medium–high | Medium | Low–medium |
| String validation | High | Needs default path | High in facade | High in existing path | High | High |
| Equality semantics | High | Needs S contract | Two models | Multiple models | Consumer-defined | Can remain name-based |
| API/bound/error complexity | Low | High | High | Medium–high | None in Statekit | Low |
| Borrowed iterator compatibility | High | Risky | Old facade can preserve | Separate types | High | Must preserve refs |
| Runtime/memory | Measured baseline | Unknown; S ≠ indexing | Duplicate-representation risk | Unknown | Map cost at consumer | Potential gains; measure |
| VPE benefit | Projection by strings | Reuse some IDs, mismatch remains | Same mismatch | Test adapter | Already has map | Statekit-only optimization |
| Long-term maintenance | Lowest | Highest | High | Medium–high | Lowest Statekit burden | Medium |

The strongest alternative to the recommendation is a separate generic structural type, because it preserves existing source usage while allowing resolved identities. Its weakness is parallel APIs. Evidence that would change this recommendation: two independent consumers with stable IDs, a demonstrably smaller adapter using generic Statekit, passing external compile tests, and no material ordinary-user/API or performance regression.

## 11. Ranked recommendations and decision gates

1. **Retain StateName as sole public logical identity for now — recommend.** Best matches runtime text definitions, current errors, consumer evidence, and Core.
2. **Document a consumer-owned mapping — recommend, no API change.** Make mappings bijective and retained by the host; compare after mapping. This both lowers friction and tests whether it is genuinely burdensome.
3. **Prototype private lowering independently — defer to v0.6 evidence.** Compare current and indexed implementations on same workloads, preserving names, errors, equality, and public transition iterators.
4. **Revisit a separate generic model if another consumer appears — future research.** Count duplicated docs/tests/errors/equality as costs.
5. **Do not expose Statekit StateId now — exclude.** No requirement for stable machine-scoped handles has been demonstrated.

Decision gates:

- Keep strings while consumer APIs remain natural and performance can be addressed internally.
- Add private dense slots only if measured wins justify maps and retained memory.
- Add generic identity only after all acceptance criteria pass and a real consumer benefits beyond a toy example.
- Expose StateId only after consumers need opaque handles across calls and scope/foreign/rebuild rules are specified.
- Keep v0.5 identity semantics distinct from v0.6 optimization; v0.5 may simply affirm strings as logical identity and leave private layout open.

## 12. Compatibility experiment suite and results

### Execution result

The environment check found no <code>rustc</code> or <code>cargo</code> in PATH or common installation paths. Experiments were **not run**. Outcomes below are test goals/predictions, not compiler claims.

| Experiment | Test | Outcome here | Decision value |
|---|---|---|---|
| 1. Existing API compilation | Unchanged README code; external signatures taking Machine and borrowed Transition | Not run; current source examples inspected | Whether default specialization preserves type paths and iterators. |
| 2. Custom identity | Enum and numeric newtype with Eq+Hash; construct/query/enumerate/traverse | Not run | Whether Clone/Debug/Display/Send+Sync/custom trait are over-required. |
| 3. Conversion behavior | TryFrom for validated name, Into for infallible newtype, TryInto error handling | Not run; official docs reviewed | Whether conversion contracts compose. |
| 4. Type inference | Machine::builder(), explicit default, explicit custom type, let m: Machine | Not run; exact current default type is private | Whether defaults select strings and custom S infers naturally. |
| 5. Iterator compatibility | Assign states item to &str; annotate &Transition; enumerate custom refs | Not run | Whether public borrowed contract survives. |
| 6. Invariants | Property tests: self/duplicate/equality/failure atomicity/instance independence | Not run | Whether behavior matches string specialization. |
| 7. External crate | Compile consumer using only public exports | Not run | Catches private type leakage and source changes. |
| 8. Error ergonomics | Match variants and payloads with IDs lacking Display | Not run | Measures error complexity and bounds. |

Test sketch (hypothetical candidate API; not compile-verified):

<pre><code>let strings = Machine::builder()
    .try_allow("idle", "busy")?
    .build()?;

let phases = Machine::&lt;Phase&gt;::builder()
    .try_allow(Phase::Idle, Phase::Busy)?
    .build()?;

let _: Option&lt;&amp;str&gt; = strings.states().next();
let _: Option&lt;&amp;Transition&gt; = strings.transitions().next();
let _: &amp;Phase = phases.instance(Phase::Idle)?.state();</code></pre>

Add compile-fail cases for private StateName leakage and duplicate inherent builders. Run existing proptests for strings, then the same property model for numeric newtypes. Test Send+Sync assertions and exact error matching.

## 13. Minimal next experiment and follow-up

### Smallest high-value experiment

In a disposable Rust crate, compile two API prototypes side by side:

1. Existing <code>Machine</code> string API with private lowering only.
2. Explicitly named generic type (for example, hypothetical <code>IdentityMachine&lt;S&gt;</code>) with owned custom IDs, string specialization, and borrowed <code>&amp;S</code> queries.

Compile unchanged README use, an external signature using public iterators/errors, and a VPE-like <code>usize</code> projection. Do not modify production code or accept a roadmap change. API friction comes first; benchmark only after both variants satisfy invariants.

Future tracks: v0.5 identity semantics; v0.6 name-keyed vs dense private index benchmarks; a VPE test-only projection enumerating unsupported topologies and adapter cost; and a second independent compiler/runtime consumer with a stable typed identity.

## 14. Risks and unresolved questions

1. Would Statekit make StateName public or use public String as the default?
2. Can string-specialized construction retain &str plus validation while a generic builder accepts custom S without overlapping method names?
3. Should errors carry generic S, stringify IDs, or split string validation and topology errors?
4. Should S require Send+Sync to preserve current shared-machine expectations?
5. How should identity stability under interior mutability be documented?
6. Is Clone acceptable for borrowed instance transitions, or should generic operations consume values?
7. Are consumer IDs stable across machine builds, or are they local indices?
8. Which independent consumer has repeated measurable conversion costs?
9. Does a v0.5 generic API constrain v0.6 implementation choices or post-1.0 compatibility more than it helps?

## Primary sources

### Statekit

- [Statekit repository](https://github.com/whitephoenixsoft/statekit)
- [Current src/lib.rs](https://github.com/whitephoenixsoft/statekit/blob/main/src/lib.rs)
- [Current src/builder.rs](https://github.com/whitephoenixsoft/statekit/blob/main/src/builder.rs)
- [Current src/machine.rs](https://github.com/whitephoenixsoft/statekit/blob/main/src/machine.rs)
- [Current src/instance.rs](https://github.com/whitephoenixsoft/statekit/blob/main/src/instance.rs)
- [Current src/transition.rs](https://github.com/whitephoenixsoft/statekit/blob/main/src/transition.rs)
- [Current src/state_name.rs](https://github.com/whitephoenixsoft/statekit/blob/main/src/state_name.rs)
- [Current src/error.rs](https://github.com/whitephoenixsoft/statekit/blob/main/src/error.rs)
- User-supplied Project Core v0.4, Statekit specification v0.4, README, roadmap, and frozen v0.4 benchmark.

### Prior research

- [Configuration-driven consumer research](sandbox:/workspace/scratch/3524c4ebe062/statekit-consumer-research.md)
- [Reference-model research](sandbox:/workspace/scratch/3524c4ebe062/statekit-reference-model-research.md)
- [Structural IR/interchange research](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-ir-interchange-research.md)
- [Structural analysis research](sandbox:/workspace/scratch/3524c4ebe062/statekit-structural-analysis-algorithms-research.md)
- [Validation diagnostics research](sandbox:/workspace/scratch/3524c4ebe062/statekit-validation-diagnostics-research.md)
- [Runtime identity research](sandbox:/workspace/scratch/3524c4ebe062/statekit-runtime-state-identity-research.md)
- Post-1.0 structural analysis/engine integration/Cargo features synthesis and draft post-1.0 roadmap.

### VPE

- [Verified Process Engine repository](https://github.com/whitephoenixsoft/verified-process-engine)
- [Compiler compile.rs](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compile.rs)
- [Compiled representation compiled.rs](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/compiled.rs)
- [Compiler validation validate.rs](https://github.com/whitephoenixsoft/verified-process-engine/blob/main/crates/vpe/src/compiler/validate.rs)

### Rust

- [Rust Reference: generic parameters](https://doc.rust-lang.org/reference/items/generics.html)
- [Rust E0446: private type in public interface](https://doc.rust-lang.org/stable/error_codes/E0446.html)
- [Rust E0201: duplicate associated items](https://doc.rust-lang.org/error_codes/E0201.html)
- [From: use TryFrom for fallible conversions](https://doc.rust-lang.org/std/convert/trait.From.html)
- [TryInto](https://doc.rust-lang.org/std/convert/trait.TryInto.html)
- [Borrow: equivalent Eq/Hash semantics](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)

---

**Conclusion:** Generic identity could remove an adapter for some advanced consumers, but Rust makes the compatibility surface much larger than a default parameter suggests. Current evidence favors retaining the simple validated string API and measuring private representation improvements independently. Reconsider generic identity only after a real consumer demonstrates material benefit and the external-crate experiment suite passes.
