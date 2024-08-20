use std::fs;

fn part1_die_gen(i: u32) -> u32 {
    return 3*(3*i + 1) + 3;
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

    const BOARD_SIZE : u32 = 10;

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

}
