use std::fs;

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


    let result = 0;

    println!("Result part 1: {result}");

}
