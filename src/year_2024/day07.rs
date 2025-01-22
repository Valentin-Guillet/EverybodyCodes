use crate::args::RunArgs;

use std::fs::read_to_string;

type Pos = (usize, usize);
type Plan = (char, Vec<char>);

pub fn run(args: &RunArgs) -> i32 {
    let data = read_to_string(&args.input_file).expect("Error opening input file");

    let (race_track, plans) = parse_input(data);
    let race_track = race_track.chars().collect::<Vec<char>>();

    if args.part < 3 {
        let nb_loops = if args.part == 1 { 1 } else { 10 };
        let ranked_plans = rank_plans(&race_track, &plans, nb_loops);
        println!("Ranked plans: {}", String::from_iter(ranked_plans));
        return 0;
    }

    count_winning_plans(&race_track, &plans[0], 2024).into()
}

fn parse_input(data: String) -> (String, Vec<Plan>) {
    let mut racetrack_vec: Vec<Vec<char>> = Vec::new();
    let mut plans = Vec::new();

    let mut is_racetrack = true;
    for line in data.lines() {
        if line.is_empty() {
            is_racetrack = false;
            continue;
        }

        if is_racetrack {
            racetrack_vec.push(line.chars().collect());
        } else {
            let (name, plan) = line.split_once(':').unwrap();
            plans.push((
                name.chars().next().unwrap(),
                plan.split(',').map(|c| c.chars().next().unwrap()).collect(),
            ));
        }
    }
    (parse_racetrack(racetrack_vec), plans)
}

fn parse_racetrack(racetrack_vec: Vec<Vec<char>>) -> String {
    let mut racetrack = String::new();
    let mut prev_pos = (0, 0);
    let mut pos = (0, 1);
    while pos != (0, 0) {
        racetrack.push(racetrack_vec[pos.0][pos.1]);
        let tmp_pos = pos;
        pos = get_next_pos(&racetrack_vec, pos, prev_pos);
        prev_pos = tmp_pos;
    }
    racetrack.push('S');
    racetrack
}

fn get_next_pos(racetrack_vec: &[Vec<char>], pos: Pos, prev_pos: Pos) -> Pos {
    for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let next_x = pos.0.wrapping_add_signed(dx);
        let next_y = pos.1.wrapping_add_signed(dy);

        if (0..racetrack_vec.len()).contains(&next_x)
            && (0..racetrack_vec[0].len()).contains(&next_y)
            && racetrack_vec[next_x][next_y] != ' '
            && (next_x != prev_pos.0 || next_y != prev_pos.1)
        {
            return (next_x, next_y);
        }
    }
    unreachable!()
}

fn get_plan_score(track: &[char], plan: &[char], nb_loops: u16) -> u64 {
    let mut total_power = 0;

    let mut states: Vec<(usize, u64, u64)> = Vec::new();
    let mut power = 10;
    let mut plan_step = 0;
    for loop_id in 0..nb_loops {
        if !states.is_empty() && states[0].0 == plan_step {
            for i in 0..nb_loops - loop_id {
                let state_id = ((loop_id + i) % states.len() as u16) as usize;
                let (_, loop_sum, loop_diff) = states[state_id];
                total_power += power * track.len() as u64 + loop_sum;
                power += loop_diff;
            }

            return total_power;
        }
        let init_plan_step = plan_step;

        let mut loop_sum: i32 = 0;
        let mut loop_diff: i32 = 0;
        for track_action in track {
            loop_diff = match track_action {
                '+' => loop_diff + 1,
                '-' => loop_diff - 1,
                _ => match plan[plan_step % plan.len()] {
                    '+' => loop_diff + 1,
                    '-' => loop_diff - 1,
                    _ => loop_diff,
                },
            };
            loop_sum += loop_diff;
            plan_step = (plan_step + 1) % plan.len();
        }

        let loop_sum = loop_sum as u64;
        let loop_diff = loop_diff as u64;
        states.push((init_plan_step, loop_sum, loop_diff));
        total_power += power * track.len() as u64 + loop_sum;
        power += loop_diff;
    }

    total_power
}

fn rank_plans(track: &[char], plans: &Vec<Plan>, nb_loops: u16) -> Vec<char> {
    let mut plan_scores: Vec<(char, u64)> = Vec::new();
    for (name, plan) in plans {
        let score = get_plan_score(track, plan, nb_loops);
        plan_scores.push((*name, score));
    }
    plan_scores.sort_by(|a, b| b.1.cmp(&a.1));
    plan_scores.iter().map(|(name, _)| *name).collect()
}

fn count_winning_plans(track: &[char], rival_plan: &Plan, nb_loops: u16) -> u16 {
    let rival_score = get_plan_score(track, &rival_plan.1, nb_loops);

    let mut count = 0;
    for plan in generate_all_plans(5, 3, 3) {
        let score = get_plan_score(track, &plan.1, nb_loops);
        if score > rival_score {
            count += 1;
        }
    }
    count
}

fn generate_all_plans(nb_plus: u8, nb_minus: u8, nb_equal: u8) -> Vec<Plan> {
    if nb_plus == 0 && nb_minus == 0 && nb_equal == 0 {
        return Vec::from([(' ', Vec::new())]);
    }
    let mut plans = Vec::new();
    if nb_plus > 0 {
        for mut remaining_plan in generate_all_plans(nb_plus - 1, nb_minus, nb_equal) {
            remaining_plan.1.push('+');
            plans.push(remaining_plan);
        }
    }
    if nb_minus > 0 {
        for mut remaining_plan in generate_all_plans(nb_plus, nb_minus - 1, nb_equal) {
            remaining_plan.1.push('-');
            plans.push(remaining_plan);
        }
    }
    if nb_equal > 0 {
        for mut remaining_plan in generate_all_plans(nb_plus, nb_minus, nb_equal - 1) {
            remaining_plan.1.push('=');
            plans.push(remaining_plan);
        }
    }
    plans
}
