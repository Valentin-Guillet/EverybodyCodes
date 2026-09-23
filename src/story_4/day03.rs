use crate::args::RunArgs;

use std::fs::read_to_string;

struct Config {
    width: usize,
    height: usize,
    horizontal_pattern: Vec<u64>,
    vertical_pattern: Vec<u64>,
}

impl Config {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        Self {
            width: lines.next().unwrap()[6..].parse().unwrap(),
            height: lines.next().unwrap()[7..].parse().unwrap(),
            horizontal_pattern: lines.next().unwrap()[19..]
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u64)
                .collect(),
            vertical_pattern: lines.next().unwrap()[17..]
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u64)
                .collect(),
        }
    }
}

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let config = Config::new(data.trim_end());

    let (tile_group1, tile_group2) = get_isolated_tile_groups(&config);
    match args.part {
        1 => tile_group1 + tile_group2,
        2 | 3 => tile_group1.max(tile_group2),
        _ => unreachable!(),
    }
}

fn get_isolated_tile_groups(config: &Config) -> (u64, u64) {
    let nb_rows = {
        let m = config.horizontal_pattern.len();
        if m % 2 == 0 { m } else { 2 * m }
    };
    let nb_cols = {
        let n = config.vertical_pattern.len();
        if n % 2 == 0 { n } else { 2 * n }
    };

    let horizontal_pattern: Vec<u64> = config
        .horizontal_pattern
        .iter()
        .cloned()
        .cycle()
        .take(nb_rows + 1)
        .collect();
    let vertical_pattern: Vec<u64> = config
        .vertical_pattern
        .iter()
        .cloned()
        .cycle()
        .take(nb_cols + 1)
        .collect();

    let horizontal_alternate_color = (0..nb_rows)
        .filter(|&row| horizontal_pattern[row] == 0)
        .count()
        % 2
        == 1;
    let vertical_alternate_color = (0..nb_cols)
        .filter(|&col| vertical_pattern[col] == 0)
        .count()
        % 2
        == 1;

    let mut tile_counts = vec![0, 0];
    let mut first_col_tile_color = 0;
    for row in 0..nb_rows {
        if horizontal_pattern[row] == 0 {
            first_col_tile_color = 1 - first_col_tile_color;
        }

        if horizontal_pattern[row] != horizontal_pattern[row + 1] {
            continue;
        }

        let mut tile_color = first_col_tile_color;
        for col in 0..nb_cols {
            if col > 0 && (row % 2) as u64 == vertical_pattern[col] {
                tile_color = 1 - tile_color;
            }

            if vertical_pattern[col] != vertical_pattern[col + 1] {
                continue;
            }

            if col as u64 % 2 != horizontal_pattern[row] || row as u64 % 2 != vertical_pattern[col]
            {
                continue;
            }

            let nb_horizontal_patterns =
                (config.width / nb_cols) + if col < config.width % nb_cols { 1 } else { 0 };
            let nb_vertical_patterns =
                (config.height / nb_rows) + if row < config.height % nb_rows { 1 } else { 0 };

            let nb_same_color;
            let nb_other_color;
            if vertical_alternate_color {
                nb_same_color = nb_horizontal_patterns.div_ceil(2);
                nb_other_color = nb_horizontal_patterns - nb_same_color;
            } else {
                nb_same_color = nb_horizontal_patterns;
                nb_other_color = 0;
            }

            let nb_same_total;
            let nb_other_total;
            if horizontal_alternate_color {
                let nb_same_rows = nb_vertical_patterns.div_ceil(2);
                let nb_other_rows = nb_vertical_patterns - nb_same_rows;
                nb_same_total = nb_same_color * nb_same_rows + nb_other_color * nb_other_rows;
                nb_other_total = nb_same_color * nb_other_rows + nb_other_color * nb_same_rows;
            } else {
                nb_same_total = nb_same_color * nb_vertical_patterns;
                nb_other_total = nb_other_color * nb_vertical_patterns;
            }
            tile_counts[tile_color] += nb_same_total;
            tile_counts[1 - tile_color] += nb_other_total;
        }
    }
    (tile_counts[0] as u64, tile_counts[1] as u64)
}
