use std::io::{self, Write};
use std::fs;

fn main(){
    io::stdout()
        .write_all(b"Starting...\n")
        .expect("Error writing to stdout");

    let input_file : String = fs::read_to_string("./input/day02.txt")
                                    .expect("Cannot parse input file into String");

    let mut pos : i32 = 0;
    let mut dep : i32 = 0;
    for line in input_file.lines() {
        let words : Vec<&str> = line.split_whitespace().collect();
        if let [command, arg] = &words[..]{
            match *command {
                "forward" => {
                    pos += (*arg).parse::<i32>().expect(format!("cannot parse arg {arg}").as_str());
                },
                "up" => {
                    dep -= (*arg).parse::<i32>().expect(format!("cannot parse arg {arg}").as_str());
                },
                "down" => {
                    dep += (*arg).parse::<i32>().expect(format!("cannot parse arg {arg}").as_str());

                },
                _ => {
                    io::stdout()
                        .write_all(format!{"Cannot parse command {command}\n"}.as_bytes())
                        .expect("Error writing to stdout");
                }
            }
        }
    }

    io::stdout()
        .write_all(format!{"Result part 1: {}\n", pos * dep}.as_bytes())
        .expect("Error writing to stdout");

    pos = 0;
    dep = 0;
    let mut aim = 0;
    for line in input_file.lines() {
        let words : Vec<&str> = line.split_whitespace().collect();
        if let [command, arg] = &words[..]{
            match *command {
                "forward" => {
                    pos += (*arg).parse::<i32>().expect(format!("cannot parse arg {arg}").as_str());
                    dep += aim*(*arg).parse::<i32>().expect(format!("cannot parse arg {arg}").as_str());
                },
                "up" => {
                    aim -= (*arg).parse::<i32>().expect(format!("cannot parse arg {arg}").as_str());
                },
                "down" => {
                    aim += (*arg).parse::<i32>().expect(format!("cannot parse arg {arg}").as_str());

                },
                _ => {
                    io::stdout()
                    .write_all(format!{"Cannot parse command {command}\n"}.as_bytes())
                    .expect("Error writing to stdout");
                }
            }
        }
    }

    io::stdout()
        .write_all(format!{"Result part 2: {}\n", pos * dep}.as_bytes())
        .expect("Error writing to stdout");
}
