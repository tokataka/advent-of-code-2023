macro_rules! export_solutions {
    ($($module:ident),* $(,)?) => (
        $(mod $module;)*

        pub fn solutions() -> &'static [(&'static str, fn(Vec<&str>) -> String)] {
            &[
                $((stringify!($module), $module::solution),)*
            ]
        }
    );
}

export_solutions!(
    day1_p1, day1_p2, day2_p1, day2_p2, day3_p1, day3_p2, day4_p1, day4_p2, day5_p1, day5_p2,
    day6_p1, day6_p2, day7_p1, day7_p2, day8_p1, day8_p2, day9_p1, day9_p2, day10_p1, day10_p2,
);
