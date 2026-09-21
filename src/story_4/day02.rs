use crate::args::RunArgs;

use std::collections::HashSet;
use std::fs::read_to_string;

struct Config {
    start: (i32, i32),
    positions: [(i32, i32); 3],
    moves: Option<Vec<usize>>,
}

fn parse_position(input: &str) -> (i32, i32) {
    let open_ind = input.find('[').unwrap() + 1;
    let comma_ind = input.find(',').unwrap();
    let x = input[open_ind..comma_ind].parse().unwrap();
    let y = input[comma_ind + 1..input.len() - 1].parse().unwrap();
    (x, y)
}

impl Config {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let start = parse_position(lines.next().unwrap());
        let positions = std::array::from_fn(|_| parse_position(lines.next().unwrap()));
        let moves = lines
            .next()
            .and_then(|s| Some(s[6..].chars().map(|c| c as usize - 'A' as usize).collect()));
        Self {
            start,
            positions,
            moves,
        }
    }
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let config = Config::new(data.trim_end());

    match args.part {
        1 => get_illuminated_squares(&config).len() as u32,
        2 => count_fireflies(&get_illuminated_squares(&config)),
        3 => count_fireflies(&get_theoretical_illuminated_squares(&config)),
        _ => unreachable!(),
    }
}

fn get_illuminated_squares(config: &Config) -> HashSet<(i32, i32)> {
    let mut pos = config.start;
    let mut seen: HashSet<(i32, i32)> = HashSet::from([pos]);
    for target in config.moves.as_ref().unwrap() {
        let target_pos = config.positions[*target];
        pos.0 = (pos.0 + target_pos.0) / 2;
        pos.1 = (pos.1 + target_pos.1) / 2;
        seen.insert(pos);
    }
    seen
}

fn count_fireflies(illuminated_squares: &HashSet<(i32, i32)>) -> u32 {
    // let illuminated_squares = get_illuminated_squares(config);
    let prev_len = illuminated_squares.len();
    let mut fireflies = illuminated_squares.iter().copied().collect::<HashSet<_>>();
    for &(x, y) in illuminated_squares {
        fireflies.insert((x - 1, y));
        fireflies.insert((x, y - 1));
        fireflies.insert((x + 1, y));
        fireflies.insert((x, y + 1));
    }
    fireflies.len() as u32 - prev_len as u32
}

fn get_theoretical_illuminated_squares_rec(seen: &mut HashSet<(i32, i32)>, pos: (i32, i32), config: &Config) {
    for (x, y) in config.positions {
        let next_pos = ((pos.0 + x) / 2, (pos.1 + y) / 2);
        if seen.contains(&next_pos) {
            continue;
        }

        seen.insert(next_pos);
        get_theoretical_illuminated_squares_rec(seen, next_pos, config);
    }
}

fn get_theoretical_illuminated_squares(config: &Config) -> HashSet<(i32, i32)> {
    let mut seen = HashSet::new();
    get_theoretical_illuminated_squares_rec(&mut seen, config.positions[0], config);
    seen
}
