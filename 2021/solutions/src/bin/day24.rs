use std::fs;
use std::fmt;

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

#[derive(Clone, Debug)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl ALU {
    fn get_reg(&self, r: Register) -> i64 {
        match r {
            Register::W => {
                self.w
            },
            Register::X => {
                self.x
            },
            Register::Y => {
                self.y
            },
            Register::Z => {
                self.z
            },
        }
    }

    fn set_reg(&mut self, r: Register, v: i64) {
        match r {
            Register::W => {
                self.w = v;
            },
            Register::X => {
                self.x = v;
            },
            Register::Y => {
                self.y = v;
            },
            Register::Z => {
                self.z = v;
            },
        }
    }

    fn exec(&self, p: Program, mut input: &[char]) -> Self {
        let mut res = self.clone();
        for inst in p {
            match inst {
                Instruction::INP(reg) => {
                    if input.is_empty() {panic!("empty input slice")}
                    if let Some(inp) = input[0].to_digit(10){
                        res.set_reg(reg, inp as i64);
                        input = &input[1..];
                    } else {
                        panic!("cannot parse input");
                    }
                },
                Instruction::ADD(reg1, ro) => {
                    match ro {
                        RightOperand::Literal(lit) => {
                            res.set_reg(reg1, lit+res.get_reg(reg1));
                        },
                        RightOperand::Reg(reg2) => {
                            res.set_reg(reg1, res.get_reg(reg1)+res.get_reg(reg2));
                        },
                    }
                },
                Instruction::MUL(reg1, ro) => {
                    match ro {
                        RightOperand::Literal(lit) => {
                            res.set_reg(reg1, lit*res.get_reg(reg1));
                        },
                        RightOperand::Reg(reg2) => {
                            res.set_reg(reg1, res.get_reg(reg1)*res.get_reg(reg2));
                        },
                    }
                },
                Instruction::DIV(reg1, ro) => {
                    match ro {
                        RightOperand::Literal(lit) => {
                            if lit == 0i64 {
                                panic!("Cannot DIV by 0");
                            }
                            res.set_reg(reg1, res.get_reg(reg1)/lit);
                        },
                        RightOperand::Reg(reg2) => {
                            if res.get_reg(reg2) == 0i64 {
                                panic!("Cannot DIV by 0");
                            }
                            res.set_reg(reg1, res.get_reg(reg1)/res.get_reg(reg2));
                        },
                    }
                },
                Instruction::MOD(reg1, ro) => {
                    if res.get_reg(reg1) < 0i64 {
                        panic!("Cannot MOD A B, A cannot be neg");
                    }
                    match ro {
                        RightOperand::Literal(lit) => {
                            if lit <= 0i64 {
                                panic!("Cannot MOD A B, B cannot be NEG or ZERO");
                            }
                            res.set_reg(reg1, res.get_reg(reg1)%lit);
                        },
                        RightOperand::Reg(reg2) => {
                            if res.get_reg(reg2) <= 0i64 {
                                panic!("Cannot MOD A B, B cannot be NEG or ZERO");
                            }
                            res.set_reg(reg1, res.get_reg(reg1)%res.get_reg(reg2));
                        },
                    }
                },
                Instruction::EQL(reg1, ro) => {
                    match ro {
                        RightOperand::Literal(lit) => {
                            if res.get_reg(reg1)==lit {
                                res.set_reg(reg1, 1);
                            } else {
                                res.set_reg(reg1, 0);
                            }
                        },
                        RightOperand::Reg(reg2) => {
                            if res.get_reg(reg1)==res.get_reg(reg2) {
                                res.set_reg(reg1, 1);
                            } else {
                                res.set_reg(reg1, 0);
                            }
                        },
                    }
                },
            }
        }
        return res;
    }
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

    let start_alu = ALU{
        w: 0,
        x: 0,
        y: 0,
        z: 0,
    };

    let inp_u64 = 99999999999999u64;

    let input = inp_u64.to_string().chars().collect::<Vec<char>>();

    for line in input_lines {
        let words = line.split(|x| x == ' ').collect::<Vec<&str>>();
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

    let input = 39999999999999u64.to_string().chars().collect::<Vec<char>>();
    let alu = start_alu.exec(program.clone(), &input);

    println!("{z}", z = alu.z);



}
