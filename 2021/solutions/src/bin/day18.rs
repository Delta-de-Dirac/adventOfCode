use std::fs;

#[derive(Debug)]
enum SNumElement {
    Literal(i32),
    Pair(Box<SNumElement>, Box<SNumElement>),
}

fn explode_snum(root: &mut SNumElement) {
    let mut stack = vec![(root, 0)];

    let mut to_left: Option<&mut SNumElement> = None;


    while let Some((node, d)) = stack.pop() {
        match node {
            SNumElement::Literal(i) => {
                println!("{i} {d}");
                to_left = Some(node);
            },
            SNumElement::Pair(left, right) if d == 4 => {
                if let (SNumElement::Literal(lval), SNumElement::Literal(rval)) = (**right,**left){
                    println!("{lval}, {rval}");
                }
            },
            SNumElement::Pair(left, right) => {
                stack.push((right, d+1));
                stack.push((left, d+1));
            },
        }
    }
    if let Some(SNumElement::Literal(val)) = to_left {
        *val = 0;
    }
}

fn reduce_snum(x: SNumElement) -> SNumElement {
    todo!();
}

fn add_snum(x: SNumElement, y: SNumElement) -> SNumElement {
    let result = SNumElement::Pair(Box::new(x), Box::new(y));
    let result = reduce_snum(result);
    return result;
}

fn print_snum(snum: &SNumElement) {
    match snum {
        SNumElement::Literal(x) => {print!("{x}");}
        SNumElement::Pair(x, y) => {
            print!("[");
            print_snum(x);
            print!(",");
            print_snum(y);
            print!("]");
        }
    };
}

fn parse_snum(input: &[char]) -> (SNumElement, usize) {
    match input.get(0).expect("can't parse empty") {
        x if x.is_ascii_digit() => {
            return (
                SNumElement::Literal(
                    x.to_digit(10)
                    .expect("is ascii num but cant convert to base 10??")
                    as i32
                ),
                1
            );
        },
        '[' => {
            //SNum is pair
            let (l, llen) = parse_snum(input.get(1..).expect("can't parse empty"));
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
            let (r, rlen) = parse_snum(input.get((comma_pos+1)..).expect("can't parse empty"));
            return (SNumElement::Pair(Box::new(l), Box::new(r)), 3+rlen+llen);
        }
        _ => {panic!("invalid char found");}
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
        let (my_snum, _) = parse_snum(&parse_input);
        snums.push(my_snum);
    }
    let root = &mut snums.pop().expect("input invalid");

    explode_snum(root);

    print_snum(root);


    let result = 0;

    println!("Result part 1: {result}");


}
