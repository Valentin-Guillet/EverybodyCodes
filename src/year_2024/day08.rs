use crate::args::RunArgs;

use std::fs::read_to_string;

pub fn run(args: &RunArgs) -> u64 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let input = data.trim().parse().unwrap();

    match args.part {
        1 => build_pyramid(input),
        2 => build_tower(input),
        3 => build_shrine(input),
        _ => unreachable!(),
    }
}

fn build_pyramid(nb_blocks: u64) -> u64 {
    let mut layer = 1;
    while layer * layer < nb_blocks {
        layer += 1;
    }
    (2 * layer - 1) * (layer * layer - nb_blocks)
}

fn build_tower(nb_priests: u64) -> u64 {
    const NB_ACCOLYTES: u64 = 1111;
    const NB_MARBLE: u64 = 20240000;

    let mut width = 3;
    let mut prev_thickness = 1;
    let mut total_blocks = 1;
    loop {
        let thickness = (prev_thickness * nb_priests) % NB_ACCOLYTES;
        total_blocks += thickness * width;

        if total_blocks > NB_MARBLE {
            break;
        }

        width += 2;
        prev_thickness = thickness;
    }

    width * (total_blocks - NB_MARBLE)
}

fn build_shrine(nb_priests: u64) -> u64 {
    const NB_ACCOLYTES: u64 = 10;
    const NB_MARBLE: u64 = 202400000;

    let mut columns = vec![1];
    let mut width = 3;
    let mut prev_thickness = 1;
    let mut total_blocks = 1;
    let mut nb_blocks_to_remove: u64;
    loop {
        let thickness = (prev_thickness * nb_priests) % NB_ACCOLYTES + NB_ACCOLYTES;
        total_blocks += thickness * width;
        for height in &mut columns {
            *height += thickness;
        }

        nb_blocks_to_remove = (nb_priests * width * columns[0]) % NB_ACCOLYTES;
        for height in columns.iter().skip(1) {
            nb_blocks_to_remove += 2 * ((nb_priests * width * height) % NB_ACCOLYTES);
        }
        columns.push(thickness);

        if total_blocks - nb_blocks_to_remove > NB_MARBLE {
            break;
        }

        width += 2;
        prev_thickness = thickness;
    }
    total_blocks - nb_blocks_to_remove - NB_MARBLE
}
