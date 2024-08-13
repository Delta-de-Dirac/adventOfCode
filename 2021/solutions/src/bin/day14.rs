use std::fs;
use std::collections::HashMap;

fn grow(polymer: Vec<char>, polymap: &HashMap<[char; 2], char>) -> Vec<char>{
    let mut result : Vec<char> = Vec::new();
    for w in polymer.windows(2){
        let a = w.get(0).expect("Invalid window");
        let b = w.get(1).expect("Invalid window");
        result.push(*a);
        if let Some(c) = polymap.get(&[*a,*b]) {
            result.push(*c);
        }
    }
    result.push(*polymer.last().expect("got empty polymer"));
    return result;
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day14.txt";

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

    let mut lines_i = input_lines.iter();

    let mut polymer = lines_i
    .next()
    .expect("can't parse first line")
    .trim()
    .chars()
    .collect::<Vec<char>>();

    let mut lines_i = lines_i.skip(1);

    let mut polymap : HashMap<[char; 2], char> = HashMap::new();

    for line in lines_i {
        let line = line.split(|x| x == ' ').collect::<Vec<&str>>();

        let key : [char; 2] = line.get(0)
        .expect("Can't parse input")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("Can't parse input");

        let value = line.get(2)
        .expect("Can't parse input")
        .chars()
        .next()
        .expect("Can't parse input");

        polymap.insert(key, value);
    }

    for _ in 0..10 {
        polymer = grow(polymer, &polymap);
    }

    let el_freq = polymer.iter().fold(HashMap::new(), |mut map, x| {
        map.entry(x)
        .and_modify(|frq| *frq+= 1usize)
        .or_insert(1usize);
        map
    });

    let freq_vals = el_freq.values();

    let min_val = freq_vals.clone().min().expect("no elements in polymer");
    let max_val = freq_vals.clone().max().expect("no elements in polymer");

    let result = max_val - min_val;

    println!("Result part 1: {el_freq:#?}");
}
