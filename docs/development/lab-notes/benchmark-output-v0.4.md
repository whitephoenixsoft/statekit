Running benches/machine_memory.rs (target/release/deps/machine_memory-234d867f2682a7b3)
Statekit memory benchmark

=== Machine Retained Memory ===

transitions     retained        bytes/transition
        100     7926            79.26
       1000     118207          118.21
      10000     1000672         100.07
     100000     8600385         86.00

=== Machine Allocated Memory ===

transitions     allocated       bytes/transition
        100     16442           164.42
       1000     242435          242.44
      10000     2043388         204.34
     100000     17422837        174.23

=== Machine Allocation Count ===

transitions     count   allocations/transition
        100     407             4.07
       1000     4011            4.01
      10000     40014           4.00
     100000     400017          4.00

=== Machine Peak Construction Memory ===

transitions     peak additional bytes/transition
        100     10317           103.17
       1000     166472          166.47
      10000     1345403         134.54
     100000     10873182        108.73


MachineInstance size: 32 bytes

=== Instance Retained Memory ===

transitions     instances       retained        bytes/instance
        100     1               39              39.00
        100     100             3900            39.00
        100     1000            39000           39.00

       1000     1               39              39.00
       1000     100             3900            39.00
       1000     1000            39000           39.00

      10000     1               39              39.00
      10000     100             3900            39.00
      10000     1000            39000           39.00

     100000     1               39              39.00
     100000     100             3900            39.00
     100000     1000            39000           39.00


=== Instance Allocated Memory ===

transitions     instances       allocated       bytes/instance
        100     1               39              39.00
        100     100             3900            39.00
        100     1000            39000           39.00

       1000     1               39              39.00
       1000     100             3900            39.00
       1000     1000            39000           39.00

      10000     1               39              39.00
      10000     100             3900            39.00
      10000     1000            39000           39.00

     100000     1               39              39.00
     100000     100             3900            39.00
     100000     1000            39000           39.00


=== Instance Allocation Count ===

transitions     instances       count   allocations/instance
        100     1               2               2.00
        100     100             101             1.01
        100     1000            1001            1.00

       1000     1               2               2.00
       1000     100             101             1.01
       1000     1000            1001            1.00

      10000     1               2               2.00
      10000     100             101             1.01
      10000     1000            1001            1.00

     100000     1               2               2.00
     100000     100             101             1.01
     100000     1000            1001            1.00


=== Instance Peak Construction Memory ===

transitions     instances       peak additional bytes/instance
        100     1               39              39.00
        100     100             3900            39.00
        100     1000            39000           39.00

       1000     1               39              39.00
       1000     100             3900            39.00
       1000     1000            39000           39.00

      10000     1               39              39.00
      10000     100             3900            39.00
      10000     1000            39000           39.00

     100000     1               39              39.00
     100000     100             3900            39.00
     100000     1000            39000           39.00

