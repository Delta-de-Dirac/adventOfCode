use std::fs;

fn is_valid(dx: i32, dy: i32, x_from: i32, x_to: i32, y_from: i32, y_to: i32) -> bool {
    let x_from = x_from - dx;
    let x_to = x_to - dx;
    let y_from =  y_from - dy;
    let y_to = y_to - dy;
    if ((x_from..=x_to).contains(&0)) && ((y_from..=y_to).contains(&0)) {return true;}
    if x_to < 0 {return false;}
    if y_from > 0 {return false;}

    let dx = if dx > 0 {dx - 1} else {0};
    let dy = dy - 1;
    return is_valid(dx, dy, x_from, x_to, y_from, y_to);
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day17.txt";

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

    let first_line = input_lines.get(0).expect("Invalid input");
    let words = first_line
    .split(|x: char| ['=','.',','].contains(&x))
    .collect::<Vec<&str>>();

    let x_from : i32 = words.get(1).expect("No word for x_from found").parse().expect("Cannot parse x_from word to i32");
    let x_to : i32 = words.get(3).expect("No word for x_to found").parse().expect("Cannot parse x_to word to i32");
    let y_from : i32 = words.get(5).expect("No word for y_from found").parse().expect("Cannot parse y_from word to i32");
    let y_to : i32 = words.get(7).expect("No word for y_to found").parse().expect("Cannot parse y_to word to i32");

    let max_dy = -1*y_from-1;

    //Part 1 is simple equation solving.
    println!("Result part 1: {result}", result = ((max_dy)*(max_dy)+(max_dy))/2);

    let min_dx = {let mut candidate = 0; while candidate*candidate+candidate < 2*x_from {candidate+=1} candidate};
    let max_dx = x_to;

    let min_dy = y_from;

    let mut result = 0;

    for dy in min_dy..=max_dy {
        for dx in min_dx..=max_dx {
            if is_valid(dx, dy, x_from, x_to, y_from, y_to){
                result += 1;
            }
        }
    }

    println!("Result part 2: {result}");
}
