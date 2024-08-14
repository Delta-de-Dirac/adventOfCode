use std::fs;
use std::collections::HashMap;

fn merge_hash(
    x: HashMap<char, u64>,
    mut y: HashMap<char, u64>
) -> HashMap<char, u64> {
    for (key, val_x) in x.iter() {
        y.entry(*key).and_modify(|val_y| *val_y += val_x).or_insert(*val_x);
    }
    return y;
}

fn count_part2(
    polymer: &Vec<char>,
    polymap: &HashMap<[char; 2], char>,
    t: u32
    ) -> HashMap<char, u64>{

    let mut memo : HashMap<(char, char, u32), HashMap<char, u64>> = HashMap::new();
    let mut result : HashMap<char, u64> = HashMap::new();

    for ch in polymer.iter() {
        result.entry(*ch).and_modify(|v| *v += 1).or_insert(1);
    }

    for w in polymer.windows(2){
        let l = w.get(0).expect("Invalid window");
        let r = w.get(1).expect("Invalid window");

        result = merge_hash(count_generated(l, r, t, polymap, &mut memo), result);
    }
    return result;
}

fn count_generated(
  l: &char,
  r: &char,
  t: u32,
  polymap: &HashMap<[char; 2], char>,
  memo: &mut HashMap<(char, char, u32), HashMap<char, u64>>) -> HashMap<char, u64> {
    if t == 0 {
        return HashMap::new();
    }
    if let Some(result) = memo.get(&(*l, *r, t)){
        return result.clone();
    }
    let mut result : HashMap<char, u64> = HashMap::new();
    if let Some(c) = polymap.get(&[*l,*r]) {
        result.insert(*c, 1);
        result = merge_hash(count_generated(l, c, t-1, polymap, memo), result);
        result = merge_hash(count_generated(c, r, t-1, polymap, memo), result);
    }
    memo.insert((*l, *r, t), result.clone());
    return result;
}

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

    let polymer_input = lines_i
    .next()
    .expect("can't parse first line")
    .trim()
    .chars()
    .collect::<Vec<char>>();

    let lines_i = lines_i.skip(1);

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

    let mut polymer = polymer_input.clone();

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

    println!("Result part 1: {result}");

    let polymer = polymer_input.clone();

    let el_freq = count_part2(
        &polymer,
        &polymap,
        40,
    );

    let freq_vals = el_freq.values();

    let min_val = freq_vals.clone().min().expect("no elements in polymer");
    let max_val = freq_vals.clone().max().expect("no elements in polymer");

    let result = max_val - min_val;

    println!("Result part 2: {result}");
}
