use std::io::{self, Write};
use std::fs;

fn main(){
    io::stdout()
        .write_all(b"Starting...\n")
        .expect("Error writing to stdout");

    let input_file : String = fs::read_to_string("./input/day01.txt")
                                    .expect("Cannot parse input file into String");

    let mut depths = input_file
                    .lines()
                    .map(|l| l.parse::<i32>().expect("Cannot parse line"))
                    .into_iter();

    let mut increase_count: i32 = 0;

    if let Some(first) = depths.next(){
        let mut curr = first;
        while let Some(next) = depths.next() {
            if next > curr {increase_count+=1;}
            curr = next;
        }
    }

    io::stdout()
        .write_all(format!("Result part 1: {increase_count}\n").as_bytes())
        .expect("Error writing to stdout");

    increase_count = 0;

    let mut depths = input_file
        .lines()
        .map(|l| l.parse::<i32>().expect("Cannot parse line"))
        .into_iter();

    if let (Some(first), Some(second), Some(third)) = (depths.next(), depths.next(), depths.next()){
        let mut window_back = first;
        let mut queue0 = second;
        let mut queue1 = third;
        while let Some(next) = depths.next() {
            if next > window_back {increase_count+=1;}
            window_back = queue0;
            queue0 = queue1;
            queue1 = next;
        }
    }

    io::stdout()
        .write_all(format!("Result part 2: {increase_count}\n").as_bytes())
        .expect("Error writing to stdout");

}
