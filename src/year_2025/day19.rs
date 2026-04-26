use crate::args::RunArgs;

use std::fs::read_to_string;

#[derive(Clone)]
struct Range {
    min: u32,
    max: u32,
}

struct Wall {
    ahead: u32,
    ranges: Vec<Range>,
}

pub fn run(args: &RunArgs) -> u32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let mut walls = parse_walls(&data);

    get_fewest_flap_nb(&mut walls)
}

fn parse_walls(data: &str) -> Vec<Wall> {
    let mut walls: Vec<Wall> = vec![Wall {
        ahead: 0,
        ranges: vec![Range { min: 0, max: 0 }],
    }];
    for line in data.trim().lines() {
        let numbers: Vec<u32> = line.split(',').map(|d| d.parse().unwrap()).collect();
        let ahead = numbers[0];
        let new_range = Range {
            min: numbers[1],
            max: numbers[1] + numbers[2] - 1,
        };
        if let Some(wall) = walls.last_mut()
            && wall.ahead == ahead
        {
            wall.ranges.push(new_range);
        } else {
            walls.push(Wall {
                ahead: ahead,
                ranges: vec![new_range],
            });
        }
    }
    walls
}

fn merge_ranges(ranges: &mut Vec<Range>) {
    let mut i = 0;
    while i < ranges.len() {
        let mut j = i + 1;
        while j < ranges.len() {
            if ranges[i].min > ranges[j].max || ranges[j].min > ranges[i].max {
                j += 1;
                continue;
            }
            ranges[i].min = ranges[i].min.min(ranges[j].min);
            ranges[i].max = ranges[i].max.max(ranges[j].max);
            ranges.swap_remove(j);
        }
        i += 1;
    }
}

fn extend_ranges(ranges: &mut Vec<Range>, advance: u32) {
    for range in ranges.iter_mut() {
        range.min = range.min.saturating_sub(advance);
        range.max += advance;
    }
    merge_ranges(ranges);
}

fn intersect_ranges(ranges1: &mut Vec<Range>, ranges2: &[Range]) {
    let mut new_ranges = Vec::new();
    for range1 in ranges1.iter() {
        for range2 in ranges2 {
            if range1.min > range2.max || range2.min > range1.max {
                continue;
            }
            new_ranges.push(Range {
                min: range1.min.max(range2.min),
                max: range1.max.min(range2.max),
            });
        }
    }
    merge_ranges(&mut new_ranges);
    *ranges1 = new_ranges;
}

fn restrict_feasibility<'a>(mut walls: impl Iterator<Item = &'a mut Wall>) {
    let Wall {
        ahead: cur_pos,
        ranges: cur_ranges,
    } = walls.next().unwrap();
    let mut cur_pos = *cur_pos;
    let mut cur_ranges = cur_ranges.to_vec();

    for Wall { ahead, ranges } in walls {
        extend_ranges(&mut cur_ranges, ahead.abs_diff(cur_pos));
        intersect_ranges(&mut cur_ranges, ranges);
        *ranges = cur_ranges.clone();
        cur_pos = *ahead;
    }
}

fn get_fewest_flap_nb(walls: &mut [Wall]) -> u32 {
    restrict_feasibility(walls.iter_mut());
    restrict_feasibility(walls.iter_mut().rev());

    let mut flap_nb = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;
    let path = walls.iter().map(|wall| (wall.ahead, wall.ranges[0].min));
    for (step_x, step_y) in path {
        let target_y = if (step_x - pos_x) % 2 == step_y.abs_diff(pos_y) % 2 {
            step_y
        } else {
            step_y + 1
        };

        // flap to get to the minimum height of the hole in next wall
        if target_y > pos_y {
            flap_nb += target_y - pos_y;
        }
        pos_x += target_y.abs_diff(pos_y);

        // flap to keep height
        flap_nb += (step_x - pos_x) / 2;

        pos_x = step_x;
        pos_y = target_y;
    }

    flap_nb
}
