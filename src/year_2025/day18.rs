use crate::args::RunArgs;

use std::fs::read_to_string;

struct Branch {
    connection: Option<usize>,
    thickness: i32,
}

struct Plant {
    thickness: u64,
    branches: Vec<Branch>,
}

type Test = Vec<bool>;

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let (plants, test_cases) = build_plant_network(&data);

    match args.part {
        1 | 2 => compute_energy(&plants, &test_cases),
        3 => compute_energy_differences(&plants, &test_cases),
        _ => unreachable!(),
    }
}

fn build_plant_network(input: &str) -> (Vec<Plant>, Vec<Test>) {
    let mut lines = input.lines();
    let mut nb_free_branches = 0;

    // Parse plants
    let mut plants = Vec::new();
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let thickness = line
            .split(' ')
            .last()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse()
            .unwrap();
        let mut branches = Vec::new();
        while let Some(branch_line) = lines.next()
            && !branch_line.is_empty()
        {
            let mut words = branch_line.split(' ');
            let connection = if words.nth(1).unwrap() != "free" {
                Some(words.nth(2).unwrap().parse().unwrap())
            } else {
                nb_free_branches += 1;
                None
            };
            let branch_thickness = words.last().unwrap().parse().unwrap();
            branches.push(Branch {
                connection,
                thickness: branch_thickness,
            });
        }
        plants.push(Plant {
            thickness,
            branches,
        });
    }

    // Parse test cases
    let mut tests: Vec<Test> = Vec::new();
    while let Some(line) = lines.next() {
        let test = line.split(' ').map(|word| word == "1").collect();
        tests.push(test);
    }

    if tests.is_empty() {
        tests.push(vec![true; nb_free_branches]);
    }

    (plants, tests)
}

fn compute_energy_rec(
    plants: &[Plant],
    index: usize,
    test_case: &Test,
    cache: &mut [Option<u64>],
) -> u64 {
    if let Some(energy) = cache[index] {
        return energy;
    }

    let mut total_incoming_energy = 0;
    for branch in &plants[index].branches {
        let incoming_energy = match branch.connection {
            Some(connection) => compute_energy_rec(plants, connection - 1, test_case, cache),
            None => test_case[index] as u64,
        };
        total_incoming_energy += branch.thickness * incoming_energy as i32;
    }
    let mut total_incoming_energy = total_incoming_energy.max(0) as u64;
    if total_incoming_energy < plants[index].thickness {
        total_incoming_energy = 0;
    }
    cache[index] = Some(total_incoming_energy);
    total_incoming_energy
}

fn compute_energy(plants: &[Plant], test_cases: &[Test]) -> u64 {
    let mut total_energy = 0;
    for test_case in test_cases {
        let mut cache = vec![None; plants.len()];
        total_energy += compute_energy_rec(plants, plants.len() - 1, test_case, &mut cache)
    }
    total_energy
}

fn compute_max_energy(plants: &[Plant], nb_free_branches: usize) -> u64 {
    // We can cheat: by observing our input, we can see that it has a very specific structure where
    // - plants 1 to 81 are free
    // - plants 82 to 90 are connected to 1..=9, 10..=18, ..., 73..=81
    // - plants 91 to 99 are connected to 1-10-...-73, 2-11-...-74, ..., 9-18-...-81
    // - plants 100 to 108 are connected to 82&91, 83&92, ..., 90&99
    // - plant 109 is connected to 100..=108
    // So only plants 1..=81 are connected to two other plants, all others are connected to only one
    // Moreover, if we inspect the thickness of the branches connecting the plants 1..=81 to 82..=90
    // and 91..=99, they're all of the same sign, i.e.
    // - sign(thickness(1, 82)) == sign(thickness(1, 91))
    // - sign(thickness(2, 83)) == sign(thickness(2, 92))
    // - ...
    // As we want to maximize all energy flowing in the network, we only have to activate base
    // plants connected through branches with positive thickness, and disable all others
    let mut max_test_case: Test = vec![true; nb_free_branches];
    for plant in plants {
        for &Branch {
            connection,
            thickness,
        } in &plant.branches
        {
            if let Some(plant_id) = connection
                && plant_id < nb_free_branches
            {
                max_test_case[plant_id - 1] = thickness > 0;
            }
        }
    }

    let mut cache = vec![None; plants.len()];
    compute_energy_rec(plants, plants.len() - 1, &max_test_case, &mut cache)
}

fn compute_energy_differences(plants: &[Plant], test_cases: &[Test]) -> u64 {
    let mut energy_diff = 0;
    let max_energy = compute_max_energy(plants, test_cases.len());

    for test_case in test_cases {
        let mut cache = vec![None; plants.len()];
        let energy = compute_energy_rec(plants, plants.len() - 1, test_case, &mut cache);
        if energy > 0 {
            energy_diff += max_energy.saturating_sub(energy);
        }
    }
    energy_diff
}
