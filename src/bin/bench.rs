use advent_of_code_2023::solutions;

use std::env;
use std::fs;
use std::time::Instant;

const ITERATION_COUNT: usize = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    let solution_names = args.get(1..);

    println!("Result of 100 iterations");
    println!();
    println!("| Solution Name |        Average |            Min |            Max |");
    println!("| ------------- | -------------: | -------------: | -------------: |");

    for (name, solution) in solutions() {
        if let Some(solution_names) = solution_names {
            if solution_names.iter().all(|x| x != name) {
                continue;
            }
        }

        let input_file = format!("resource/input/{}.txt", name.split('_').next().unwrap());
        let input = fs::read_to_string(input_file).unwrap();

        let mut elapsed_time_sum = 0.0;
        let mut elapsed_time_min = f64::MAX;
        let mut elapsed_time_max = f64::MIN;

        for _ in 0..ITERATION_COUNT {
            let lines = input.lines().collect();

            let start_time = Instant::now();

            solution(lines);

            let elapsed_time = start_time.elapsed().as_secs_f64();

            elapsed_time_sum += elapsed_time;

            if elapsed_time < elapsed_time_min {
                elapsed_time_min = elapsed_time;
            }

            if elapsed_time > elapsed_time_max {
                elapsed_time_max = elapsed_time;
            }
        }

        let mut elapsed_time_avg = elapsed_time_sum / ITERATION_COUNT as f64;

        elapsed_time_avg *= 1_000_000.0;
        elapsed_time_min *= 1_000_000.0;
        elapsed_time_max *= 1_000_000.0;

        println!("| {name:<13} | {elapsed_time_avg:>12.3}µs | {elapsed_time_min:>12.3}µs | {elapsed_time_max:>12.3}µs |");
    }
}
