use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

fn isSmallCave(name: &str) -> bool {
    name != "start" && name != "end" && name.chars().all(|x| x.is_lowercase())
}

fn count_paths(name: String,
               c_system: &HashMap<String, HashSet<String>>,
               unvisited: &HashSet<String>
               ) -> u32 {
    match name.as_str() {
        "end" => {1}
        _ => {
            let mut acc = 0;
            for way in c_system.get(&name).expect("Name not found in c_system"){
                match way.as_str() {
                    "start" => {continue;}
                    w if isSmallCave(w) && !unvisited.contains(w) => {continue;}
                    w if isSmallCave(w) && unvisited.contains(w) => {
                        let mut new_unv = unvisited.clone();
                        new_unv.remove(w);
                        acc += count_paths(w.to_string(), c_system, &new_unv);
                    }
                    w => {
                        acc += count_paths(w.to_string(), c_system, unvisited);
                    }
                };
            }
            acc
        }
    }
}

fn count_paths_part2(name: String,
               c_system: &HashMap<String, HashSet<String>>,
               unvisited: &HashSet<String>,
               allow_twice: bool
) -> u32 {
    match name.as_str() {
        "end" => {1}
        _ => {
            let mut acc = 0;
            for way in c_system.get(&name).expect("Name not found in c_system"){
                match way.as_str() {
                    "start" => {continue;}
                    w if isSmallCave(w) && !unvisited.contains(w) && !allow_twice => {continue;}
                    w if isSmallCave(w) && !unvisited.contains(w) && allow_twice => {
                        acc += count_paths_part2(w.to_string(), c_system, unvisited, false);
                    }
                    w if isSmallCave(w) && unvisited.contains(w) => {
                        let mut new_unv = unvisited.clone();
                        new_unv.remove(w);
                        acc += count_paths_part2(w.to_string(), c_system, &new_unv, allow_twice);
                    }
                    w => {
                        acc += count_paths_part2(w.to_string(), c_system, unvisited, allow_twice);
                    }
                };
            }
            acc
        }
    }
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day12.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let input_lines = input_file
                            .trim()
                            .split(|x| x == '\n');

    let mut c_system : HashMap<String, HashSet<String>> = HashMap::new();
    let mut unvisited: HashSet<String> = HashSet::new();

    for line in input_lines{
        let line = line
                       .trim()
                       .split(|x| x == '-')
                       .map(|x| x.to_string())
                       .collect::<Vec<String>>();
        let c1 = line.get(0).expect("invalid input");
        let c2 = line.get(1).expect("invalid input");
        if isSmallCave(c1) {
            unvisited.insert(c1.clone());
        }
        if isSmallCave(c2){
            unvisited.insert(c2.clone());
        }
        c_system.entry(c1.to_string())
                .and_modify(|set| {set.insert(c2.to_string());})
                .or_insert(HashSet::<String>::from([c2.to_string()]));
        c_system.entry(c2.to_string())
                .and_modify(|set| {set.insert(c1.to_string());})
                .or_insert(HashSet::<String>::from([c1.to_string()]));
    }

    let result = count_paths("start".to_string(), &c_system, &unvisited);
    println!("Result part 1: {result}");
    let result = count_paths_part2("start".to_string(), &c_system, &unvisited, true);
    println!("Result part 2: {result}");
}
