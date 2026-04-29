use everybody_codes::{RunArgs, run_solution};
use std::process;

fn main() {
    let args = RunArgs::parse().unwrap_or_else(|err| {
        println!("Error in arguments: {}", err);
        process::exit(1);
    });

    let solution = run_solution(&args);
    let msg = if args.year.is_some() {
        format!("Year {}", args.year.unwrap())
    } else {
        format!("Story {}", args.story.unwrap())
    };
    println!("{msg}/{:02}/{} Answer: {solution}", args.day, args.part);
}
