use std::fs;
use std::collections::HashMap;

fn part1_die_gen(i: u32) -> u32 {
    return 3*(3*i + 1) + 3;
}

fn count_universe(
    turn: bool,
    p1: u8,
    p2: u8,
    s1: u8,
    s2: u8,
    memo: &mut HashMap::<(bool,u8,u8,u8,u8), [u64; 2]>
) -> [u64; 2] {
    if s1 >= 21 {
        return [1, 0];
    }
    if s2 >= 21 {
        return [0, 1];
    }
    if memo.contains_key(&(turn, p1, p2, s1, s2)) {
        return memo.get(&(turn, p1, p2, s1, s2)).expect("").clone();
    }
    if memo.contains_key(&(!turn, p2, p1, s2, s1)) {
        let inv_result = memo.get(&(!turn, p2, p1, s2, s1)).expect("").clone();
        return [inv_result[1], inv_result[0]];
    }
    match turn {
        false => {
            let mut u1 = 0;
            let mut u2 = 0;
            for d1 in 1..=3 {
                for d2 in 1..=3{
                    for d3 in 1..=3{
                        let [partial1, partial2] = count_universe(
                            true,
                            ((p1+(d1+d2+d3)-1)%10)+1,
                            p2,
                            s1+((p1+(d1+d2+d3)-1)%10)+1,
                            s2,
                            memo
                        );
                        u1 += partial1;
                        u2 += partial2;
                    }
                }
            }
            let result = [u1, u2];
            memo.insert((turn, p1, p2, s1, s2), result);
            return result;
        }
        true => {
            let mut u1 = 0;
            let mut u2 = 0;
            for d1 in 1..=3 {
                for d2 in 1..=3{
                    for d3 in 1..=3{
                        let [partial1, partial2] = count_universe(
                            false,
                            p1,
                            ((p2+(d1+d2+d3)-1)%10)+1,
                            s1,
                            s2+((p2+(d1+d2+d3)-1)%10)+1,
                            memo
                        );
                        u1 += partial1;
                        u2 += partial2;
                    }
                }
            }
            let result = [u1, u2];
            memo.insert((turn, p1, p2, s1, s2), result);
            return result;
        }
    }
}

fn main() {
    println!("Starting...");

    let file_name = "./input/day21.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let input_lines = input_file
    .trim()
    .split(|x| x == '\n')
    .collect::<Vec<&str>>();

    let player_1_start : u32 = input_lines
                            .get(0)
                            .expect("invalid input")
                            .split(|x| x == ' ')
                            .nth(4)
                            .expect("invalid input")
                            .parse()
                            .expect("invalid input");

    let player_2_start : u32 = input_lines
                            .get(1)
                            .expect("invalid input")
                            .split(|x| x == ' ')
                            .nth(4)
                            .expect("invalid input")
                            .parse()
                            .expect("invalid input");

    let mut player_1_pos = player_1_start;
    let mut player_2_pos = player_2_start;
    let mut player_1_score = 0;
    let mut player_2_score = 0;
    let mut rolls = 0;
    let mut losing_score = 0;

    loop {
        player_1_pos = ((player_1_pos + part1_die_gen(rolls) - 1)%10)+1;
        player_1_score += player_1_pos;
        rolls += 1;
        if player_1_score >= 1000 {
            losing_score = player_2_score;
            break;
        }
        player_2_pos = ((player_2_pos + part1_die_gen(rolls) - 1)%10)+1;
        player_2_score += player_2_pos;
        rolls += 1;
        if player_2_score >= 1000 {
            losing_score = player_1_score;
            break;
        }
    }

    println!("Result part 1: {}", losing_score*rolls*3);

    let mut memo = HashMap::<(bool,u8,u8,u8,u8), [u64; 2]>::new();
    let [u1, u2] = count_universe(false, player_1_start as u8, player_2_start as u8, 0, 0, &mut memo);

    let result = if u2 > u1 {u2} else {u1};

    println!("Result part 2: {}", result);
}
