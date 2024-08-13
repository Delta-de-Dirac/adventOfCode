use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
enum Fold {
    FoldX(u32),
    FoldY(u32),
}

fn print_dots(dots: &HashSet<[u32; 2]>){
    let mut max_x = 0;
    let mut max_y = 0;
    for dot in dots.iter(){
        max_x = if dot[0] > max_x {dot[0]} else {max_x};
        max_y = if dot[1] > max_y {dot[1]} else {max_y};
    }
    for l in 0..=max_y {
        for c in 0..=max_x {
            if dots.contains(&[c,l]) {print!("#")}
            else {print!(" ")}
        }
    print!("\n");
    }
}

fn fold_dots(dots: &mut HashSet<[u32; 2]>, f: &Fold) {
    match f {
        Fold::FoldX(f_val) => {
            for dot in dots.clone().iter() {
                if dot[0] > *f_val {
                    dots.remove(dot);
                    dots.insert([2*f_val-dot[0], dot[1]]);
                }
            }
        },
        Fold::FoldY(f_val) => {
            for dot in dots.clone().iter() {
                if dot[1] > *f_val {
                    dots.remove(dot);
                    dots.insert([dot[0], 2*f_val-dot[1]]);
                }
            }
        },
    };
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day13.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let input_lines = input_file
    .trim()
    .split(|x| x == '\n');

    let mut dots : HashSet<[u32 ;2]> = HashSet::new();
    let mut folds : VecDeque<Fold> = VecDeque::new();
    let mut parsing_dots = true;
    for line in input_lines {
        if parsing_dots { //dots
            if line.trim().is_empty() {
                parsing_dots = false;
                continue;
            }
            let mut line = line.split(|x| x == ',');
            let x = line.next().expect("Error parsing dot x");
            let y = line.next().expect("Error parsing dot y");
            dots.insert(
                [x.parse::<u32>().expect("Error parsing dot x"),
                 y.parse::<u32>().expect("Error parsing dot y")]);
        } else {
            let mut line = line.split(|x| x == ' ' || x == '=').skip(2);
            match line.next().expect("Invalid fold line"){
                "x" => {
                    let value = line.next().expect("Invalid fold value");
                    let value = value.parse::<u32>().expect("Invalid fold value");
                    folds.push_back(Fold::FoldX(value))
                },
                "y" => {
                    let value = line.next().expect("Invalid fold value");
                    let value = value.parse::<u32>().expect("Invalid fold value");
                    folds.push_back(Fold::FoldY(value))
                },
                other => {eprintln!("got \"{other}\" expecting axis")},
            }
        }
    }

    let first_fold = folds.pop_front().expect("No folds found at input");

    fold_dots(&mut dots, &first_fold);

    let result = dots.len();

    println!("Result part 1: {result}");

    while let Some(fold) = folds.pop_front() {
        fold_dots(&mut dots, &fold);
    }
    println!("Result part 2:");
    print_dots(&dots);
}
