use std::fs;
use std::collections::HashSet;
use std::fmt::Display;
use std::fmt::Formatter;

struct Board {
    algo: Vec<char>,
    light_pix: HashSet<[i32; 2]>,
    dark_pix: HashSet<[i32; 2]>,
    outside: char,
}

impl Display for Board{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let algo : String = self.algo.iter().collect();
        let outside = self.outside;

        let mut max_l = i32::MIN;
        let mut min_l = i32::MAX;
        let mut max_c = i32::MIN;
        let mut min_c = i32::MAX;

        for pos in self.light_pix.iter() {
            if pos[0] > max_l {max_l = pos[0];}
            if pos[0] < min_l {min_l = pos[0];}
            if pos[1] > max_c {max_c = pos[1];}
            if pos[1] < min_c {min_c = pos[1];}
        }

        for pos in self.dark_pix.iter() {
            if pos[0] > max_l {max_l = pos[0];}
            if pos[0] < min_l {min_l = pos[0];}
            if pos[1] > max_c {max_c = pos[1];}
            if pos[1] < min_c {min_c = pos[1];}
        }

        write!(f, "algo: {algo}")?;
        write!(f, "\n")?;
        write!(f, "outside: {outside}")?;
        write!(f, "\n")?;
        write!(f, "board:")?;
        write!(f, "\n")?;
        for i in min_l..=max_l {
            for j in min_c..=max_c {
                if self.light_pix.contains(&[i, j]){
                    write!(f, "#")?;
                    continue;
                }
                if self.dark_pix.contains(&[i, j]){
                    write!(f, ".")?;
                    continue;
                }
                else {
                    write!(f, "?")?;
                    continue;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    fn get_algo_result(&self, i: i32, j: i32) -> char {
        let input = self.get_algo_input(i, j);
        return self.algo[input];
    }

    fn get_algo_input(&self, i: i32, j: i32) -> usize{
        let mut piece = 256;
        let mut result = 0;

        for ii in i-1..=i+1{
            for jj in j-1..=j+1{
                if self.light_pix.contains(&[ii,jj]) {
                    result += piece;
                } else if self.dark_pix.contains(&[ii,jj]){
                    result += 0;
                } else {
                    if self.outside == '#' {
                        result += piece;
                    }
                }
                piece /= 2;
            }
        }
        return result;
    }

    fn tick(&self) -> Self {
        let new_outside = match (self.outside, self.algo.get(0), self.algo.get(511)) {
            ('.', Some('#'), _        ) => {'#'},
            ('#', _,         Some('.')) => {'.'},
            _ => {self.outside},
        };
        let mut result = Board{
            algo: self.algo.clone(),
            light_pix: HashSet::new(),
            dark_pix: HashSet::new(),
            outside: new_outside,
        };
        let mut visited : HashSet<[i32; 2]> = HashSet::new();

        for pos in self.light_pix.iter() {
            for i in pos[0]-1..=pos[0]+1{
                for j in pos[1]-1..=pos[1]+1{
                    if visited.contains(&[i, j]){continue;}
                    visited.insert([i,j]);
                    match self.get_algo_result(i, j) {
                        '#' => {result.light_pix.insert([i,j]);},
                        '.' => {result.dark_pix.insert([i,j]);},
                        _   => {panic!("impossible");},
                    }
                }
            }
        }

        for pos in self.dark_pix.iter() {
            for i in pos[0]-1..=pos[0]+1{
                for j in pos[1]-1..=pos[1]+1{
                    if visited.contains(&[i, j]){continue;}
                    visited.insert([i,j]);
                    match self.get_algo_result(i, j) {
                        '#' => {result.light_pix.insert([i,j]);},
                        '.' => {result.dark_pix.insert([i,j]);},
                        _   => {panic!("impossible");},
                    }
                }
            }
        }

        return result;
    }
}



fn main(){
    println!("Starting...");

    let file_name = "./input/day20.txt";

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

    let algo : Vec<char> = input_lines.get(0).expect("input error").chars().collect();

    let mut board : Board = Board{
        algo: algo,
        light_pix: HashSet::new(),
        dark_pix: HashSet::new(),
        outside: '.',
    };

    for (i, line) in input_lines[2..].iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            match ch{
                '#' => { board.light_pix.insert([i as i32,j as i32]);},
                '.' => { board.dark_pix.insert([i as i32,j as i32]);},
                _ => {},
            }
        }
    }

    board = board.tick();
    board = board.tick();
    //println!("{board}");
    println!("Result part 1: {}", board.light_pix.len());

    for _ in 0..48 {
        board = board.tick();
    }
    //println!("{board}");
    println!("Result part 2: {}", board.light_pix.len());

}
