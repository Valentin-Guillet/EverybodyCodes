use crate::args::RunArgs;

use std::fs::read_to_string;

#[derive(Debug)]
struct Scale {
    id: u32,
    colors: [u8; 3],
    shine: Option<u8>,
}

impl Scale {
    fn from(input: &str) -> Self {
        let index = input.find(':').unwrap();
        let id = input[..index].parse().unwrap();
        let mut components = Vec::new();
        for component in input[index + 1..].split(' ') {
            let value = component.chars().fold(0, |value, c| {
                value * 2 + if c.is_uppercase() { 1 } else { 0 }
            });
            components.push(value);
        }
        Self {
            id,
            colors: components[..3].as_array().unwrap().to_owned(),
            shine: components.get(3).copied(),
        }
    }

    fn brightness(&self) -> u16 {
        self.colors.iter().sum::<u8>() as u16
    }
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let scales: Vec<Scale> = data.trim_end().lines().map(Scale::from).collect();

    match args.part {
        1 => scales
            .iter()
            .filter(|scale| scale.colors[1] > scale.colors[0].max(scale.colors[2]))
            .map(|scale| scale.id)
            .sum(),
        2 => {
            scales
                .iter()
                .max_by_key(|scale| (scale.shine, u16::MAX - scale.brightness()))
                .unwrap()
                .id
        }
        3 => get_largest_group_sum(&scales),
        _ => unreachable!(),
    }
}

fn get_largest_group_sum(scales: &[Scale]) -> u32 {
    // Groups are: red matte, red shiny, green matte, green shiny, blue matte, and blue shiny.
    let mut groups: [Vec<&Scale>; 6] = Default::default();
    for scale in scales {
        let mut group_index = match scale.shine {
            Some(0..=30) => 0,
            Some(33..) => 1,
            _ => continue,
        };
        let mut valid = false;
        for i in 0..=2 {
            if scale.colors[i] > scale.colors[(i + 1) % 3].max(scale.colors[(i + 2) % 3]) {
                valid = true;
                group_index += 2 * i as usize;
                break;
            }
        }
        if valid {
            groups[group_index].push(scale);
        }
    }
    groups
        .into_iter()
        .max_by_key(Vec::len)
        .unwrap()
        .iter()
        .map(|scale| scale.id)
        .sum()
}
