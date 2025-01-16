#!/bin/bash

day=$1

if [ -z $day ]
then
  day=$(ls | egrep -o "[0-9]+" | sort -nr | head -1)
  day=${day##0}  # Remove leading zero to avoid octal error
  ((day++))
fi

day=$(printf "%02d" $day)

year=$(basename $(pwd))
mkdir ../../input/$year/day$day

sed -i "/load_year\!/,/^$/ s/\(day[[:digit:]]\{2\}\))/\1, day$day)/" ../lib.rs

cat << EOF > day$day.rs
use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> i64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    match args.part {
        1 => run_part1(data),
        2 => run_part2(data),
        3 => run_part3(data),
        _ => unreachable!(),
  }
}

fn run_part1(input: String) -> i64 {
    0
}

fn run_part2(input: String) -> i64 {
    0
}

fn run_part3(input: String) -> i64 {
    0
}
EOF
