mod args;

use std::{fmt::Display, iter};

pub use args::RunArgs;

struct Solution {
    year_or_story: u32,
    day: u8,
    run: fn(&RunArgs) -> Box<dyn Display>,
}

macro_rules! load_src {
    ($year:ident: $($day:ident),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
        fn $year() -> Vec<Solution> {
            vec![$({
                let year_or_story = match stringify!($year) {
                    year if year.starts_with("year_") => year.strip_prefix("year_").unwrap().parse().unwrap(),
                    story => story.strip_prefix("story_").unwrap().parse().unwrap(),
                };
                let day = stringify!($day).strip_prefix("day").unwrap().parse().unwrap();
                let run = |args: &RunArgs| Box::new($year::$day::run(args)) as Box<dyn Display>;

                Solution { year_or_story, day, run }
            },)*]
        }
    }
}

load_src!(year_2024: day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20);
load_src!(year_2025: day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20);
load_src!(story_1: day01, day02, day03);
load_src!(story_2: day01, day02, day03);
load_src!(story_3: day01);

pub fn run_solution(args: &RunArgs) -> Box<dyn Display> {
    let run_fn: Vec<Solution> = iter::empty()
        .chain(year_2024())
        .chain(year_2025())
        .chain(story_1())
        .chain(story_2())
        .chain(story_3())
        .filter(|solution| {
            args.year.or(args.story).unwrap() == solution.year_or_story && args.day == solution.day
        })
        .collect();

    (run_fn[0].run)(args)
}
