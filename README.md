# From Ruby to Rust

Those are the benchmarks linked to my talk, to run the benchmarks simply checkout this repo and launch `run.sh`, the only requirement is docker.<br/>
It will take some time, and prints only benchmark output data, like this:
```
Calculating -------------------------------------
                ruby    940.605  (± 4.1%) i/s -      4.700k in   5.005957s
rust                    time:   [121.39 µs 124.28 µs 127.39 µs]                      
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

```

Output data has two different formats, so we'll need to do a little conversion, the easiest is to convert Ruby format into Rust format:<br/>
Convert iterations/second into microseconds/iteration
```
1000000/940.605 = 1063.1455 ± 4.1% => [1019,5566 µs 1063.1455 µs 1106.7345 µs]
```

# Conclusions

Rust is at least 8 times faster than Ruby on Rails
