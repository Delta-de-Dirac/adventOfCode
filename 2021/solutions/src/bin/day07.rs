use std::fs;
use std::collections::BTreeMap;

fn part2_fuel_at(crabs_freq: &BTreeMap<usize, usize>, pos: usize) -> i64 {
    let mut total_fuel = 0i64;
    for crab in crabs_freq.iter() {
        let dist = if pos >= *crab.0 {pos - *crab.0} else {*crab.0 - pos};
        total_fuel += ((dist*dist + dist)*(*crab.1)) as i64;
    }
    total_fuel = total_fuel/2;
    return total_fuel;
}


fn main(){
    println!("Starting...");

    let file_name = "./input/day07.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let mut input_lines = input_file
                            .trim()
                            .split(|x| x == '\n');

    let first_line = input_lines
                        .next()
                        .expect("Error parsing input text")
                        .trim()
                        .split(|x| x == ',');

    let crabs_freq = first_line
                    .fold(BTreeMap::new(), |mut b_tree, x| {
                                            b_tree.entry(x.parse::<usize>().expect("Can't parse input"))
                                            .and_modify(|frq| *frq+= 1usize)
                                            .or_insert(1usize);
                                            b_tree
                                        });

    let mut crabs = crabs_freq.iter();

    let mut needed_fuel = 0usize;

    let first_crab = crabs.next().expect("Can't find first crab");

    let first_crab_pos : usize = *first_crab.0;

    let mut crabs_to_the_right = 0usize;

    while let Some(crab) = crabs.next() {
        needed_fuel += (*crab.0 - first_crab_pos)*(*crab.1);
        crabs_to_the_right += *crab.1;
    }

    let mut crabs_to_the_left = *first_crab.1;

    crabs = crabs_freq.iter();

    _ = crabs.next();

    let mut current_pos = first_crab_pos;

    while crabs_to_the_right > crabs_to_the_left {
        if let Some(crab) = crabs.next() {
            let last_pos = current_pos;
            current_pos = *crab.0;

            needed_fuel -= (crabs_to_the_right - crabs_to_the_left) * (current_pos - last_pos);

            crabs_to_the_right -= *crab.1;
            crabs_to_the_left += *crab.1;
        } else {
            eprintln!("Error");
            std::process::exit(1);
        }
    }



    println!("Result part 1: {needed_fuel}");

    /* PART 2 */

    //TODO: find some smart optimitaztion, seems likely to exist, but below is very quick for input given at advent day.

    let last_crab_pos : usize = *crabs_freq.iter().last().expect("Error").0;

    let mut least_fuel = part2_fuel_at(&crabs_freq, last_crab_pos);

    for pos in first_crab_pos..last_crab_pos {
        let candidate = part2_fuel_at(&crabs_freq, pos);
        least_fuel = if candidate < least_fuel {candidate} else {least_fuel};
    }

    println!("Result part 2: {least_fuel}");
}
