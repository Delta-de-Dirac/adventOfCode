use std::fs;
use std::clone::Clone;
use std::fmt;

#[derive(Debug)]
enum SNumElement {
    Literal(i32),
    Pair(Box<Self>, Box<Self>),
}

impl SNumElement {
    fn snum_mag(&self) -> i32 {
        match self {
            SNumElement::Literal(i) => {
                return *i;
            }
            SNumElement::Pair(left, right) => {
                return 3*left.snum_mag() + 2*right.snum_mag();
            }
        }
    }

    fn split_snum(&mut self) -> bool {
        let mut stack = vec![self];
        let mut to_split: Option<&mut Self> = None;

        while let Some(node) = stack.pop() {
            match node {
                SNumElement::Literal(i) if *i >= 10 => {
                    to_split = Some(node);
                    break;
                },
                SNumElement::Pair(left, right) => {
                    stack.push(right);
                    stack.push(left);
                },
                SNumElement::Literal(_) => {
                    continue;
                },
            }
        }

        let splt_happened = to_split.is_some();

        if let Some(to_split) = to_split {
            if let SNumElement::Literal(s_val) = to_split {
                *to_split = SNumElement::Pair(
                    Box::new(
                        SNumElement::Literal(
                            (*s_val as f32 / 2.0).floor() as i32
                        )
                    ),
                    Box::new(
                        SNumElement::Literal(
                            (*s_val as f32 / 2.0).ceil() as i32
                        )
                    )
                );
            }
        }

        return splt_happened;
    }

    fn explode_snum(&mut self) -> bool {
        let mut stack = vec![(self, 0)];

        let mut to_left:    Option<&mut Self> = None;
        let mut to_right:   Option<&mut Self> = None;
        let mut to_explode: Option<&mut Self> = None;

        while let Some((node, d)) = stack.pop() {
            match node {
                SNumElement::Literal(_) => {
                    if to_explode.is_none() {
                        to_left =  Some(node);
                    } else {
                        to_right = Some(node);
                        break;
                    }
                },
                SNumElement::Pair(_, _) if d == 4 && to_explode.is_none() => {
                    to_explode = Some(node);
                },
                SNumElement::Pair(left, right) => {
                    stack.push((right, d+1));
                    stack.push((left,  d+1));
                },
            }
        }

        let exp_happened = to_explode.is_some();

        if let Some(SNumElement::Pair(left, right)) = to_explode {
            if let SNumElement::Literal(exp_val) = **left {
                if let Some(SNumElement::Literal(lval)) = to_left {
                    *lval += exp_val;
                }
            }
            if let SNumElement::Literal(exp_val) = **right {
                if let Some(SNumElement::Literal(rval)) = to_right {
                    *rval += exp_val;
                }
            }
        }
        if let Some(to_explode) = to_explode {
            *to_explode = Self::Literal(0);
        }

        return exp_happened;
    }

    fn reduce_snum(mut self) -> Self {
        loop {
            if self.explode_snum() {continue;}
            if self.split_snum() {continue;}
            break;
        }
        return self;
    }

    fn add_snum(&self, other: &Self) -> Self {
        let result = SNumElement::Pair(Box::new(self.clone()), Box::new(other.clone()));
        let result = result.reduce_snum();
        return result;
    }

    fn parse_snum(input: &[char]) -> (Self, usize) {
        match input.get(0).expect("can't parse empty") {
            x if x.is_ascii_digit() => {
                return (
                    SNumElement::Literal(
                        x
                        .to_digit(10)
                        .expect("is ascii num but cant convert to base 10??")
                        as i32
                    ),
                    1
                );
            },
            '[' => {
                //SNum is pair
                let (l, llen) = SNumElement::parse_snum(input.get(1..).expect("can't parse empty"));
                let mut comma_pos = 1+llen;
                let mut brckt_cnt = 0;
                loop {
                    let search_comma_c = input.get(comma_pos).expect("error looking for comma");
                    match search_comma_c {
                        ',' if brckt_cnt == 0 => {break;},
                        '[' => {brckt_cnt += 1;},
                        ']' => {brckt_cnt -= 1;},
                        _ => {},
                    }
                    comma_pos += 1;
                };
                let (r, rlen) = SNumElement::parse_snum(input.get((comma_pos+1)..).expect("can't parse empty"));
                return (SNumElement::Pair(Box::new(l), Box::new(r)), 3+rlen+llen);
            }
            _ => {panic!("invalid char found");}
        }
    }
}

impl Clone for SNumElement {
    fn clone(&self) -> Self {
        match self {
            SNumElement::Literal(i) => {
                return SNumElement::Literal(*i);
            }
            SNumElement::Pair(left, right) => {
                return SNumElement::Pair(
                    Box::new(*left.clone()),
                    Box::new(*right.clone())
                );
            }
        }
    }
}

impl fmt::Display for SNumElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>
    {
        match self {
            SNumElement::Literal(x) => {
                write!(f, "{x}")?;
            }
            SNumElement::Pair(x, y) => {
                write!(f, "[")?;
                x.fmt(f)?;
                write!(f, ",")?;
                y.fmt(f)?;
                write!(f, "]")?;
            }
        };
        return Ok(());
    }
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day18.txt";

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

    let mut snums = Vec::<SNumElement>::new();

    for line in input_lines{
        let parse_input = line.chars().collect::<Vec<char>>();
        let (my_snum, _) = SNumElement::parse_snum(&parse_input);
        snums.push(my_snum);
    }

    let snums = snums;

    let result = snums.clone()
                      .into_iter()
                      .reduce(|a, b| a.add_snum(&b))
                      .expect("reduce failed");

    let result = result.snum_mag();

    println!("Result part 1: {result}");

    let mut max_found = i32::MIN;
    for i in 0..snums.len() {
        for j in 0..snums.len() {
            if i == j {continue;}
            let result = snums[i].add_snum(&snums[j]);
            let result = result.snum_mag();
            if result > max_found {max_found = result;}
        }
    }
    println!("Result part 2: {max_found}");
}
