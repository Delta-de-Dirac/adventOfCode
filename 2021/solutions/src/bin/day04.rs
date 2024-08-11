use std::fs;

#[derive(Debug)]
struct Board {
    nums: [[i32; 5]; 5],
    score: [[bool; 5]; 5],
    won: bool,
}

fn compute_board_score(board: &Board, draw_num: i32) -> i32{
    let mut sum_false = 0;
    for lx in 0..5 { for cx in 0..5 {
        if board.score[lx][cx] == false {
            sum_false += board.nums[lx][cx];
        }
    }}
    return sum_false * draw_num;
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
        .trim()
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
        boards.push(new_board);
    }

    let mut result = 0;

    'outer: while let Some(draw_num) = draw_nums.next(){
        for board in (&mut boards).into_iter(){
            for l in 0..5 { for c in 0..5 {
                if board.nums[l][c] == draw_num {
                    board.score[l][c] = true;
                    //line win
                    if board.score[l].iter().all(|&x| x == true){
                        result = compute_board_score(&board, draw_num);
                        break 'outer;
                    }
                    //column win
                    if board.score.iter().all(|&x| x[c] == true){
                        result = compute_board_score(&board, draw_num);
                        break 'outer;
                    }
                }
            }}
        }
    }

    println!("Result part 1: {result}");

    let mut draw_nums = first_line
        .trim()
        .split(|x| x == ',')
        .map(|x|
            x.parse::<i32>()
            .expect("Can't parse input")
        );

    for board in (&mut boards).into_iter(){
        for l in 0..5 { for c in 0..5 {
            board.score[l][c] = false;
        }}
    }

    let mut result = 0;

    'outer0: while let Some(draw_num) = draw_nums.next(){
        let valid_boards = (&mut boards).into_iter().filter(|x| x.won == false).collect::<Vec<&mut Board>>();
        let n_boards = valid_boards.len();
        'outer1: for board in valid_boards{
            for l in 0..5 { for c in 0..5 {
                if board.nums[l][c] == draw_num {
                    board.score[l][c] = true;
                    //line win
                    if board.score[l].iter().all(|&x| x == true){
                        board.won = true;
                        if n_boards == 1 {
                            result = compute_board_score(board, draw_num);
                            break 'outer0;
                        }
                        continue 'outer1;
                    }
                    //column win
                    if board.score.iter().all(|&x| x[c] == true){
                        board.won = true;
                        if n_boards == 1 {
                            result = compute_board_score(board, draw_num);
                            break 'outer0;
                        }
                        continue 'outer1;
                    }
                }
            }}
        }
    }

    println!("Result part 2: {result}");
}
