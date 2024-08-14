use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq)]
struct State {
    cost: usize,
    position: [usize; 2],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn neighbor_list(height: usize, width: usize, pos: [usize; 2]) -> Vec<[usize; 2]> {
    let mut result : Vec<[usize; 2]> = Vec::new();
    if pos[0] > 0 {
        result.push([pos[0]-1, pos[1]]);
    }
    if pos[1] > 0 {
        result.push([pos[0], pos[1]-1]);
    }
    if pos[0] < height-1 {
        result.push([pos[0]+1, pos[1]]);
    }
    if pos[1] < width-1 {
        result.push([pos[0], pos[1]+1]);
    }
    return result;
}

fn shortest_path(cavern: &Vec<Vec<usize>>, start: [usize; 2]) -> Option<usize> {
    let goal = [cavern.len()-1, cavern[0].len()-1];
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; cavern[0].len()]; cavern.len()];
    let mut heap = BinaryHeap::new();
    dist[0][0] = 0;
    heap.push(State { cost: 0, position: start });
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return Some(cost); }
        if cost > dist[position[0]][position[1]] { continue; } /**/
        for neighbor in &neighbor_list(cavern.len(), cavern[0].len(), position) {
            let next = State { cost: cost + cavern[neighbor[0]][neighbor[1]], position: *neighbor };
            if next.cost < dist[next.position[0]][next.position[1]] {
                heap.push(next);
                dist[next.position[0]][next.position[1]] = next.cost;
            }
        }
    }
    None
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day15.txt";

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



    let mut cavern : Vec<Vec<usize>> = Vec::new();

    for line in input_lines.iter() {
        cavern.push(line.chars().map(|x| x.to_digit(10).expect("can't parse") as usize).collect());
    }

    let cavern = cavern;

    let result = shortest_path(&cavern, [0,0]).expect("Cannot find shortest path");

    println!("Result part 1: {result}");

    let lsize = cavern.len();
    let csize = cavern[0].len();


    let mut large_cavern : Vec<Vec<usize>> = vec![vec![0; csize*5]; lsize*5];

    for l in 0..large_cavern.len() {
        for c in 0..large_cavern[0].len() {
            large_cavern[l][c] = (cavern[l%lsize][c%csize] - 1 + l/lsize + c/csize)%9 + 1;
        }
    }

    let large_cavern = large_cavern;

    let result = shortest_path(&large_cavern, [0,0]).expect("Cannot find shortest path");


    println!("Result part 2: {result}");

}
