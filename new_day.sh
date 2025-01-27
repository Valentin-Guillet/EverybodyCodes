#!/bin/bash

year=$(ls src/ | egrep -o "[0-9]+" | sort -nr | head -1)

day=$(ls src/year_$year/ | egrep -o "[0-9]+" | sort -nr | head -1)
day=${day##0}  # Remove leading zero to avoid octal error
((day++))

day=$(printf "%02d" $day)

mkdir -p ./input/year_$year/day$day

sed -i "/load_year\!/,/^$/ s/\(day[[:digit:]]\{2\}\))/\1, day$day)/" src/lib.rs

cat << EOF > src/year_$year/day$day.rs
use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    match args.part {
        1 => unimplemented!(),
        2 => unimplemented!(),
        3 => unimplemented!(),
        _ => unreachable!(),
    }
}
EOF
