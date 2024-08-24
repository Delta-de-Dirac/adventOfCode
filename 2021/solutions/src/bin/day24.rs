use std::fs;
use std::fmt;

enum Register {
    X,
    Y,
    Z,
    W,
}

enum RightOperand {
    Literal(u32),
    Register,
}

enum Instruction {
    INP(Register),
    ADD(Register, RightOperand),
    MUL(Register, RightOperand),
    DIV(Register, RightOperand),
    MOD(Register, RightOperand),
    EQL(Register, RightOperand),
}

type Program = Vec<Instruction>;

Struct ALU {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
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

    println!("{input_lines:?}");

}
