use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    width: u8,
    height: u8,
    east_cucumbers:  HashSet<[u8; 2]>,
    south_cucumbers: HashSet<[u8; 2]>,
}


impl Board {
    fn step_board(&mut self) -> bool {
        let mut stepped = false;
        let mut new_east:  HashSet<[u8; 2]> = HashSet::new();
        let mut new_south: HashSet<[u8; 2]> = HashSet::new();
        for c in &self.east_cucumbers {
            let to = [c[0], (c[1]+1)%self.width];
            if self.east_cucumbers.contains(&to) ||
               self.south_cucumbers.contains(&to) {
                   new_east.insert(*c);
            } else {
                new_east.insert(to);
                stepped = true;
            }
        }
        for c in &self.south_cucumbers {
            let to = [(c[0]+1)%self.height, c[1]];
            if new_east.contains(&to) ||
                self.south_cucumbers.contains(&to) {
                    new_south.insert(*c);
                } else {
                    new_south.insert(to);
                    stepped = true;
                }
        }

        self.east_cucumbers = new_east;
        self.south_cucumbers = new_south;

        return stepped;
    }
}

fn main() {
    println!("Starting...");

    let file_name = "./input/day25.txt";

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

    let mut east_cucumbers:  HashSet::<[u8; 2]> = HashSet::new();
    let mut south_cucumbers: HashSet::<[u8; 2]> = HashSet::new();

    for (l, line) in input_lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            match ch {
                '>' => {east_cucumbers.insert( [l as u8,c as u8]);},
                'v' => {south_cucumbers.insert([l as u8,c as u8]);},
                '.' => {continue;},
                _ => {
                    eprintln!("invalid char in input: \"{ch}\"");
                    panic!("invalid char in input");
                }
            }
        }
    }
    let mut board = Board{
        width:  input_lines[0].len() as u8,
        height: input_lines.len() as u8,
        east_cucumbers:  east_cucumbers,
        south_cucumbers: south_cucumbers
    };

    let mut i = 0;

    while board.step_board() {i += 1};

    i += 1;

    println!("Result part 1: {i}")


}
