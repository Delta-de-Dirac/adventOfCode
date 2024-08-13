use std::fs;

fn flood(explored: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32{
    let mut count = 1;
    explored[i][j] = true;
    count += if i > 0                   {if explored[i-1][j] == false {flood(explored, i-1, j)} else {0}} else {0};
    count += if i < explored.len()-1    {if explored[i+1][j] == false {flood(explored, i+1, j)} else {0}} else {0};
    count += if j > 0                   {if explored[i][j-1] == false {flood(explored, i, j-1)} else {0}} else {0};
    count += if j < explored[i].len()-1 {if explored[i][j+1] == false {flood(explored, i, j+1)} else {0}} else {0};
    return count;
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day09.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let input_lines = input_file
                            .trim()
                            .split(|x| x == '\n');

    let mut heights : Vec<Vec<u8>> = Vec::new();

    for line in input_lines {
        let mut line_vec : Vec<u8> = Vec::new();
        let line = line.trim();

        for ch in line.chars() {
            line_vec.push(
                ch.to_digit(10).unwrap_or_else(|| {
                    eprintln!("Error parsing character {ch} into digit radix 10");
                    std::process::exit(1);
                }) as u8
            );
        }
        heights.push(line_vec);
    }

    let mut acc : u32 = 0;

    for (i, line) in heights.iter().enumerate() {
        for (j, height) in line.iter().enumerate(){
            let n = if i > 0                  {heights[i-1][j]} else {10};
            let s = if i < heights.len()-1    {heights[i+1][j]} else {10};
            let e = if j > 0                  {heights[i][j-1]} else {10};
            let w = if j < heights[i].len()-1 {heights[i][j+1]} else {10};

            if  *height < n &&
                *height < s &&
                *height < e &&
                *height < w {
                    acc += (*height) as u32 + 1;
                }
        }
    }

    println!("Result part 1: {acc}");

    let mut explored : Vec<Vec<bool>> = heights
    .iter()
    .map(|y|{y.iter().map(|x|{*x >= 9u8}).collect()})
    .collect();

    let mut sizes : Vec<u32> = Vec::new();

    for (i, line) in heights.iter().enumerate() {
        for (j, _) in line.iter().enumerate(){
            if explored[i][j] == false {
                let size = flood(&mut explored, i, j);
                sizes.push(size);
            }
        }
    }

    sizes.sort_by(|a, b| b.cmp(a));

    let result = sizes.iter().take(3).fold(1, |acc, x| acc*x);

    println!("Result part 2: {result}");
}
