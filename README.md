# Advent of Code 2023 Solutions in Rust

My solutions for [AoC 2023](https://adventofcode.com/2023) in Rust.

I created this project so that I could focus on solving the problem with less effort to other than problem solving.

## usage

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

Result of 100 iterations

| Solution Name |       Average |           Min |           Max |
| ------------- | ------------: | ------------: | ------------: |
| day1_p1       |      29.675µs |      26.585µs |      68.469µs |
| day1_p2       |     130.545µs |     127.284µs |     225.765µs |
| day2_p1       |      51.154µs |      49.309µs |     101.023µs |
| day2_p2       |      80.709µs |      77.909µs |     127.884µs |
| day3_p1       |     435.365µs |     428.979µs |     526.432µs |
| day3_p2       |     463.248µs |     456.991µs |     540.264µs |
| day4_p1       |     462.101µs |     451.692µs |     724.180µs |
| day4_p2       |     466.551µs |     454.609µs |     570.133µs |
| day5_p1       |      28.007µs |      26.501µs |      53.625µs |
| day5_p2       |      76.018µs |      72.843µs |     113.466µs |
| day6_p1       |       0.552µs |       0.514µs |       2.600µs |
| day6_p2       |    4169.097µs |    4117.377µs |    4484.721µs |
| day7_p1       |     381.459µs |     364.146µs |     560.471µs |
| day7_p2       |     349.677µs |     343.550µs |     431.883µs |
| day8_p1       |     492.427µs |     478.803µs |     575.588µs |
| day8_p2       |    5326.148µs |    5215.284µs |    5984.540µs |
| day9_p1       |     447.275µs |     442.515µs |     516.099µs |
| day9_p2       |     454.949µs |     448.119µs |     514.768µs |
| day10_p1      |    1863.787µs |    1785.317µs |    2167.842µs |
| day10_p2      |    2946.552µs |    2859.744µs |    3332.303µs |
| day11_p1      |     230.340µs |     229.235µs |     259.152µs |
| day11_p2      |     228.370µs |     225.552µs |     308.752µs |
| day12_p1      |    1880.147µs |    1841.468µs |    2205.182µs |
| day12_p2      |   36482.625µs |   35917.525µs |   39152.684µs |
| day13_p1      |      97.072µs |      94.924µs |     127.779µs |
| day13_p2      |     858.914µs |     815.362µs |    1147.381µs |
| day14_p1      |      39.297µs |      37.454µs |      83.693µs |
| day14_p2      |   36532.507µs |   35825.900µs |   39613.093µs |
| day15_p1      |     101.776µs |      99.937µs |     135.692µs |
| day15_p2      |     264.901µs |     258.842µs |     389.920µs |
| day16_p1      |    1319.239µs |    1244.584µs |    1548.447µs |
| day16_p2      |  334288.463µs |  330913.929µs |  344586.531µs |
| day17_p1      |  859051.627µs |  848995.725µs |  886070.817µs |
| day17_p2      | 1466612.289µs | 1439661.842µs | 1604590.138µs |
| day18_p1      |      15.798µs |      14.994µs |      31.036µs |
| day18_p2      |      19.791µs |      17.629µs |      45.007µs |
| day19_p1      |     285.704µs |     273.532µs |     421.198µs |
| day19_p2      |     217.317µs |     210.731µs |     319.667µs |
| day20_p1      |    2778.547µs |    2639.123µs |    3202.542µs |
| day20_p2      |   34054.593µs |   33416.398µs |   36313.048µs |
| day21_p1      |    1567.835µs |    1512.870µs |    1864.302µs |
| day21_p2      |  235033.820µs |  231952.201µs |  247034.354µs |
| day22_p1      |    1236.092µs |    1203.707µs |    2362.450µs |
| day22_p2      |   19755.820µs |   18383.423µs |   22479.010µs |

## License

This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
