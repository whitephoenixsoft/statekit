
## performance 

Add Throughput only to benchmarks that do full scans

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

