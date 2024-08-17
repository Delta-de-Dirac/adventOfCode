use std::fs;
use std::ops;
use std::fmt::{Display, Formatter, Debug};

type Sensor = Vec::<Vec3::<i32>>;


#[derive(Copy, Clone, PartialEq)]
struct Vec3<T: Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy + ops::Sub<Output = T> + ops::Neg<Output = T>> Vec3<T> {
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

fn overlap_test(s1: &Sensor, s2: &Sensor) -> bool {
    for (i, b1) in s1.iter().enumerate() {
        for (j, b2) in s2.iter().enumerate() {
            let rel_pos = b2.sub(*b1);
            let mut count = 1;
            for (k, b11) in s1.iter().enumerate() {
                if k == i {continue;}
                for (l, b22) in s2.iter().enumerate() {
                    if l == j {continue;}
                    if rel_pos == b22.sub(*b11) {count += 1}
                    if count == 12 {return true}
                }
            }
        }
    }
    false
}

fn overlaps(s1: &Sensor, s2: &Sensor) -> bool {
    let mut s2 = s2.clone();

    s2 = s2.iter().map(|b| b.rot()).collect();

    for _ in 0..2{
        for _ in 0..4 {
            for _ in 0..3 {
                if overlap_test(s1, &s2) {return true;}
                s2 = s2.iter().map(|b| b.rot()).collect();
            }
            s2 = s2.iter().map(|b| b.rot_y()).collect();
        }
        s2 = s2.iter().map(|b| b.rot_z()).collect();
        s2 = s2.iter().map(|b| b.rot_z()).collect();
    }
    false
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
            )
        }
        sensors.push(sensor);
    }

    for (i, s1) in sensors.iter().enumerate(){
        for (j, s2) in sensors[(i+1)..].iter().enumerate(){
            let j = j+i+1;
            if overlaps(&s1, &s2) {
                println!("sensor {i} overlaps sensor {j}");
            }
        }
    }
}
