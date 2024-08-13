use std::fs;

fn main(){
    println!("Starting...");

    let file_name = "./input/day10.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let input_lines = input_file
                            .trim()
                            .split(|x| x == '\n');

    const REGUL_SCORE : u32 = 3;
    const SQUAR_SCORE : u32 = 57;
    const CURLY_SCORE : u32 = 1197;
    const ANGLE_SCORE : u32 = 25137;

    let mut total = 0;

    let mut part2_scores : Vec<u64> = Vec::new();

    'outer: for line in input_lines {
        let mut brac_stack : Vec<char> = Vec::new();
        let mut line = line.chars();
        let mut current_block = line.next().expect("No char at line");
        while let Some(ch) = line.next(){
            match ch {
                '(' => {
                    if current_block != 'x' {
                        brac_stack.push(current_block);
                    }
                    current_block = ch;
                },
                '[' => {
                    if current_block != 'x' {
                        brac_stack.push(current_block);
                    }
                    current_block = ch;
                },
                '{' => {
                    if current_block != 'x' {
                        brac_stack.push(current_block);
                    }
                    current_block = ch;
                },
                '<' => {
                    if current_block != 'x' {
                        brac_stack.push(current_block);
                    }
                    current_block = ch;
                },
                ')' => {
                    if current_block == '(' {
                        current_block = brac_stack.pop().unwrap_or('x');
                    } else { //Expected _, but found ) instead.
                        total += REGUL_SCORE;
                        continue 'outer;
                    }
                },
                ']' => {
                    if current_block == '[' {
                        current_block = brac_stack.pop().unwrap_or('x');
                    } else { //Expected _, but found ] instead.
                        total += SQUAR_SCORE;
                        continue 'outer;
                    }
                },
                '}' => {
                    if current_block == '{' {
                        current_block = brac_stack.pop().unwrap_or('x');
                    } else { //Expected _, but found } instead.
                        total += CURLY_SCORE;
                        continue 'outer;
                    }
                },
                '>' => {
                    if current_block == '<' {
                        current_block = brac_stack.pop().unwrap_or('x');
                    } else { //Expected _, but found > instead.
                        total += ANGLE_SCORE;
                        continue 'outer;
                    }
                },
                _ => {
                    eprintln!("Unexpected char at line");
                    std::process::exit(1);
                },
            }
        }

        let mut p2_score : u64 = 0;

        if current_block != 'x' {
            brac_stack.push(current_block);
        }

        while let Some(closing_ch) = brac_stack.pop() {
            p2_score *= 5;
            match closing_ch {
                '(' => {
                    p2_score += 1;
                },
                '[' => {
                    p2_score += 2;
                },
                '{' => {
                    p2_score += 3;
                },
                '<' => {
                    p2_score += 4;
                },
                _ => {
                    eprintln!("Unexpected char at line part 2");
                    std::process::exit(1);
                },
            }
        }
        part2_scores.push(p2_score);
    }

    part2_scores.sort();

    let result_2 = part2_scores.get(part2_scores.len()/2).expect("Error getting result part 2");

    println!("Result part 1: {total}");
    println!("Result part 2: {result_2}");
}
