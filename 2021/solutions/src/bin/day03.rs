use std::iter;
use std::fs;
use std::error;

fn oxy_filter(lines: Vec<&[u8]>, idx: usize) -> Result<&[u8], Box<dyn error::Error>> {
    match (&lines).as_slice() {
        &[x] => {return Ok(x);}
        &[_, ..] => {
            let mut acc : i32 = 0;
            for line in &lines {
                match line.get(idx) {
                    Some(b'1') => {acc += 1}
                    Some(b'0') => {acc -= 1}
                    _ => {return Err(Box::from("Invalid digit at index"));}
                }
            }
            let most_common = if acc >= 0 {b'1'} else {b'0'};
            let lines : Vec<&[u8]> = lines.into_iter()
                .filter(|x| x.get(idx) == Some(&most_common))
                .collect();
            return oxy_filter(lines, idx+1);
        }
        _ => {return Err(Box::from("Could not find oxy rating"));}
    }
}

fn oxy_rating(lines: &Vec<&[u8]>) -> Result<usize, Box<dyn error::Error>>{
    let lines = (*lines).clone();
    let result = oxy_filter(lines, 0)?;
    let result = std::str::from_utf8(result)?;
    let result = usize::from_str_radix(&(result), 2)?;
    return Ok(result);
}

fn co2_filter(lines: Vec<&[u8]>, idx: usize) -> Result<&[u8], Box<dyn error::Error>>{
    match (&lines).as_slice() {
        &[x] => {return Ok(x);}
        &[_, ..] => {
            let mut acc : i32 = 0;
            for line in &lines {
                match line.get(idx) {
                    Some(b'1') => {acc += 1}
                    Some(b'0') => {acc -= 1}
                    _ => {return Err(Box::from("Invalid digit at index"));}
                }
            }
            let least_common = if acc >= 0 {b'0'} else {b'1'};
            let lines : Vec<&[u8]> = lines.into_iter()
            .filter(|x| x.get(idx) == Some(&least_common))
            .collect();
            return co2_filter(lines, idx+1);
        }
        _ => {return Err(Box::from("Could not find co2 rating"));}
    }
}

fn co2_rating(lines : &Vec<&[u8]>) -> Result<usize, Box<dyn error::Error>>{
    let lines = (*lines).clone();
    let result = co2_filter(lines, 0)?;
    let result = std::str::from_utf8(result)?;
    let result = usize::from_str_radix(&(result), 2)?;
    return Ok(result);
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day03.txt";

    let input_file : Vec<u8> = fs::read(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let mut input_lines = input_file
        .trim_ascii()
        .split(|x| *x == b'\n');

    let mut acc_column = Vec::<i32>::new();

    let first_line = input_lines.next().unwrap_or_else(|| {
        eprintln!("No line found at input");
        std::process::exit(1);
    });

    for entry in first_line {
        match entry{
            b'1' => {
                acc_column.push(1);
            }
            b'0' => {
                acc_column.push(-1);
            }
            _ => {
                eprintln!("Unexpected digit: {entry}");
                std::process::exit(1);
            }
        }
    }

    for line in input_lines {
        for (acc, entry) in iter::zip(&mut acc_column, line) {
            match entry {
                b'1' =>{*acc += 1;}
                b'0' =>{*acc -= 1;}
                _ => {
                    eprintln!("Unexpected digit: {entry}");
                    std::process::exit(1);
                }
            }
        }
    }
    let gam = (&acc_column).into_iter()
                        .map(|x| if *x >= 0 { '1' } else { '0' } )
                        .collect::<String>();

    let gam = usize::from_str_radix(&gam, 2)
        .unwrap_or_else(|err| {
            println!("Cannot parse gam into : {gam}");
            println!("{err}");
            std::process::exit(1);
        });

    let eps = (&acc_column).into_iter()
        .map(|x| if *x >= 0 { '0' } else { '1' } )
        .collect::<String>();

    let eps = usize::from_str_radix(&eps, 2)
        .unwrap_or_else(|err| {
            println!("Cannot parse eps into binary: {eps}");
            println!("{err}");
            std::process::exit(1);
        });

    println!("Result part 1: {result}", result = gam * eps);

    let input_lines = input_file
        .trim_ascii()
        .split(|x| *x == b'\n')
        .collect::<Vec<&[u8]>>();

    let oxy = oxy_rating(&input_lines).unwrap_or_else(|err| {
        println!("{err}");
        std::process::exit(1);
    });

    let co2 = co2_rating(&input_lines).unwrap_or_else(|err| {
        println!("{err}");
        std::process::exit(1);
    });

    println!("Result part 2: {result}", result = oxy * co2);
}