Running benches/machine_queries.rs (target/release/deps/machine_queries-974528a11bbfac4c)
Gnuplot not found, using plotters backend
Benchmarking can_transition/existing/100: WarmingBenchmarking can_transition/existing/100: Collecting 100 samples in estimated 5.0001 s (150M iteraBenchmarking can_transition/existing/100: Analyzican_transition/existing/100
                        time:   [33.035 ns 33.052 ns 33.067 ns]
                        change: [+562.11% +563.07% +563.97%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high severe
Benchmarking can_transition/existing/1000: WarminBenchmarking can_transition/existing/1000: Collecting 100 samples in estimated 5.0062 s (3.2M iterBenchmarking can_transition/existing/1000: Analyzcan_transition/existing/1000
                        time:   [1.5463 µs 1.5473 µs 1.5484 µs]
                        change: [−21.770% −20.836% −20.311%] (p = 0.00 < 0.05)
                        Performance has improved.Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high severe
Benchmarking can_transition/existing/10000: WarmiBenchmarking can_transition/existing/10000: Collecting 100 samples in estimated 5.0256 s (268k iteBenchmarking can_transition/existing/10000: Analycan_transition/existing/10000
                        time:   [18.857 µs 18.869 µs 18.883 µs]
                        change: [+23.484% +23.811% +24.187%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low severe
  2 (2.00%) high mild
  3 (3.00%) high severe
Benchmarking can_transition/existing/100000: WarmBenchmarking can_transition/existing/100000: Collecting 100 samples in estimated 5.0603 s (192k itBenchmarking can_transition/existing/100000: Analcan_transition/existing/100000
                        time:   [26.572 µs 26.600 µs 26.634 µs]
                        change: [+960.99% +966.22% +972.11%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe

Benchmarking can_transition/missing/100: Warming Benchmarking can_transition/missing/100: Collecting 100 samples in estimated 5.0003 s (76M iteratiBenchmarking can_transition/missing/100: Analyzincan_transition/missing/100
                        time:   [64.805 ns 64.873 ns 64.942 ns]
                        thrpt:  [1.5398 Gelem/s 1.5415 Gelem/s 1.5431 Gelem/s]
                 change:
                        time:   [+2.1801% +2.5324% +2.8838%] (p = 0.00 < 0.05)
                        thrpt:  [−2.8030% −2.4699% −2.1336%]
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
Benchmarking can_transition/missing/1000: WarmingBenchmarking can_transition/missing/1000: Collecting 100 samples in estimated 5.0014 s (7.1M iteraBenchmarking can_transition/missing/1000: Analyzican_transition/missing/1000
                        time:   [695.93 ns 696.42 ns 696.93 ns]
                        thrpt:  [1.4349 Gelem/s 1.4359 Gelem/s 1.4369 Gelem/s]
                 change:
                        time:   [−4.2062% −3.8854% −3.5737%] (p = 0.00 < 0.05)
                        thrpt:  [+3.7062% +4.0424% +4.3909%]
                        Performance has improved.Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
Benchmarking can_transition/missing/10000: WarminBenchmarking can_transition/missing/10000: Collecting 100 samples in estimated 5.0028 s (712k iterBenchmarking can_transition/missing/10000: Analyzcan_transition/missing/10000
                        time:   [7.2067 µs 7.2113 µs 7.2167 µs]
                        thrpt:  [1.3857 Gelem/s 1.3867 Gelem/s 1.3876 Gelem/s]
                 change:
                        time:   [−0.5555% −0.3149% −0.0850%] (p = 0.01 < 0.05)
                        thrpt:  [+0.0850% +0.3159% +0.5586%]
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking can_transition/missing/100000: WarmiBenchmarking can_transition/missing/100000: Collecting 100 samples in estimated 5.1482 s (35k iterBenchmarking can_transition/missing/100000: Analycan_transition/missing/100000
                        time:   [145.08 µs 145.53 µs 145.99 µs]
                        thrpt:  [684.98 Melem/s 687.15 Melem/s 689.26 Melem/s]
                 change:
                        time:   [−2.7776% −1.9677% −1.2468%] (p = 0.00 < 0.05)
                        thrpt:  [+1.2626% +2.0072% +2.8569%]
                        Performance has improved.Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking targets_from/existing/100: Warming uBenchmarking targets_from/existing/100: Collecting 100 samples in estimated 10.001 s (54M iteratiotargets_from/existing/100                                                time:   [182.75 ns 182.88 ns 183.04 ns]
                        thrpt:  [546.34 Melem/s 546.80 Melem/s 547.20 Melem/s]
                 change:
                        time:   [+1.3006% +1.6370% +2.0062%] (p = 0.00 < 0.05)
                        thrpt:  [−1.9668% −1.6107% −1.2839%]
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking targets_from/existing/1000: Warming Benchmarking targets_from/existing/1000: Collecting 100 samples in estimated 10.006 s (3.4M iteratBenchmarking targets_from/existing/1000: Analyzintargets_from/existing/1000
                        time:   [2.9661 µs 2.9673 µs 2.9688 µs]
                        thrpt:  [336.83 Melem/s 337.00 Melem/s 337.15 Melem/s]
                 change:
                        time:   [+20.461% +21.079% +21.695%] (p = 0.00 < 0.05)
                        thrpt:  [−17.827% −17.409% −16.986%]
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking targets_from/existing/10000: WarmingBenchmarking targets_from/existing/10000: Collecting 100 samples in estimated 10.102 s (374k iteraBenchmarking targets_from/existing/10000: Analyzitargets_from/existing/10000
                        time:   [26.617 µs 26.637 µs 26.659 µs]
                        thrpt:  [375.11 Melem/s 375.42 Melem/s 375.70 Melem/s]
                 change:
                        time:   [+3.0685% +3.4111% +3.7357%] (p = 0.00 < 0.05)
                        thrpt:  [−3.6012% −3.2986% −2.9771%]
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  2 (2.00%) high severe
Benchmarking targets_from/existing/100000: WarminBenchmarking targets_from/existing/100000: Collecting 100 samples in estimated 13.007 s (20k iteraBenchmarking targets_from/existing/100000: Analyztargets_from/existing/100000
                        time:   [642.67 µs 644.65 µs 646.55 µs]
                        thrpt:  [154.67 Melem/s 155.12 Melem/s 155.60 Melem/s]
                 change:
                        time:   [+4.9793% +5.6439% +6.2730%] (p = 0.00 < 0.05)
                        thrpt:  [−5.9027% −5.3424% −4.7431%]
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild

Benchmarking targets_from/missing/100: Warming upBenchmarking targets_from/missing/100: Collecting 100 samples in estimated 10.001 s (52M iterationtargets_from/missing/100
                        time:   [185.64 ns 185.79 ns 185.96 ns]
                        thrpt:  [537.75 Melem/s 538.26 Melem/s 538.68 Melem/s]
                 change:
                        time:   [+2.5111% +2.9150% +3.2841%] (p = 0.00 < 0.05)
                        thrpt:  [−3.1796% −2.8325% −2.4496%]
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
Benchmarking targets_from/missing/1000: Warming uBenchmarking targets_from/missing/1000: Collecting 100 samples in estimated 10.005 s (3.4M iteratitargets_from/missing/1000                                                time:   [2.9927 µs 2.9948 µs 2.9968 µs]
                        thrpt:  [333.69 Melem/s 333.91 Melem/s 334.15 Melem/s]
                 change:
                        time:   [+5.4043% +5.6793% +5.9651%] (p = 0.00 < 0.05)
                        thrpt:  [−5.6293% −5.3741% −5.1272%]
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Benchmarking targets_from/missing/10000: Warming Benchmarking targets_from/missing/10000: Collecting 100 samples in estimated 10.109 s (374k iteratBenchmarking targets_from/missing/10000: Analyzintargets_from/missing/10000
                        time:   [26.424 µs 26.461 µs 26.507 µs]
                        thrpt:  [377.27 Melem/s 377.92 Melem/s 378.45 Melem/s]
                 change:
                        time:   [+0.8796% +1.3234% +1.7177%] (p = 0.00 < 0.05)
                        thrpt:  [−1.6887% −1.3061% −0.8719%]
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
Benchmarking targets_from/missing/100000: WarmingBenchmarking targets_from/missing/100000: Collecting 100 samples in estimated 13.253 s (20k iteratBenchmarking targets_from/missing/100000: Analyzitargets_from/missing/100000
                        time:   [656.46 µs 658.61 µs 660.65 µs]
                        thrpt:  [151.37 Melem/s 151.83 Melem/s 152.33 Melem/s]
                 change:
                        time:   [+6.0512% +6.6290% +7.2315%] (p = 0.00 < 0.05)
                        thrpt:  [−6.7438% −6.2168% −5.7059%]
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild

Benchmarking sources/100: Collecting 100 samples sources/100             time:   [3.8503 µs 3.8529 µs 3.8560 µs]
                        thrpt:  [25.934 Melem/s 25.954 Melem/s 25.972 Melem/s]
                 change:
                        time:   [+4.8769% +5.1641% +5.4313%] (p = 0.00 < 0.05)
                        thrpt:  [−5.1515% −4.9105% −4.6501%]
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Benchmarking sources/1000: Warming up for 3.0000 Benchmarking sources/1000: Collecting 100 samplessources/1000            time:   [52.182 µs 52.206 µs 52.231 µs]
                        thrpt:  [19.146 Melem/s 19.155 Melem/s 19.164 Melem/s]
                 change:
                        time:   [+5.2022% +5.3279% +5.4619%] (p = 0.00 < 0.05)
                        thrpt:  [−5.1790% −5.0584% −4.9449%]
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking sources/10000: Warming up for 3.0000Benchmarking sources/10000: Collecting 100 samplesources/10000           time:   [498.14 µs 498.65 µs 499.17 µs]
                        thrpt:  [20.033 Melem/s 20.054 Melem/s 20.075 Melem/s]
                 change:
                        time:   [−3.7506% −3.4642% −3.1901%] (p = 0.00 < 0.05)
                        thrpt:  [+3.2952% +3.5885% +3.8967%]
                        Performance has improved.Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking sources/100000: Warming up for 3.000Benchmarking sources/100000: Collecting 100 samplsources/100000          time:   [8.3398 ms 8.5192 ms 8.7070 ms]
                        thrpt:  [11.485 Melem/s 11.738 Melem/s 11.991 Melem/s]
                 change:
                        time:   [−5.1576% −2.7384% −0.3875%] (p = 0.03 < 0.05)
                        thrpt:  [+0.3890% +2.8155% +5.4381%]
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking states/100: Collecting 100 samples istates/100              time:   [5.5587 µs 5.6268 µs 5.6887 µs]
                        thrpt:  [17.579 Melem/s 17.772 Melem/s 17.990 Melem/s]
                 change:
                        time:   [+3.7871% +4.3575% +4.9204%] (p = 0.00 < 0.05)
                        thrpt:  [−4.6896% −4.1756% −3.6489%]
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)  1 (1.00%) low severe
  16 (16.00%) high severe
Benchmarking states/1000: Collecting 100 samples states/1000             time:   [78.648 µs 78.690 µs 78.732 µs]
                        thrpt:  [12.701 Melem/s 12.708 Melem/s 12.715 Melem/s]
                 change:
                        time:   [+10.789% +10.909% +11.016%] (p = 0.00 < 0.05)
                        thrpt:  [−9.9225% −9.8356% −9.7381%]
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
Benchmarking states/10000: Warming up for 3.0000 Benchmarking states/10000: Collecting 100 samplesstates/10000            time:   [835.90 µs 843.94 µs 852.79 µs]
                        thrpt:  [11.726 Melem/s 11.849 Melem/s 11.963 Melem/s]
                 change:
                        time:   [+10.841% +11.404% +12.020%] (p = 0.00 < 0.05)
                        thrpt:  [−10.730% −10.237% −9.7804%]
                        Performance has regressed.
Found 19 outliers among 100 measurements (19.00%)  16 (16.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high mild
Benchmarking states/100000: Warming up for 3.0000Benchmarking states/100000: Collecting 100 samplestates/100000           time:   [25.588 ms 25.904 ms 26.232 ms]
                        thrpt:  [3.8121 Melem/s 3.8603 Melem/s 3.9081 Melem/s]
                 change:
                        time:   [+21.804% +23.811% +25.759%] (p = 0.00 < 0.05)
                        thrpt:  [−20.483% −19.231% −17.901%]
                        Performance has regressed.

Benchmarking transitions/100: Warming up for 3.00Benchmarking transitions/100: Collecting 100 samptransitions/100         time:   [64.832 ns 64.932 ns 65.052 ns]
                        thrpt:  [1.5372 Gelem/s 1.5401 Gelem/s 1.5425 Gelem/s]
                 change:
                        time:   [+4.4351% +4.6346% +4.8380%] (p = 0.00 < 0.05)
                        thrpt:  [−4.6147% −4.4293% −4.2468%]
                        Performance has regressed.
Found 20 outliers among 100 measurements (20.00%)  1 (1.00%) low mild
  8 (8.00%) high mild
  11 (11.00%) high severe
Benchmarking transitions/1000: Warming up for 3.0Benchmarking transitions/1000: Collecting 100 samtransitions/1000        time:   [629.95 ns 630.13 ns 630.32 ns]
                        thrpt:  [1.5865 Gelem/s 1.5870 Gelem/s 1.5874 Gelem/s]
                 change:
                        time:   [−0.3436% −0.1447% +0.0313%] (p = 0.14 > 0.05)
                        thrpt:  [−0.0313% +0.1449% +0.3448%]
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking transitions/10000: Warming up for 3.Benchmarking transitions/10000: Collecting 100 satransitions/10000       time:   [6.1923 µs 6.1934 µs 6.1946 µs]
                        thrpt:  [1.6143 Gelem/s 1.6146 Gelem/s 1.6149 Gelem/s]
                 change:
                        time:   [−3.4752% −3.3315% −3.1994%] (p = 0.00 < 0.05)
                        thrpt:  [+3.3051% +3.4464% +3.6003%]
                        Performance has improved.Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
Benchmarking transitions/100000: Warming up for 3Benchmarking transitions/100000: Collecting 100 stransitions/100000      time:   [98.591 µs 98.623 µs 98.660 µs]
                        thrpt:  [1.0136 Gelem/s 1.0140 Gelem/s 1.0143 Gelem/s]
                 change:
                        time:   [−3.2934% −3.2170% −3.1451%] (p = 0.00 < 0.05)
                        thrpt:  [+3.2472% +3.3239% +3.4055%]
                        Performance has improved.Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Benchmarking contains_state/existing_source/100: Benchmarking contains_state/existing_source/100: Collecting 100 samples in estimated 5.0000 s (480Benchmarking contains_state/existing_source/100: contains_state/existing_source/100
                        time:   [11.198 ns 11.222 ns 11.240 ns]
                        change: [−90.157% −90.091% −90.024%] (p = 0.00 < 0.05)
                        Performance has improved.Benchmarking contains_state/existing_source/1000:Benchmarking contains_state/existing_source/1000: Collecting 100 samples in estimated 5.0019 s (7.Benchmarking contains_state/existing_source/1000:contains_state/existing_source/1000
                        time:   [654.64 ns 661.47 ns 667.40 ns]
                        change: [+2147.3% +2160.4% +2175.7%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 31 outliers among 100 measurements (31.00%)  10 (10.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  19 (19.00%) high severe
Benchmarking contains_state/existing_source/10000Benchmarking contains_state/existing_source/10000: Collecting 100 samples in estimated 5.0169 s (2Benchmarking contains_state/existing_source/10000contains_state/existing_source/10000
                        time:   [9.7029 µs 9.7050 µs 9.7076 µs]
                        change: [+174.97% +175.21% +175.43%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)  1 (1.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe
Benchmarking contains_state/existing_source/10000Benchmarking contains_state/existing_source/10000Benchmarking contains_state/existing_source/100000: Collecting 100 samples in estimated 5.8099 s (Benchmarking contains_state/existing_source/10000contains_state/existing_source/100000
                        time:   [135.58 µs 136.14 µs 136.75 µs]
                        change: [+8288.1% +8513.6% +8776.4%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)  1 (1.00%) high mild
  12 (12.00%) high severe

Benchmarking contains_state/target_only/100: WarmBenchmarking contains_state/target_only/100: Collecting 100 samples in estimated 5.0001 s (72M iteBenchmarking contains_state/target_only/100: Analcontains_state/target_only/100
                        time:   [69.279 ns 69.414 ns 69.539 ns]
                        change: [−4.1602% −3.8291% −3.4981%] (p = 0.00 < 0.05)
                        Performance has improved.Benchmarking contains_state/target_only/1000: WarBenchmarking contains_state/target_only/1000: Collecting 100 samples in estimated 5.0004 s (9.1M iBenchmarking contains_state/target_only/1000: Anacontains_state/target_only/1000
                        time:   [540.27 ns 542.46 ns 544.24 ns]
                        change: [+16.385% +17.037% +17.718%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)  8 (8.00%) low severe
  5 (5.00%) low mild
Benchmarking contains_state/target_only/10000: WaBenchmarking contains_state/target_only/10000: Collecting 100 samples in estimated 5.0412 s (434k Benchmarking contains_state/target_only/10000: Ancontains_state/target_only/10000
                        time:   [7.6069 µs 7.6108 µs 7.6155 µs]
                        change: [+225.71% +226.68% +227.70%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
Benchmarking contains_state/target_only/100000: WBenchmarking contains_state/target_only/100000: Collecting 100 samples in estimated 5.0281 s (111kBenchmarking contains_state/target_only/100000: Acontains_state/target_only/100000
                        time:   [46.011 µs 46.138 µs 46.274 µs]
                        change: [+260.95% +262.20% +263.56%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

Benchmarking contains_state/missing/100: Warming Benchmarking contains_state/missing/100: Collecting 100 samples in estimated 5.0005 s (37M iteratiBenchmarking contains_state/missing/100: Analyzincontains_state/missing/100
                        time:   [134.74 ns 134.76 ns 134.78 ns]
                        thrpt:  [741.95 Melem/s 742.07 Melem/s 742.18 Melem/s]
                 change:
                        time:   [+37.066% +37.623% +38.208%] (p = 0.00 < 0.05)
                        thrpt:  [−27.645% −27.338% −27.042%]
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high severe
Benchmarking contains_state/missing/1000: WarmingBenchmarking contains_state/missing/1000: Collecting 100 samples in estimated 5.0048 s (3.8M iteraBenchmarking contains_state/missing/1000: Analyzicontains_state/missing/1000
                        time:   [1.2322 µs 1.2378 µs 1.2428 µs]
                        thrpt:  [804.62 Melem/s 807.85 Melem/s 811.54 Melem/s]
                 change:
                        time:   [+12.530% +13.197% +13.833%] (p = 0.00 < 0.05)
                        thrpt:  [−12.152% −11.658% −11.135%]
                        Performance has regressed.
Benchmarking contains_state/missing/10000: WarminBenchmarking contains_state/missing/10000: Collecting 100 samples in estimated 5.0603 s (348k iterBenchmarking contains_state/missing/10000: Analyzcontains_state/missing/10000
                        time:   [11.872 µs 11.971 µs 12.064 µs]
                        thrpt:  [828.93 Melem/s 835.33 Melem/s 842.29 Melem/s]
                 change:
                        time:   [+10.451% +11.424% +12.246%] (p = 0.00 < 0.05)
                        thrpt:  [−10.910% −10.253% −9.4617%]
                        Performance has regressed.
Benchmarking contains_state/missing/100000: WarmiBenchmarking contains_state/missing/100000: Collecting 100 samples in estimated 5.7235 s (30k iterBenchmarking contains_state/missing/100000: Analycontains_state/missing/100000
                        time:   [185.19 µs 186.91 µs 188.76 µs]
                        thrpt:  [529.77 Melem/s 535.03 Melem/s 540.00 Melem/s]
                 change:
                        time:   [+12.520% +13.234% +14.163%] (p = 0.00 < 0.05)
                        thrpt:  [−12.406% −11.687% −11.127%]
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Benchmarking build_and_drop/100: Warming up for 3Benchmarking build_and_drop/100: Collecting 100 sbuild_and_drop/100      time:   [19.768 µs 19.773 µs 19.778 µs]
                        thrpt:  [5.0561 Melem/s 5.0575 Melem/s 5.0587 Melem/s]
                 change:
                        time:   [+30.759% +31.091% +31.432%] (p = 0.00 < 0.05)
                        thrpt:  [−23.915% −23.717% −23.523%]
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Benchmarking build_and_drop/1000: Warming up for Benchmarking build_and_drop/1000: Collecting 100 build_and_drop/1000     time:   [216.38 µs 217.09 µs 217.83 µs]
                        thrpt:  [4.5908 Melem/s 4.6064 Melem/s 4.6215 Melem/s]
                 change:
                        time:   [+12.670% +13.317% +13.984%] (p = 0.00 < 0.05)
                        thrpt:  [−12.269% −11.752% −11.246%]
                        Performance has regressed.
Benchmarking build_and_drop/10000: Warming up forBenchmarking build_and_drop/10000: Collecting 100build_and_drop/10000    time:   [2.6091 ms 2.6168 ms 2.6247 ms]
                        thrpt:  [3.8100 Melem/s 3.8215 Melem/s 3.8328 Melem/s]
                 change:
                        time:   [+16.818% +17.254% +17.718%] (p = 0.00 < 0.05)
                        thrpt:  [−15.052% −14.715% −14.397%]
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
Benchmarking build_and_drop/100000: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.4s, or reduce sample count to 70.
Benchmarking build_and_drop/100000: Collecting 10build_and_drop/100000   time:   [61.632 ms 62.091 ms 62.568 ms]
                        thrpt:  [1.5983 Melem/s 1.6105 Melem/s 1.6225 Melem/s]
                 change:
                        time:   [+11.666% +12.668% +13.638%] (p = 0.00 < 0.05)
                        thrpt:  [−12.001% −11.244% −10.447%]
                        Performance has regressed.

Running benches/machine_runtime.rs (target/release/deps/machine_runtime-fab24ffa83af4f7d)
Gnuplot not found, using plotters backend
Benchmarking instance_construction/existing_sourcBenchmarking instance_construction/existing_sourcBenchmarking instance_construction/existing_source/100: Collecting 100 samples in estimated 5.0007Benchmarking instance_construction/existing_sourcinstance_construction/existing_source/100
                        time:   [138.49 ns 138.84 ns 139.14 ns]
                        change: [+100.94% +101.58% +102.23%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking instance_construction/existing_sourcBenchmarking instance_construction/existing_sourcBenchmarking instance_construction/existing_source/1000: Collecting 100 samples in estimated 5.001Benchmarking instance_construction/existing_sourcinstance_construction/existing_source/1000
                        time:   [1.0421 µs 1.0542 µs 1.0648 µs]
                        change: [+52.722% +53.616% +54.510%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 38 outliers among 100 measurements (38.00%)  19 (19.00%) low mild
  19 (19.00%) high severe
Benchmarking instance_construction/existing_sourcBenchmarking instance_construction/existing_sourcBenchmarking instance_construction/existing_source/10000: Collecting 100 samples in estimated 5.02Benchmarking instance_construction/existing_sourcinstance_construction/existing_source/10000
                        time:   [7.5065 µs 7.5126 µs 7.5204 µs]
                        change: [+28.611% +28.918% +29.275%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 20 outliers among 100 measurements (20.00%)  2 (2.00%) high mild
  18 (18.00%) high severe
Benchmarking instance_construction/existing_sourcBenchmarking instance_construction/existing_sourcBenchmarking instance_construction/existing_source/100000: Collecting 100 samples in estimated 5.2Benchmarking instance_construction/existing_sourcinstance_construction/existing_source/100000
                        time:   [57.146 µs 57.650 µs 58.201 µs]
                        change: [−38.078% −37.552% −36.986%] (p = 0.00 < 0.05)
                        Performance has improved.Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

Benchmarking instance_construction/terminal/100: Benchmarking instance_construction/terminal/100: Collecting 100 samples in estimated 5.0001 s (47MBenchmarking instance_construction/terminal/100: instance_construction/terminal/100
                        time:   [107.56 ns 107.73 ns 107.88 ns]
                        change: [−16.917% −16.754% −16.578%] (p = 0.00 < 0.05)
                        Performance has improved.Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Benchmarking instance_construction/terminal/1000:Benchmarking instance_construction/terminal/1000: Collecting 100 samples in estimated 5.0020 s (5.Benchmarking instance_construction/terminal/1000:instance_construction/terminal/1000
                        time:   [717.97 ns 719.76 ns 721.53 ns]
                        change: [+80.990% +83.111% +85.315%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)  9 (9.00%) low mild
  8 (8.00%) high severe
Benchmarking instance_construction/terminal/10000Benchmarking instance_construction/terminal/10000: Collecting 100 samples in estimated 5.0023 s (9Benchmarking instance_construction/terminal/10000instance_construction/terminal/10000
                        time:   [485.67 ns 486.08 ns 486.51 ns]
                        change: [−94.971% −94.962% −94.952%] (p = 0.00 < 0.05)
                        Performance has improved.Benchmarking instance_construction/terminal/10000Benchmarking instance_construction/terminal/10000Benchmarking instance_construction/terminal/100000: Collecting 100 samples in estimated 5.0980 s (Benchmarking instance_construction/terminal/10000instance_construction/terminal/100000
                        time:   [167.12 µs 167.74 µs 168.38 µs]
                        change: [+12.569% +13.915% +14.902%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

Benchmarking instance_construction/missing/100: WBenchmarking instance_construction/missing/100: Collecting 100 samples in estimated 5.0003 s (25M Benchmarking instance_construction/missing/100: Ainstance_construction/missing/100
                        time:   [153.10 ns 153.46 ns 153.82 ns]
                        change: [+16.512% +18.667% +20.901%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high severe
Benchmarking instance_construction/missing/1000: Benchmarking instance_construction/missing/1000: Collecting 100 samples in estimated 5.0014 s (4.1Benchmarking instance_construction/missing/1000: instance_construction/missing/1000
                        time:   [1.2150 µs 1.2179 µs 1.2204 µs]
                        change: [+19.489% +19.847% +20.192%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) low mild
Benchmarking instance_construction/missing/10000:Benchmarking instance_construction/missing/10000: Collecting 100 samples in estimated 5.0359 s (19Benchmarking instance_construction/missing/10000:instance_construction/missing/10000
                        time:   [13.452 µs 14.380 µs 15.585 µs]
                        change: [+79.817% +96.653% +112.16%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking instance_construction/missing/100000Benchmarking instance_construction/missing/100000: Collecting 100 samples in estimated 5.5789 s (3Benchmarking instance_construction/missing/100000instance_construction/missing/100000
                        time:   [177.75 µs 178.44 µs 179.19 µs]
                        change: [+6.8001% +7.8002% +8.8572%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking can_transition_to/allowed/100: WarmiBenchmarking can_transition_to/allowed/100: Collecting 100 samples in estimated 5.0000 s (68M iterBenchmarking can_transition_to/allowed/100: Analycan_transition_to/allowed/100
                        time:   [67.852 ns 68.431 ns 68.897 ns]
                        change: [−26.732% −26.255% −25.747%] (p = 0.00 < 0.05)
                        Performance has improved.Benchmarking can_transition_to/allowed/1000: WarmBenchmarking can_transition_to/allowed/1000: Collecting 100 samples in estimated 5.0007 s (5.2M itBenchmarking can_transition_to/allowed/1000: Analcan_transition_to/allowed/1000
                        time:   [962.47 ns 962.79 ns 963.16 ns]
                        change: [+91.831% +92.114% +92.342%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe
Benchmarking can_transition_to/allowed/10000: WarBenchmarking can_transition_to/allowed/10000: Collecting 100 samples in estimated 5.0011 s (1.6M iBenchmarking can_transition_to/allowed/10000: Anacan_transition_to/allowed/10000
                        time:   [2.4910 µs 2.4917 µs 2.4925 µs]
                        change: [−42.059% −42.026% −41.998%] (p = 0.00 < 0.05)
                        Performance has improved.Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
Benchmarking can_transition_to/allowed/100000: WaBenchmarking can_transition_to/allowed/100000: Collecting 100 samples in estimated 5.0727 s (197k Benchmarking can_transition_to/allowed/100000: Ancan_transition_to/allowed/100000
                        time:   [25.532 µs 25.578 µs 25.632 µs]
                        change: [−16.739% −16.292% −15.651%] (p = 0.00 < 0.05)
                        Performance has improved.Found 12 outliers among 100 measurements (12.00%)  5 (5.00%) high mild
  7 (7.00%) high severe

Benchmarking can_transition_to/disallowed_existinBenchmarking can_transition_to/disallowed_existinBenchmarking can_transition_to/disallowed_existing/100: Collecting 100 samples in estimated 5.0004Benchmarking can_transition_to/disallowed_existincan_transition_to/disallowed_existing/100
                        time:   [94.805 ns 94.854 ns 94.910 ns]
                        change: [−30.995% −30.957% −30.917%] (p = 0.00 < 0.05)
                        Performance has improved.Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Benchmarking can_transition_to/disallowed_existinBenchmarking can_transition_to/disallowed_existinBenchmarking can_transition_to/disallowed_existing/1000: Collecting 100 samples in estimated 5.002Benchmarking can_transition_to/disallowed_existincan_transition_to/disallowed_existing/1000
                        time:   [851.05 ns 855.37 ns 858.87 ns]
                        change: [−5.2539% −4.8111% −4.3350%] (p = 0.00 < 0.05)
                        Performance has improved.Benchmarking can_transition_to/disallowed_existinBenchmarking can_transition_to/disallowed_existinBenchmarking can_transition_to/disallowed_existing/10000: Collecting 100 samples in estimated 5.00Benchmarking can_transition_to/disallowed_existincan_transition_to/disallowed_existing/10000
                        time:   [8.6174 µs 8.6835 µs 8.7356 µs]
                        change: [+17.334% +18.189% +19.212%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking can_transition_to/disallowed_existinBenchmarking can_transition_to/disallowed_existinBenchmarking can_transition_to/disallowed_existing/100000: Collecting 100 samples in estimated 5.1Benchmarking can_transition_to/disallowed_existincan_transition_to/disallowed_existing/100000
                        time:   [236.32 µs 238.03 µs 239.80 µs]
                        change: [+62.441% +64.482% +66.491%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

Benchmarking can_transition_to/missing/100: WarmiBenchmarking can_transition_to/missing/100: Collecting 100 samples in estimated 5.0005 s (39M iterBenchmarking can_transition_to/missing/100: Analycan_transition_to/missing/100
                        time:   [93.133 ns 93.176 ns 93.224 ns]
                        change: [+15.067% +15.217% +15.357%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
Benchmarking can_transition_to/missing/1000: WarmBenchmarking can_transition_to/missing/1000: Collecting 100 samples in estimated 5.0027 s (6.0M itBenchmarking can_transition_to/missing/1000: Analcan_transition_to/missing/1000
                        time:   [822.50 ns 822.81 ns 823.13 ns]
                        change: [+9.6397% +10.381% +11.150%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
Benchmarking can_transition_to/missing/10000: WarBenchmarking can_transition_to/missing/10000: Collecting 100 samples in estimated 5.0260 s (273k iBenchmarking can_transition_to/missing/10000: Anacan_transition_to/missing/10000
                        time:   [18.512 µs 18.568 µs 18.636 µs]
                        change: [+126.18% +127.45% +129.33%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
Benchmarking can_transition_to/missing/100000: WaBenchmarking can_transition_to/missing/100000: Collecting 100 samples in estimated 6.1119 s (25k iBenchmarking can_transition_to/missing/100000: can_transition_to/missing/100000
                        time:   [167.06 µs 167.52 µs 168.07 µs]
                        change: [+1.6340% +5.4522% +9.7548%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 15 outliers among 100 measurements (15.00%)  1 (1.00%) low mild
  2 (2.00%) high mild
  12 (12.00%) high severe

Benchmarking transition_to/allowed/100: Warming uBenchmarking transition_to/allowed/100: Collecting 100 samples in estimated 5.0015 s (15M iteratiotransition_to/allowed/100                                                time:   [132.06 ns 139.50 ns 146.00 ns]
                        change: [−22.169% −19.630% −16.833%] (p = 0.00 < 0.05)
                        Performance has improved.Found 17 outliers among 100 measurements (17.00%)  17 (17.00%) high severe
Benchmarking transition_to/allowed/1000: Warming Benchmarking transition_to/allowed/1000: Collecting 100 samples in estimated 5.0074 s (2.3M iteratBenchmarking transition_to/allowed/1000: Analyzintransition_to/allowed/1000
                        time:   [973.20 ns 975.15 ns 976.97 ns]
                        change: [+13.489% +14.018% +14.558%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking transition_to/allowed/10000: WarmingBenchmarking transition_to/allowed/10000: Collecting 100 samples in estimated 5.1305 s (111k iteraBenchmarking transition_to/allowed/10000: Analyzitransition_to/allowed/10000
                        time:   [12.515 µs 13.715 µs 15.106 µs]
                        change: [+763.17% +822.65% +880.17%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 32 outliers among 100 measurements (32.00%)  22 (22.00%) low severe
  5 (5.00%) high mild
  5 (5.00%) high severe
Benchmarking transition_to/allowed/100000: WarminBenchmarking transition_to/allowed/100000: Collecting 100 samples in estimated 5.9465 s (30k iteraBenchmarking transition_to/allowed/100000: Analyztransition_to/allowed/100000
                        time:   [100.34 µs 101.38 µs 102.26 µs]
                        change: [+385.47% +390.51% +395.26%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking transition_to/disallowed_existing/10Benchmarking transition_to/disallowed_existing/10Benchmarking transition_to/disallowed_existing/100: Collecting 100 samples in estimated 5.0018 s (Benchmarking transition_to/disallowed_existing/10transition_to/disallowed_existing/100
                        time:   [216.92 ns 218.04 ns 219.04 ns]
                        change: [−19.861% −18.998% −18.136%] (p = 0.00 < 0.05)
                        Performance has improved.Benchmarking transition_to/disallowed_existing/10Benchmarking transition_to/disallowed_existing/10Benchmarking transition_to/disallowed_existing/1000: Collecting 100 samples in estimated 5.0015 s Benchmarking transition_to/disallowed_existing/10transition_to/disallowed_existing/1000
                        time:   [981.18 ns 982.57 ns 984.03 ns]
                        change: [−4.6088% −4.1394% −3.7091%] (p = 0.00 < 0.05)
                        Performance has improved.Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
Benchmarking transition_to/disallowed_existing/10Benchmarking transition_to/disallowed_existing/10Benchmarking transition_to/disallowed_existing/10000: Collecting 100 samples in estimated 5.0127 sBenchmarking transition_to/disallowed_existing/10transition_to/disallowed_existing/10000
                        time:   [8.6395 µs 8.6544 µs 8.6691 µs]
                        change: [−8.0315% −7.7735% −7.5064%] (p = 0.00 < 0.05)
                        Performance has improved.Found 15 outliers among 100 measurements (15.00%)  10 (10.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe
Benchmarking transition_to/disallowed_existing/10Benchmarking transition_to/disallowed_existing/10Benchmarking transition_to/disallowed_existing/100000: Collecting 100 samples in estimated 5.5155 Benchmarking transition_to/disallowed_existing/10transition_to/disallowed_existing/100000
                        time:   [175.07 µs 175.49 µs 175.93 µs]
                        change: [−6.6740% −5.7376% −4.9177%] (p = 0.00 < 0.05)
                        Performance has improved.Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

Benchmarking transition_to/missing/10Benchmarking transition_to/missing/10Benchmarking transition_to/missing/100: Collecting 100 samples in estimated 5.0029 Benchmarking transition_to/missing/10transition_to/disallowed_existing/100
                        time:   [256.14 ns 256.88 ns 257.58 ns]
                        change: [+27.004% +28.184% +29.327%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 16 outliers among 100 measurements (16.00%)  10 (10.00%) low severe
  5 (5.00%) low mild
  1 (1.00%) high mild
Benchmarking transition_to/missing/10Benchmarking transition_to/missing/10Benchmarking transition_to/missing/1000: Collecting 100 samples in estimated 5.0135Benchmarking transition_to/missing/10transition_to/disallowed_existing/1000
                        time:   [981.04 ns 986.77 ns 993.50 ns]
                        change: [+1.1394% +1.7140% +2.3436%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe
Benchmarking transition_to/missing/10Benchmarking transition_to/missing/10Benchmarking transition_to/missing/10000: Collecting 100 samples in estimated 5.014Benchmarking transition_to/missing/10transition_to/disallowed_existing/10000
                        time:   [9.1783 µs 9.2653 µs 9.3358 µs]
                        change: [+2.4562% +3.2796% +4.0955%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking transition_to/missing/10Benchmarking transition_to/missing/10Benchmarking transition_to/missing/100000: Collecting 100 samples in estimated 6.34Benchmarking transition_to/missing/10transition_to/missing/100000
                        time:   [238.30 µs 240.20 µs 242.16 µs]
                        change: [+33.314% +34.490% +35.658%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking is_terminal/source/100: Warming up fBenchmarking is_terminal/source/100: Collecting 1is_terminal/source/100  time:   [86.404 ns 86.788 ns 87.076 ns]
                        change: [−5.2609% −4.5851% −3.9149%] (p = 0.00 < 0.05)
                        Performance has improved.Benchmarking is_terminal/source/1000: Warming up Benchmarking is_terminal/source/1000: Collecting 100 samples in estimated 5.0029 s (4.6M iterationis_terminal/source/1000 time:   [1.1609 µs 1.1661 µs 1.1714 µs]
                        change: [+271.98% +273.58% +275.22%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking is_terminal/source/10000: Warming upBenchmarking is_terminal/source/10000: Collecting 100 samples in estimated 5.0462 s (298k iteratiois_terminal/source/10000
                        time:   [7.4226 µs 7.7081 µs 8.0950 µs]
                        change: [+74.961% +91.413% +109.07%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking is_terminal/source/100000: Warming uBenchmarking is_terminal/source/100000: Collecting 100 samples in estimated 5.3676 s (56k iteratiois_terminal/source/100000                                                time:   [102.75 µs 103.24 µs 103.65 µs]
                        change: [+102.70% +105.11% +107.19%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking is_terminal/terminal/100: Warming upBenchmarking is_terminal/terminal/100: Collecting 100 samples in estimated 5.0005 s (41M iterationis_terminal/terminal/100
                        time:   [119.74 ns 119.78 ns 119.82 ns]
                        change: [+49.514% +49.788% +50.067%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)  1 (1.00%) low mild
  4 (4.00%) high mild
  8 (8.00%) high severe
Benchmarking is_terminal/terminal/1000: Warming uBenchmarking is_terminal/terminal/1000: Collecting 100 samples in estimated 5.0053 s (4.0M iteratiis_terminal/terminal/1000                                                time:   [1.2886 µs 1.2894 µs 1.2900 µs]
                        change: [+3.2373% +3.6156% +3.9424%] (p = 0.00 < 0.05)
                        Performance has regressed.
Benchmarking is_terminal/terminal/10000: Warming Benchmarking is_terminal/terminal/10000: Collecting 100 samples in estimated 5.0337 s (525k iteratBenchmarking is_terminal/terminal/10000: Analyzinis_terminal/terminal/10000
                        time:   [8.1645 µs 8.2518 µs 8.3336 µs]
                        change: [−0.9152% −0.4579% +0.0846%] (p = 0.09 > 0.05)
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)  3 (3.00%) high mild
  14 (14.00%) high severe
Benchmarking is_terminal/terminal/100000: WarmingBenchmarking is_terminal/terminal/100000: Collecting 100 samples in estimated 6.0184 s (25k iteratBenchmarking is_terminal/terminal/100000: Analyziis_terminal/terminal/100000
                        time:   [236.08 µs 237.98 µs 239.99 µs]
                        change: [+46.018% +48.152% +50.181%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe