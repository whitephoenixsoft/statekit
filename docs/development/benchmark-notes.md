
## performance 

Add Throughput only to benchmarks that do full scans In the external representation of the API. This means that if the API seems like a singular function, and I'm only calling it once, I should not add it. 

For example:
A machine takes multiple state names as parameters. The parameter size increase is with the benchmark. This will get us throughput if it's a full scan. 

The instance for the machine still takes only one machine despite the size of the machine. So even if the instance ends up doing a full scan through the machine, it will not get us throughput because there are not multiple instances.

>If we later benchmark an operation where each iteration genuinely processes N logical items, that's where Throughput::Elements(N) belongs. 

## memory

Total allocated bytes
    How many bytes were requested from the allocator over an operation's lifetime.

Allocation count
    How many allocations occurred.

Peak live bytes
    Maximum simultaneously allocated heap memory.

Retained/live bytes
    Heap memory still owned after construction/operation completes.
    
### statistics 
Machine retained memory
Machine construction allocation volume
Machine construction peak memory
Machine construction allocation count
Instance incremental retained memory
N instances incremental retained memory

### questions
Retained
"How expensive is a Machine to keep?"

Peak
"How much heap pressure can construction create at once?"

Total allocated
"How much allocator traffic did construction cause?"

Allocation count
"How many individual allocations did construction perform?"

### counters 
ALLOCATED       cumulative bytes ever allocated
DEALLOCATED     cumulative bytes ever deallocated
LIVE_BYTES      bytes currently alive
PEAK_LIVE_BYTES highest LIVE_BYTES we've observed

retained_bytes
    How much heap does the finished Machine retain?

allocated_bytes
    How many cumulative bytes flowed through allocation/reallocation
    during construction?

allocation_count
    How many allocation/reallocation requests occurred?
### output 
Statekit memory benchmark

=== Machine Retained Memory ===

 transitions    retained       bytes/transition
         100       7.95 KB             79.50
       1,000     118.21 KB            118.21
...

