use std::fs;

fn next_day(arr: [usize; 9]) -> [usize; 9]{
    return [
        arr[1],
        arr[2],
        arr[3],
        arr[4],
        arr[5],
        arr[6],
        arr[7] + arr[0],
        arr[8],
        arr[0],
    ];
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day06.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let mut input_lines = input_file
        .trim()
        .split(|x| x == '\n');

    let first_line = input_lines.next().expect("Can't parse input");

    let mut input_fish = first_line.trim().split(|x| x == ',');

    let mut fish_timer = [0usize; 9];

    while let Some(fish) = input_fish.next(){
        let fish = fish.parse::<usize>().expect("cannot parse input number");
        match fish {
            0..=8 => {
                fish_timer[fish] += 1;
            }
            _ => {
                eprintln!("fish with invalid life cycle in input: {fish}");
                std::process::exit(1);
            }
        }
    }

    let starting_fish_timer = fish_timer.clone();

    for _ in 0..80 {
        fish_timer = next_day(fish_timer);
    }

    let result : usize = fish_timer.iter().sum();

    println!("Result part 1: {result}");

    fish_timer = starting_fish_timer.clone();

    for _ in 0..256 {
        fish_timer = next_day(fish_timer);
    }

    let result : usize = fish_timer.iter().sum();

    println!("Result part 2: {result}");


    //println!("Result part 1: {result}");

}
