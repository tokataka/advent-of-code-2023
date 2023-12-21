# Advent of Code 2023 Solutions in Rust

My solutions for [AoC 2023](https://adventofcode.com/2023) in Rust.

I created this project so that I could focus on solving the problem with less effort to other than problem solving.

## Usage

```sh
# run
cargo run [--release] -- day1_p1

# benchmark
cargo run [--release] --bin bench [-- day1_p1 [day1_p2 ...]]
```

## How to add solutions

All solutions are named in {name}\_{part} format.

Solutions with same name share one input file.

You might want to add

- `src/solutions/{name}_{part}.rs` which includes following function
  ```rust
  pub fn solution(lines: Vec<&str>) -> String {
      ...
  }
  ```
- `resource/input/{name}.txt`
- solution name in `src/solutions/mod.rs` with comma seperated
  ```rust
  export_solutions!(
      day1_p1, day1_p2, ...
  );
  ```

## Benchmark Results (Ryzen 5 3600)

```
[day1_p1]
Result of 100 iterations
Average: 29.675us, Min: 26.585us, Max: 68.469us

[day1_p2]
Result of 100 iterations
Average: 130.545us, Min: 127.284us, Max: 225.765us

[day2_p1]
Result of 100 iterations
Average: 51.154us, Min: 49.309us, Max: 101.023us

[day2_p2]
Result of 100 iterations
Average: 80.709us, Min: 77.909us, Max: 127.884us

[day3_p1]
Result of 100 iterations
Average: 435.365us, Min: 428.979us, Max: 526.432us

[day3_p2]
Result of 100 iterations
Average: 463.248us, Min: 456.991us, Max: 540.264us

[day4_p1]
Result of 100 iterations
Average: 462.101us, Min: 451.692us, Max: 724.180us

[day4_p2]
Result of 100 iterations
Average: 466.551us, Min: 454.609us, Max: 570.133us

[day5_p1]
Result of 100 iterations
Average: 28.007us, Min: 26.501us, Max: 53.625us

[day5_p2]
Result of 100 iterations
Average: 76.018us, Min: 72.843us, Max: 113.466us

[day6_p1]
Result of 100 iterations
Average: 0.552us, Min: 0.514us, Max: 2.600us

[day6_p2]
Result of 100 iterations
Average: 4169.097us, Min: 4117.377us, Max: 4484.721us

[day7_p1]
Result of 100 iterations
Average: 381.459us, Min: 364.146us, Max: 560.471us

[day7_p2]
Result of 100 iterations
Average: 349.677us, Min: 343.550us, Max: 431.883us

[day8_p1]
Result of 100 iterations
Average: 492.427us, Min: 478.803us, Max: 575.588us

[day8_p2]
Result of 100 iterations
Average: 5326.148us, Min: 5215.284us, Max: 5984.540us

[day9_p1]
Result of 100 iterations
Average: 447.275us, Min: 442.515us, Max: 516.099us

[day9_p2]
Result of 100 iterations
Average: 454.949us, Min: 448.119us, Max: 514.768us

[day10_p1]
Result of 100 iterations
Average: 1863.787us, Min: 1785.317us, Max: 2167.842us

[day10_p2]
Result of 100 iterations
Average: 2946.552us, Min: 2859.744us, Max: 3332.303us

[day11_p1]
Result of 100 iterations
Average: 230.340us, Min: 229.235us, Max: 259.152us

[day11_p2]
Result of 100 iterations
Average: 228.370us, Min: 225.552us, Max: 308.752us

[day12_p1]
Result of 100 iterations
Average: 1880.147us, Min: 1841.468us, Max: 2205.182us

[day12_p2]
Result of 100 iterations
Average: 36482.625us, Min: 35917.525us, Max: 39152.684us

[day13_p1]
Result of 100 iterations
Average: 97.072us, Min: 94.924us, Max: 127.779us

[day13_p2]
Result of 100 iterations
Average: 858.914us, Min: 815.362us, Max: 1147.381us

[day14_p1]
Result of 100 iterations
Average: 39.297us, Min: 37.454us, Max: 83.693us

[day14_p2]
Result of 100 iterations
Average: 36532.507us, Min: 35825.900us, Max: 39613.093us

[day15_p1]
Result of 100 iterations
Average: 101.776us, Min: 99.937us, Max: 135.692us

[day15_p2]
Result of 100 iterations
Average: 264.901us, Min: 258.842us, Max: 389.920us

[day16_p1]
Result of 100 iterations
Average: 1319.239us, Min: 1244.584us, Max: 1548.447us

[day16_p2]
Result of 100 iterations
Average: 334288.463us, Min: 330913.929us, Max: 344586.531us

[day17_p1]
Result of 100 iterations
Average: 859051.627us, Min: 848995.725us, Max: 886070.817us

[day17_p2]
Result of 100 iterations
Average: 1466612.289us, Min: 1439661.842us, Max: 1604590.138us

[day18_p1]
Result of 100 iterations
Average: 15.798us, Min: 14.994us, Max: 31.036us

[day18_p2]
Result of 100 iterations
Average: 19.791us, Min: 17.629us, Max: 45.007us

[day19_p1]
Result of 100 iterations
Average: 285.704us, Min: 273.532us, Max: 421.198us

[day19_p2]
Result of 100 iterations
Average: 217.317us, Min: 210.731us, Max: 319.667us

[day20_p1]
Result of 100 iterations
Average: 2778.547us, Min: 2639.123us, Max: 3202.542us

[day20_p2]
Result of 100 iterations
Average: 34054.593us, Min: 33416.398us, Max: 36313.048us

[day21_p1]
Result of 100 iterations
Average: 1567.835us, Min: 1512.870us, Max: 1864.302us

[day21_p2]
Result of 100 iterations
Average: 235033.820us, Min: 231952.201us, Max: 247034.354us
```

## License

This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
