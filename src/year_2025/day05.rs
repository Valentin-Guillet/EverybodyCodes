use crate::args::RunArgs;

use std::{cmp::Ordering, fs::read_to_string, iter::zip};

fn get_level((a, b, c): &(u64, u64, u64)) -> u64 {
    if *c > 0 {
        100 * a + 10 * b + c
    } else {
        10 * a + b
    }
}

struct Sword {
    identifier: u64,
    fishbone: Vec<(u64, u64, u64)>,
}

impl Sword {
    fn from(input: &str) -> Self {
        let index = input.find(':').unwrap();
        let identifier = input[..index].parse().unwrap();
        let mut sword = Self {
            identifier,
            fishbone: Vec::new(),
        };
        let numbers = input
            .get(index + 1..)
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u64>().unwrap());
        for number in numbers {
            let mut placed = false;
            for segment in sword.fishbone.iter_mut() {
                if segment.0 == 0 && number < segment.1 {
                    segment.0 = number;
                    placed = true;
                    break;
                } else if segment.2 == 0 && number > segment.1 {
                    segment.2 = number;
                    placed = true;
                    break;
                }
            }
            if !placed {
                sword.fishbone.push((0, number, 0));
            }
        }
        sword
    }

    fn get_quality(&self) -> u64 {
        let mut quality = 0;
        for (_, n, _) in &self.fishbone {
            quality = 10 * quality + n;
        }
        quality
    }

    fn cmp(&self, rhs: &Self) -> Ordering {
        self.get_quality()
            .cmp(&rhs.get_quality())
            // .then(self.fishbone.cmp(&rhs.fishbone))
            .then_with(|| {
                for (f1, f2) in zip(&self.fishbone, &rhs.fishbone) {
                    let lvl1 = get_level(&f1);
                    let lvl2 = get_level(&f2);
                    let cmp = lvl1.cmp(&lvl2);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                Ordering::Equal
            })
            .then(self.identifier.cmp(&rhs.identifier))
    }
}

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let swords: Vec<Sword> = data.lines().map(Sword::from).collect();

    match args.part {
        1 => swords[0].get_quality(),
        2 => get_quality_diff(&swords),
        3 => get_checksum(swords),
        _ => unreachable!(),
    }
}

fn get_quality_diff(swords: &[Sword]) -> u64 {
    let mut min_quality = u64::MAX;
    let mut max_quality = 0;
    for sword in swords {
        let quality = sword.get_quality();
        if quality > max_quality {
            max_quality = quality;
        }
        if quality < min_quality {
            min_quality = quality;
        }
    }
    max_quality - min_quality
}

fn get_checksum(mut swords: Vec<Sword>) -> u64 {
    swords.sort_unstable_by(|a, b| b.cmp(a));
    swords
        .iter()
        .enumerate()
        .map(|(i, sword)| (i + 1) as u64 * sword.identifier)
        .sum()
}
