use std::fs;


fn flash_octos(octos: &mut [[u8; 10]; 10], flashed: &mut [[bool; 10]; 10]) -> u32 {
    let mut flashes = 0;
    let mut live = false;
    for l in 0..10 {
        for c in 0..10 {
            let o = &mut octos[l][c];
            if *o > 9 {
                *o = 0;
                flashed[l][c] = true;
                live = true;
                flashes += 1;
                for i in -1..=1 {
                    for j in -1..=1 {
                        match ((l as i32)+i, (c as i32)+j) {
                            (x, y) if x == l as i32 && y == c as i32 => {continue},
                            (x, y) if x <  0  || y <  0 => {continue},
                            (x, y) if x >= 10 || y >= 10 => {continue},
                            (x, y) if flashed[x as usize][y as usize] => {continue},
                            (x, y) => {
                                octos[x as usize][y as usize] += 1;
                            },
                        }
                    }
                }
            }
        }
    }
    if live {
        flashes += flash_octos(octos, flashed);
    }
    return flashes;
}

fn increment_octos(octos: &mut [[u8; 10]; 10]) {
    for line in octos {
        for o in line {
            *o += 1;
        }
    }
}

fn step(octos: &mut [[u8; 10]; 10]) -> u32{
    let mut flashes = 0;
    let mut flashed = [[false ; 10]; 10];

    increment_octos(octos);
    flashes += flash_octos(octos, &mut flashed);

    return flashes;
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day11.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let input_lines = input_file
                            .trim()
                            .split(|x| x == '\n');

    let mut octo = [[0u8; 10]; 10];

    for (l, line) in input_lines.clone().enumerate() {
        octo[l] = line
                    .chars()
                    .map(|x| x.to_digit(10).expect("invalid digit at input") as u8)
                    .collect::<Vec<u8>>()
                    .try_into()
                    .expect("line does not fit into octo array");
    }

    let mut result = 0;

    for _i in 0..100 {
        result += step(&mut octo);
    }

    println!("Result part 1: {result}");

    let mut octo = [[0u8; 10]; 10];

    for (l, line) in input_lines.enumerate() {
        octo[l] = line
        .chars()
        .map(|x| x.to_digit(10).expect("invalid digit at input") as u8)
        .collect::<Vec<u8>>()
        .try_into()
        .expect("line does not fit into octo array");
    }

    let mut result = 0;

    loop {
        let delta = step(&mut octo);
        result += 1;
        if delta  == 100 {break;}
    }

    println!("Result part 1: {result}");
}
