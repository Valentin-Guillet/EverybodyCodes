use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> i32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    let mut lines = data.lines();
    let needles: Vec<&str> = lines.next().unwrap()[6..].split(',').collect();
    lines.next().unwrap();
    let haystacks: Vec<&str> = lines.collect();

    match args.part {
        1 => count_words(&haystacks, &needles),
        2 => count_runes(&haystacks, &needles),
        3 => find_words_in_grid(&haystacks, &needles),
        _ => unreachable!(),
    }
}

fn count_words(haystacks: &[&str], needles: &[&str]) -> i32 {
    let mut ans = 0;
    for haystack in haystacks {
        for needle in needles {
            ans += haystack
                .as_bytes()
                .windows(needle.len())
                .filter(|&w| w == needle.as_bytes())
                .count() as i32;
        }
    }
    ans
}

fn count_runes(haystacks: &[&str], needles: &[&str]) -> i32 {
    let mut ans = 0;
    for haystack in haystacks {
        let haystack = haystack.as_bytes();
        let n = haystack.len();
        let mut rune_used = vec![false; n];
        for needle in needles {
            let needle = needle.as_bytes();
            let m = needle.len();
            for i in 0..(n - m + 1) {
                if haystack[i..i + m] != *needle
                    && haystack[i..i + m].iter().ne(needle.iter().rev())
                {
                    continue;
                }
                for j in 0..m {
                    if !rune_used[i + j] {
                        ans += 1;
                    }
                    rune_used[i + j] = true;
                }
            }
        }
    }
    ans
}

fn find_words_in_grid(grid: &[&str], needles: &[&str]) -> i32 {
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for needle in needles {
                mark_word(grid, needle, row, col, &mut seen);
            }
        }
    }
    seen.iter()
        .map(|row| row.iter().filter(|&v| *v).count() as i32)
        .sum()
}

fn mark_word(grid: &[&str], needle: &str, row: usize, col: usize, seen: &mut [Vec<bool>]) {
    let needle = needle.as_bytes();
    if grid[row].as_bytes()[col] != needle[0] {
        return;
    }

    for dir in 0..4 {
        if dir == 0 && row < needle.len() - 1 {
            continue;
        }
        if dir == 2 && (grid.len() < needle.len() || row > grid.len() - needle.len()) {
            continue;
        }
        if (1..needle.len()).any(|i| {
            let (row, col) = get_neighbor(grid, row, col, i, dir);
            grid[row].as_bytes()[col] != needle[i]
        }) {
            continue;
        }

        seen[row][col] = true;
        for i in 1..needle.len() {
            let (row, col) = get_neighbor(grid, row, col, i, dir);
            seen[row][col] = true;
        }
    }
}

fn get_neighbor(grid: &[&str], row: usize, col: usize, i: usize, dir: u8) -> (usize, usize) {
    match dir {
        0 => (row - i, col),                                   // NORTH
        1 => (row, (col + i) % grid[0].len()),                 // EAST
        2 => (row + i, col),                                   // SOUTH
        3 => (row, (col + grid[0].len() - i) % grid[0].len()), // WEST
        _ => unreachable!(),
    }
}
