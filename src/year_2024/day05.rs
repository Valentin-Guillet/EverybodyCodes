use crate::args::RunArgs;

use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> i64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut columns: [VecDeque<i64>; 4] = Default::default();
    for line in data.lines() {
        for (i, n) in line.split(' ').map(|c| c.parse().unwrap()).enumerate() {
            columns[i].push_back(n);
        }
    }

    match args.part {
        1 => dance_for(&mut columns, 10),
        2 => dance_until_nth_repeat(&mut columns, 2024),
        3 => get_largest_number(&mut columns),
        _ => unreachable!(),
    }
}

fn shout_value(columns: &[VecDeque<i64>; 4]) -> i64 {
    if columns[0][0] < 10 {
        1000 * columns[0][0] + 100 * columns[1][0] + 10 * columns[2][0] + columns[3][0]
    } else if columns[0][0] < 100 {
        1000000 * columns[0][0] + 10000 * columns[1][0] + 100 * columns[2][0] + columns[3][0]
    } else {
        1000000000000 * columns[0][0]
            + 100000000 * columns[1][0]
            + 10000 * columns[2][0]
            + columns[3][0]
    }
}

fn dance_a_round(columns: &mut [VecDeque<i64>; 4], col: usize) {
    let next_col = (col + 1) % 4;
    let n = columns[next_col].len();
    let clapper = columns[col].pop_front().unwrap();
    let target_pos = ((clapper as usize - 1) % (2 * n)) + 1;
    if target_pos <= n + 1 {
        columns[next_col].insert(target_pos - 1, clapper);
    } else {
        columns[next_col].insert(2 * n - target_pos + 1, clapper);
    }
}

fn dance_for(columns: &mut [VecDeque<i64>; 4], nb_turns: u32) -> i64 {
    for turn in 0..nb_turns {
        dance_a_round(columns, (turn % 4) as usize);
    }
    shout_value(columns)
}

fn dance_until_nth_repeat(columns: &mut [VecDeque<i64>; 4], nb_repeat: u32) -> i64 {
    let mut turn: i64 = 0;
    let mut count_map: HashMap<i64, u32> = HashMap::new();
    loop {
        dance_a_round(columns, (turn % 4) as usize);

        let result = shout_value(columns);
        count_map.entry(result).and_modify(|e| *e += 1).or_insert(1);

        if *count_map.get(&result).unwrap() == nb_repeat {
            return result * (turn + 1);
        }
        turn += 1;
    }
}

fn get_largest_number(columns: &mut [VecDeque<i64>; 4]) -> i64 {
    let mut turn = 0;
    let mut max_value = 0;
    loop {
        dance_a_round(columns, (turn % 4) as usize);

        let result = shout_value(columns);
        if result > max_value {
            max_value = result;
        }

        // Columns are stable: only the first 4 elts can be moved to other column
        if columns
            .iter()
            .all(|col| col.iter().take(4).all(|v| *v % 50 <= 5))
        {
            return max_value;
        }
        turn += 1;
    }
}
