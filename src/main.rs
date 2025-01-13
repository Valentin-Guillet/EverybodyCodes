use everybody_codes::{run_solution, RunArgs};
use std::process;

fn main() {
    let args = RunArgs::parse().unwrap_or_else(|err| {
        println!("Error in arguments: {}", err);
        process::exit(1);
    });

    let solution = run_solution(&args);
    println!("Answer: {solution}");
}
