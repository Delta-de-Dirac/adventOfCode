use std::fs;
use std::collections::HashMap;
use std::collections::hash_map;

enum ThermalState {
    Single,
    Many,
}

fn insert_thermal_part2(thermals : &mut HashMap::<[i32; 2], ThermalState>,
                 line: [i32; 4]) -> Option<i32> {
    let mut count = 0;
    match (line[0] == line[2], line[1] == line[3]) {
        (true, _) => {
            let (from, to) = if line[1] <= line[3] {(line[1], line[3])} else {(line[3], line[1])};
            for y in from..=to {
                match thermals.entry([line[0], y]) {
                    hash_map::Entry::Vacant(entry) => {
                        entry.insert(ThermalState::Single);
                    },
                    hash_map::Entry::Occupied(mut entry) => {
                        match entry.get() {
                            ThermalState::Single => {
                                count += 1;
                                entry.insert(ThermalState::Many);
                            }
                            _ => {continue;}
                        }
                    },
                }
            }
        }
        (_, true) => {
            let (from, to) = if line[0] <= line[2] {(line[0], line[2])} else {(line[2], line[0])};
            for x in from..=to {
                match thermals.entry([x, line[1]]) {
                    hash_map::Entry::Vacant(entry) => {
                        entry.insert(ThermalState::Single);
                    },
                    hash_map::Entry::Occupied(mut entry) => {
                        match entry.get() {
                            ThermalState::Single => {
                                count += 1;
                                entry.insert(ThermalState::Many);
                            }
                            _ => {continue;}
                        }
                    },
                }
            }
        }
        _ => { //diagonal
            let steps = if line[0] <= line[2] {line[2]-line[0]} else {line[0] - line[2]};
            let dx = (line[2] - line[0])/steps;
            let dy = (line[3] - line[1])/steps;
            let mut x = line[0];
            let mut y = line[1];
            for i in 0..=steps{
                match thermals.entry([x, y]) {
                    hash_map::Entry::Vacant(entry) => {
                        entry.insert(ThermalState::Single);
                    },
                    hash_map::Entry::Occupied(mut entry) => {
                        match entry.get() {
                            ThermalState::Single => {
                                count += 1;
                                entry.insert(ThermalState::Many);
                            }
                            _ => {
                                x+=dx;
                                y+=dy;
                                continue;
                            }
                        }
                    },
                }
                x+=dx;
                y+=dy;
            }
        }
    }
    return Some(count);
}

fn insert_thermal(thermals : &mut HashMap::<[i32; 2], ThermalState>,
                 line: [i32; 4]) -> Option<i32> {
    let mut count = 0;
    match (line[0] == line[2], line[1] == line[3]) {
        (true, _) => {
            let (from, to) = if line[1] <= line[3] {(line[1], line[3])} else {(line[3], line[1])};
            for y in from..=to {
                match thermals.entry([line[0], y]) {
                    hash_map::Entry::Vacant(entry) => {
                        entry.insert(ThermalState::Single);
                    },
                    hash_map::Entry::Occupied(mut entry) => {
                        match entry.get() {
                            ThermalState::Single => {
                                count += 1;
                                entry.insert(ThermalState::Many);
                            }
                            _ => {continue;}
                        }
                    },
                }
            }
        }
        (_, true) => {
            let (from, to) = if line[0] <= line[2] {(line[0], line[2])} else {(line[2], line[0])};
            for x in from..=to {
                match thermals.entry([x, line[1]]) {
                    hash_map::Entry::Vacant(entry) => {
                        entry.insert(ThermalState::Single);
                    },
                    hash_map::Entry::Occupied(mut entry) => {
                        match entry.get() {
                            ThermalState::Single => {
                                count += 1;
                                entry.insert(ThermalState::Many);
                            }
                            _ => {continue;}
                        }
                    },
                }
            }
        }
        _ => {
            return Some(0);
        }
    }
    return Some(count);
}


fn main(){
    println!("Starting...");

    let file_name = "./input/day05.txt";

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

    let mut thermals = HashMap::<[i32; 2], ThermalState>::new();

    let mut result = 0;

    for line in input_lines.iter() {
        let mut line = line.split(|x| x == ',' || x == ' ');
        let x1 = line.next().expect("Error parsing line").parse::<i32>().expect("Error parsing number");
        let y1 = line.next().expect("Error parsing line").parse::<i32>().expect("Error parsing number");
        let _ = line.next().expect("Error parsing line");
        let x2 = line.next().expect("Error parsing line").parse::<i32>().expect("Error parsing number");
        let y2 = line.next().expect("Error parsing line").parse::<i32>().expect("Error parsing number");
        result += insert_thermal(&mut thermals, [x1,y1,x2,y2]).expect("Error inserting thermal");
    }

    println!("Result part 1: {result}");

    let mut thermals = HashMap::<[i32; 2], ThermalState>::new();

    let mut result = 0;

    for line in input_lines.iter() {
        let mut line = line.split(|x| x == ',' || x == ' ');
        let x1 = line.next().expect("Error parsing line").parse::<i32>().expect("Error parsing number");
        let y1 = line.next().expect("Error parsing line").parse::<i32>().expect("Error parsing number");
        let _ = line.next().expect("Error parsing line");
        let x2 = line.next().expect("Error parsing line").parse::<i32>().expect("Error parsing number");
        let y2 = line.next().expect("Error parsing line").parse::<i32>().expect("Error parsing number");
        result += insert_thermal_part2(&mut thermals, [x1,y1,x2,y2]).expect("Error inserting thermal");
    }

    println!("Result part 2: {result}");

}
