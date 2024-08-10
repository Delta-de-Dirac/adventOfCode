use std::fs;

#[derive(Debug)]
struct Board {
    nums: [[i32; 5]; 5],
    score: [[bool; 5]; 5],
    won: bool,
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day04.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let mut input_lines = input_file
        .trim()
        .split(|x| x == '\n');

    let first_line = input_lines.next().expect("No lines at input file");

    let mut draw_nums = first_line
        .split(|x| x == ',')
        .map(|x|
            x.parse::<i32>()
            .expect("Can't parse input")
        );

    let mut boards : Vec<Board> = Vec::new();

    while let Some(_) = input_lines.next(){
        let mut new_board = Board {
            nums: [[0; 5]; 5],
            score: [[false; 5]; 5],
            won: false,
        };
        for i in 0..5 {
            if let Some(line) = input_lines.next(){
                for (j, number) in line.split_whitespace()
                                    .map(|x| x.parse::<i32>()
                                    .expect("Can't parse board number"))
                                    .enumerate(){
                    new_board.nums[i][j] = number;
                }
            } else {
                break;
            }
        }
        println!("{new_board:#?}");
        boards.push(new_board);
    }

}
