mod args;

use std::{fmt::Display, iter};

pub use args::RunArgs;

struct Solution {
    year: u32,
    day: u8,
    run: fn(&RunArgs) -> Box<dyn Display>,
}

macro_rules! load_year {
    ($year:ident: $($day:ident),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

macro_rules! define_fun {
    ($year_fn:ident: $($day:ident),*) => {
        fn $year_fn() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year_fn).strip_prefix("year_").unwrap().parse().unwrap();
                let day = stringify!($day).strip_prefix("day").unwrap().parse().unwrap();
                let run = |args: &RunArgs| Box::new($year_fn::$day::run(args)) as Box<dyn Display>;

                Solution { year, day, run }
            },)*]
        }
    }
}

load_year!(year_2024: day01, day02, day03, day04, day05, day06, day07);
define_fun!(year_2024: day01, day02, day03, day04, day05, day06, day07);

pub fn run_solution(args: &RunArgs) -> Box<dyn Display> {
    let run_fn: Vec<Solution> = iter::empty()
        .chain(year_2024())
        .filter(|solution| args.year == solution.year && args.day == solution.day)
        .collect();

    (run_fn[0].run)(args)
}
