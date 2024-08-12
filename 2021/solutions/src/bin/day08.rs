use std::fs;

use::std::collections::HashSet;

#[derive(Debug)]
struct Entry {
    patterns: [ [bool;7]; 10],
    output: [ [bool;7]; 4],
}

struct DecodeMap {
    top: usize,
    middle: usize,
    bottom: usize,
    top_l: usize,
    top_r: usize,
    bottom_l: usize,
    bottom_r: usize,
}

fn indexes_true(arr: &[bool; 7]) -> HashSet<usize> {
    let mut out = HashSet::<usize>::new();
    for (i, x) in arr.iter().enumerate() {
        if *x {out.insert(i as usize);}
    }
    return out;
}

fn decode_digit(solution: &DecodeMap, pat: &[bool; 7]) -> u32 {
    let set = indexes_true(pat);
    let two_pattern = HashSet::from([
                                 solution.top,
                                 solution.top_r,
                                 solution.middle,
                                 solution.bottom_l,
                                 solution.bottom,
                                ]);

    let three_pattern = HashSet::from([
                                solution.top,
                                solution.top_r,
                                solution.middle,
                                solution.bottom_r,
                                solution.bottom,
                                ]);

    let five_pattern = HashSet::from([
        solution.top,
        solution.top_l,
        solution.middle,
        solution.bottom_r,
        solution.bottom,
    ]);

    let zero_pattern = HashSet::from([
        solution.top,
        solution.top_l,
        solution.top_r,
        solution.bottom_l,
        solution.bottom_r,
        solution.bottom,
    ]);

    let six_pattern = HashSet::from([
        solution.top,
        solution.top_l,
        solution.middle,
        solution.bottom_l,
        solution.bottom_r,
        solution.bottom,
    ]);

    let nine_pattern = HashSet::from([
        solution.top,
        solution.top_l,
        solution.top_r,
        solution.middle,
        solution.bottom_r,
        solution.bottom,
    ]);

    match set.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        5 if set.is_superset(&two_pattern) => 2,
        5 if set.is_superset(&three_pattern) => 3,
        5 if set.is_superset(&five_pattern) => 5,
        6 if set.is_superset(&zero_pattern) => 0,
        6 if set.is_superset(&six_pattern) => 6,
        6 if set.is_superset(&nine_pattern) => 9,
        _ => {
            eprintln!("Error decoding digit!");
            std::process::exit(1);
        }
    }
}

fn solve_entry(entry: &Entry) -> u32 {
    let mut solution = DecodeMap {
        top: 0usize,
        middle: 0usize,
        bottom: 0usize,
        top_l: 0usize,
        top_r: 0usize,
        bottom_l: 0usize,
        bottom_r: 0usize,
    };

    let mut unknown = HashSet::<usize>::from([0,1,2,3,4,5,6]);
    let mut one = HashSet::<usize>::from([0,1,2,3,4,5,6]);
    let mut four = HashSet::<usize>::from([0,1,2,3,4,5,6]);
    let mut seven = HashSet::<usize>::from([0,1,2,3,4,5,6]);
    let mut f5 = HashSet::<usize>::from([0,1,2,3,4,5,6]);
    let mut f6 = HashSet::<usize>::from([0,1,2,3,4,5,6]);

    for pat in entry.patterns.iter() {
        match patten_count(pat) {
            2 => one = indexes_true(pat),
            3 => seven = indexes_true(pat),
            4 => four = indexes_true(pat),
            5 => f5 = f5.intersection(&indexes_true(pat)).copied().collect(),
            6 => f6 = f6.intersection(&indexes_true(pat)).copied().collect(),
            7 => {},
            _ => {
                eprintln!("Error invalid number of sinals");
                std::process::exit(1);
            }
        }
    }

    solution.top = *seven.difference(&one).next().expect("cannot solve for top");
    unknown.remove(&solution.top);
    solution.middle = *f5.difference(&f6).next().expect("cannot solve for middle");
    unknown.remove(&solution.middle);
    solution.bottom = *f5.intersection(&unknown).next().expect("cannot solve for bottom");
    unknown.remove(&solution.bottom);
    solution.bottom_l = *unknown.difference(&four).next().expect("cannot solve for bottom left");
    unknown.remove(&solution.bottom_l);
    solution.top_l = *unknown.difference(&one).next().expect("cannot solve for top left");
    unknown.remove(&solution.top_l);
    solution.top_r = *unknown.difference(&f6).next().expect("cannot solve for top right");
    unknown.remove(&solution.top_r);
    solution.bottom_r = *unknown.iter().next().expect("cannot solve for bottom right");


    return {
        decode_digit(&solution, &entry.output[0])*1000 +
        decode_digit(&solution, &entry.output[1])*100 +
        decode_digit(&solution, &entry.output[2])*10 +
        decode_digit(&solution, &entry.output[3])*1
    }
}

fn patten_count(pat: &[bool; 7]) -> u32 {
    return pat.iter().fold(0, |acc, x| if *x {acc+1} else {acc});
}

fn decode_text_pattern(text: &str) -> Option<[bool; 7]> {
    let mut out = [false; 7];
    for ch in text.chars() {
        match ch {
            'a' => out[0] = true,
            'b' => out[1] = true,
            'c' => out[2] = true,
            'd' => out[3] = true,
            'e' => out[4] = true,
            'f' => out[5] = true,
            'g' => out[6] = true,
            _ => {return None;},
        }
    }
    Some(out)
}

fn entry_from_line(line: &str) -> Option<Entry>{
    let mut patterns = line
            .trim()
            .split(|x| x == ' ')
            .filter_map(|x| decode_text_pattern(x));

    let mut out = Entry {
        patterns: [ [false;7]; 10],
        output: [ [false;7]; 4],
    };

    out.patterns = patterns
                    .by_ref()
                    .take(10)
                    .collect::<Vec<[bool;7]>>()
                    .try_into().ok()?;

    out.output = patterns
                    .by_ref()
                    .take(4)
                    .collect::<Vec<[bool;7]>>()
                    .try_into().ok()?;

    Some(out)
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day08.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let input_lines = input_file
                            .trim()
                            .split(|x| x == '\n');


    let x = input_lines
    .fold(0, |acc_l, l| {
        acc_l + entry_from_line(l)
        .expect("Cannot parse line")
        .output
        .iter()
        .fold(0, |acc_p, p| {
            match patten_count(p) {
                2 | 4 | 3 | 7 => {acc_p+1},
                _ => acc_p,
            }
        })
    });


    println!("Result part 1: {x}");

    let input_lines = input_file
    .trim()
    .split(|x| x == '\n');

    let x = input_lines
    .fold(0, |acc_l, l| {
        acc_l + solve_entry(&entry_from_line(l).expect("Cannot parse line"))
    });

    println!("Result part 2: {x}");





}
