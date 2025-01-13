mod args;

use std::iter;

pub use args::RunArgs;

struct Solution {
    year: u32,
    day: u8,
    run: fn(&RunArgs) -> i64,
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
                let run_fn = |args: &RunArgs| {
                    use $year_fn::$day::run;
                    run(args)
                };

                let year = stringify!($year_fn).strip_prefix("year_").unwrap().parse().unwrap();
                let day = stringify!($day).strip_prefix("day").unwrap().parse().unwrap();

                Solution { year, day, run: run_fn }
            },)*]
        }
    }
}

load_year!(year_2024: day01);
define_fun!(year_2024: day01);

pub fn run_solution(args: &RunArgs) -> i64 {
    let run_fn: Vec<Solution> = iter::empty()
        .chain(year_2024())
        .filter(|solution| args.year == solution.year && args.day == solution.day)
        .collect();

    (run_fn[0].run)(args)
}
