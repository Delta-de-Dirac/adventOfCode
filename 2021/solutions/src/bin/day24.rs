use std::fs;

#[derive(Clone, Copy, Debug)]
enum Register {
    X,
    Y,
    Z,
    W,
}

impl Register {
    fn parse_from(s: &str) -> Option<Self> {
        match s {
            "x" => {Some(Register::X)},
            "y" => {Some(Register::Y)},
            "z" => {Some(Register::Z)},
            "w" => {Some(Register::W)},
            _   => {None},
        }
    }
}

#[derive(Debug, Clone)]
enum RightOperand {
    Literal(i64),
    Reg(Register),
}

impl RightOperand {
    fn parse_from(s: &str) -> Option<Self> {
        if let Ok(lit) = s.parse::<i64>() {
            Some(RightOperand::Literal(lit))
        } else {
            if let Some(reg) = Register::parse_from(s) {
                Some(RightOperand::Reg(reg))
            }
            else {
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    INP(Register),
    ADD(Register, RightOperand),
    MUL(Register, RightOperand),
    DIV(Register, RightOperand),
    MOD(Register, RightOperand),
    EQL(Register, RightOperand),
}

type Program = Vec<Instruction>;

fn program_subprocs(prog: &Program) -> Vec<SubProc> {
    let mut result : Vec<SubProc> = Vec::new();
    for (i, inst) in prog.iter().enumerate() {
        let mut new_subproc = SubProc{p1: 0, p2: 0, p3: 0};
        if let Instruction::INP(Register::W) = inst {
            if let Instruction::ADD(_, RightOperand::Literal(lit)) = prog[i+5]{
                new_subproc.p1 = lit;
            } else {
                panic!("Invalid subproc!");
            }
            if let Instruction::ADD(_, RightOperand::Literal(lit)) = prog[i+15]{
                new_subproc.p2 = lit;
            } else {
                panic!("Invalid subproc!");
            }
            if let Instruction::DIV(_, RightOperand::Literal(lit)) = prog[i+4]{
                new_subproc.p3 = lit;
            } else {
                panic!("Invalid subproc!");
            }
            result.push(new_subproc);
        } else {
            continue;
        }
    }
    return result;
}

#[derive(Debug)]
struct SubProc {
    p1: i64,
    p2: i64,
    p3: i64,
}

impl SubProc {
    fn inv_run(&self, output: i64) -> Vec<[i64; 2]> {
        let mut result : Vec<[i64; 2]> = Vec::new();
        for i in 0..self.p3 {
            for w in 1..=9 {
                if ((output*self.p3+i) % 26)+self.p1 == w{
                    result.push([output*self.p3+i, w]);
                }
            }
        }
        for w in 1..=9{
            let k = output-w-self.p2;
            if k % 26 != 0{
                continue;
            }
            let k = k / 26;
            for i in 0..self.p3{
                let z = k*self.p3+i;
                if ((z % 26) + self.p1) != w{
                    result.push([z, w]);
                }
            }
        }
        return result
    }

}

fn solve(sub_procs: &Vec<SubProc>, depth: usize, expected_output: i64) -> Vec<u64> {
    let mut result : Vec<u64> = Vec::new();
    let current = sub_procs.get(depth-1).expect("invalid depth");
    if depth == 1 {
        let next = current.inv_run(expected_output);
        for n in next{
            if n[0] == 0 {
                result.push((n[1] as u64)*10u64.pow(13))
            }
        }
    } else {
        let next = current.inv_run(expected_output);
        for n in next{
            for n_res in solve(sub_procs, depth-1, n[0]) {
                result.push(n_res + (n[1] as u64)*10u64.pow( (14-depth) as u32));
            }
        }
    }
    return result;
}

fn main() {
    println!("Starting...");

    let file_name = "./input/day24.txt";

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

    let mut program : Program = Vec::new();

    for line in input_lines {
        let words = line.trim().split(|x| x == ' ').collect::<Vec<&str>>();
        let op = words[0];
        match op {
            "inp" => {
                if let Some(reg) = Register::parse_from(words[1]) {
                    program.push(Instruction::INP(reg));
                } else {
                    panic!("invalid input");
                }
            },
            "add" => {
                if let (Some(reg), Some(ro)) = (Register::parse_from(words[1]), RightOperand::parse_from(words[2])) {
                    program.push(Instruction::ADD(reg, ro));
                } else {
                    panic!("invalid input");
                }
            },
            "mul" => {
                if let (Some(reg), Some(ro)) = (Register::parse_from(words[1]), RightOperand::parse_from(words[2])) {
                    program.push(Instruction::MUL(reg, ro));
                } else {
                    panic!("invalid input");
                }
            },
            "div" => {
                if let (Some(reg), Some(ro)) = (Register::parse_from(words[1]), RightOperand::parse_from(words[2])) {
                    program.push(Instruction::DIV(reg, ro));
                } else {
                    panic!("invalid input");
                }
            },
            "mod" => {
                if let (Some(reg), Some(ro)) = (Register::parse_from(words[1]), RightOperand::parse_from(words[2])) {
                    program.push(Instruction::MOD(reg, ro));
                } else {
                    panic!("invalid input");
                }
            },
            "eql" => {
                if let (Some(reg), Some(ro)) = (Register::parse_from(words[1]), RightOperand::parse_from(words[2])) {
                    program.push(Instruction::EQL(reg, ro));
                } else {
                    panic!("invalid input");
                }
            },
            _ =>{
                panic!("invalid input");
            },
        };
    }

    let sub_procs = program_subprocs(&program);

    let res = solve(&sub_procs, 14, 0);

    println!("Result part 1: {result}", result = res.iter().max().expect("no result possible"));
    println!("Result part 2: {result}", result = res.iter().min().expect("no result possible"));

}
