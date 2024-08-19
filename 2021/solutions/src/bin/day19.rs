use std::fs;
use std::ops;
use std::fmt::{Display, Formatter, Debug};
use std::collections::{HashSet, HashMap};

type Sensor = Vec::<Vec3::<i32>>;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Vec3<T: Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy + ops::Add<Output = T> + ops::Sub<Output = T> + ops::Neg<Output = T>> Vec3<T> {

    fn transform (
        mut self,
        r:  u8,
        ry: u8,
        rz: u8,
        rp: Self,
    ) -> Self {
        for _ in 0..rz {
            self = self.rot_z();
            self = self.rot_z();
        }
        for _ in 0..ry {
            self = self.rot_y();
        }
        for _ in 0..r {
            self = self.rot();
        }
        self = self.sub(rp);
        self
    }

    fn sub(self, other: Self) -> Self {
        Self{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    fn rot(self) -> Self {
        Self{
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }
    fn rot_y(self) -> Self {
        Self{
            x: -self.z,
            y:  self.y,
            z:  self.x,
        }
    }
    fn rot_z(self) -> Self {
        Self{
            x:  self.y,
            y: -self.x,
            z:  self.z,
        }
    }
}

impl<T: Display + Copy + ops::Sub<Output = T>> Debug for Vec3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error>{
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)?;
        Ok(())
    }
}

impl<T: Display + Copy + ops::Sub<Output = T>> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error>{
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)?;
        Ok(())
    }
}

impl Vec3<i32> {
    fn manhattan_distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

fn overlap_test(s1: &Sensor, s2: &Sensor) -> (bool, Vec3<i32>){
    let l1 = s1.len();
    for (i, b1) in s1.iter().enumerate() {
        for (j, b2) in s2.iter().enumerate() {
            let rel_pos = b2.sub(*b1);
            let mut count = 1;
            for (k, b11) in s1.iter().enumerate() {
                if k == i {continue;}
                if l1 - k < 12-count {break;}
                for (l, b22) in s2.iter().enumerate() {
                    if l == j {continue;}
                    if rel_pos == b22.sub(*b11) {
                        count += 1;
                        if count == 12 {return (true, rel_pos)}
                        break;
                    }

                }
            }
        }
    }
    (false, Vec3{x:0, y:0, z:0})
}

fn overlaps(s1: &Sensor, s2: &Sensor) -> (bool, [u8 ;3], Vec3<i32>) {
    let mut s2 = s2.clone();
    for i in 0..2u8{
        for j in 0..4u8 {
            for k in 0..3u8 {
                let (o_test, rp) = overlap_test(s1, &s2);
                if o_test {return (true, [i, j, k], rp);}
                s2 = s2.iter().map(|b| b.rot()).collect();
            }
            s2 = s2.iter().map(|b| b.rot_y()).collect();
        }
        s2 = s2.iter().map(|b| b.rot_z()).collect();
        s2 = s2.iter().map(|b| b.rot_z()).collect();
    }
    (false, [0, 0, 0], Vec3{x:0, y:0, z:0})
}



fn restore_map(
    over_map: &HashMap::<(usize, usize), ([u8 ;3], Vec3<i32>)>,
    sens: &Vec<Sensor>,
    unvisited: &mut Vec<bool>,
    current: usize,
) -> HashSet::<Vec3<i32>> {
    unvisited[current] = false;
    let mut result = HashSet::<Vec3<i32>>::new();
    for beacon in sens[current].iter() {
        result.insert(*beacon);
    }
    for key in over_map.keys().filter(|x| x.0 == current){
        if unvisited[key.1]{
            let ([rz, ry, r], rp) = over_map.get(key).expect("invalid key");
            let mut other_map = restore_map(
                over_map,
                sens,
                unvisited,
                key.1,
            );
            other_map = other_map.iter().map(|x| x.transform(*r, *ry, *rz, *rp)).collect();
            for beacon in other_map.iter() {
                result.insert(*beacon);
            }
        }
    }
    return result;
}

fn compute_positions(
    unvisited: &mut Vec<bool>,
    over_map: &HashMap::<(usize, usize), ([u8 ;3], Vec3<i32>)>,
    current: usize,
) -> HashSet::<Vec3<i32>>
{
    unvisited[current] = false;
    let mut result = HashSet::<Vec3<i32>>::new();
    result.insert(Vec3{x:0, y:0, z:0});

    for key in over_map.keys().filter(|x| x.0 == current){
        if unvisited[key.1]{
            let ([rz, ry, r], rp) = over_map.get(key).expect("invalid key");
            let mut other_map = compute_positions(
                unvisited,
                over_map,
                key.1,
            );
            other_map = other_map.iter().map(|x| x.transform(*r, *ry, *rz, *rp)).collect();
            for x in other_map.iter() {
                result.insert(*x);
            }
        }
    }
    return result;
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day19.txt";

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

    let mut sensors : Vec<Sensor> = Vec::new();
    let mut unvisited : Vec<bool> = Vec::new();

    for sensor_block in input_lines.split(|x| x.is_empty()) {
        let mut sensor : Sensor = Vec::new();
        for line in sensor_block[1..].iter() {
            let line : Vec<&str> = line.split(|x| x == ',').collect();
            let x : i32 = line.get(0).expect("Invalid input file").parse::<i32>().expect("Invalid input file");
            let y : i32 = line.get(1).expect("Invalid input file").parse::<i32>().expect("Invalid input file");
            let z : i32 = line.get(2).expect("Invalid input file").parse::<i32>().expect("Invalid input file");
            sensor.push(
                Vec3 {
                    x: x,
                    y: y,
                    z: z,
                }
            );
        }
        sensors.push(sensor);
        unvisited.push(true);
    }

    let mut over_map = HashMap::<(usize, usize), ([u8 ;3], Vec3<i32>)>::new();

    for (i, s1) in sensors.iter().enumerate(){
        for (j, s2) in sensors.iter().enumerate(){
            if i == j {continue;}
            let (overlap, [rz, ry, r], rp) = overlaps(&s1, &s2);
            if overlap {
                over_map.insert((i, j), ([rz, ry, r], rp));
            }
        }
    }


    let full_map = restore_map(
        &over_map,
        &sensors,
        &mut unvisited,
        0,
    );

    println!("Result part 1: {result}", result = full_map.len());

    for i in 0..unvisited.len() {
        unvisited[i] = true;
    }

    let positions : Vec<Vec3<i32>> = compute_positions(
        &mut unvisited,
        &over_map,
        0,
    ).into_iter().collect();

    let mut largest_manhattan = i32::MIN;

    for (i, p1) in positions.iter().enumerate(){
        for (_, p2) in positions[(i+1)..].iter().enumerate(){
            let dist = p1.manhattan_distance(*p2);
            if dist > largest_manhattan {
                largest_manhattan = dist;
            }
        }
    }

    println!("Result part 2: {largest_manhattan}");
}
