# From Ruby to Rust

Those are the benchmarks linked to my talk, to run the benchmarks simply checkout this repo and launch `run.sh`, the only requirement is docker.<br/>
It will take some time, and prints only benchmark output data, like this:
```
Calculating -------------------------------------
                ruby    940.605  (± 4.1%) i/s -      4.700k in   5.005957s
cached_method           time:   [128.52 µs 135.20 µs 142.91 µs]
                        change: [-7.6763% -2.6618% +2.6886%] (p = 0.33 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

cached_method_trait     time:   [133.68 µs 144.82 µs 155.86 µs]
                        change: [-5.7364% -0.5558% +4.7331%] (p = 0.84 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe

```

Output data has two different formats, so we'll need to do a little conversion, the easiest is to convert Ruby format into Rust format:<br/>
Convert iterations/second into microseconds/iteration
```
1000000/940.605 = 1063.1455 ± 4.1% => [1019,5566 µs 1063.1455 µs 1106.7345 µs]
```

# Conclusions

Our Rust implementation is at least 8 times faster than Ruby on Rails, while the trait version is 10% slower due to allocations.
