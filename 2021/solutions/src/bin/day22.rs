use std::fs;

type Interval = [i32; 2];

type Rect3D = [Interval; 3];

fn decompose_r(a: &Rect3D, b: &Rect3D) -> (Vec::<Rect3D>, Vec::<Rect3D>, Option<Rect3D>) {
    if !overlap(a, b) {
        return (vec![*a],vec![*b],None);
    }
    else {
        let mut res_a = Vec::<Rect3D>::new();
        let mut res_b = Vec::<Rect3D>::new();
        //Solve X
        let (new_a_i, new_b_i, over_i_x) = decompose_i(&a[0], &b[0]);

        let over_i_x = over_i_x.expect("There should be an overlap");

        for interval in new_a_i {
            res_a.push([
                interval,
                a[1],
                a[2],
            ]);
        }
        for interval in new_b_i {
            res_b.push([
                interval,
                b[1],
                b[2],
            ]);
        }

        //Solve Y
        let (new_a_i, new_b_i, over_i_y) = decompose_i(&a[1], &b[1]);

        let over_i_y = over_i_y.expect("There should be an overlap");

        for interval in new_a_i {
            res_a.push([
                over_i_x,
                interval,
                a[2],
            ]);
        }
        for interval in new_b_i {
            res_b.push([
                over_i_x,
                interval,
                b[2],
            ]);
        }

        //Solve Z
        let (new_a_i, new_b_i, over_i_z) = decompose_i(&a[2], &b[2]);

        let over_i_z = over_i_z.expect("There should be an overlap");

        for interval in new_a_i {
            res_a.push([
                over_i_x,
                over_i_y,
                interval,
            ]);
        }
        for interval in new_b_i {
            res_b.push([
                over_i_x,
                over_i_y,
                interval,
            ]);
        }
        return (res_a, res_b, Some([over_i_x, over_i_y, over_i_z]));
    }
}

fn decompose_i(a: &Interval, b: &Interval) -> (Vec::<Interval>, Vec::<Interval>, Option<Interval>) {
    if a[1] < b[0] {
        return (vec![*a],vec![*b],None);
    } else if b[1] < a[0] {
        return (vec![*a],vec![*b],None);
    } else {
        let mut new_a = Vec::<Interval>::new();
        let mut new_b = Vec::<Interval>::new();
        let mut over : Interval = [0,0];
        if a[0] == b[0] {
            over[0] = a[0];
            if a[1] == b[1] { //       start and end the same
                over[1] = a[1];
                return (new_a, new_b, Some(over));
            } else if a[1] < b[1] { // start same and b tail to right
                over[1] = a[1];
                new_b.push([a[1]+1, b[1]]);
                return (new_a, new_b, Some(over));
            } else { // b[1] < a[1]    start same and a tail to right
                over[1] = b[1];
                new_a.push([b[1]+1, a[1]]);
                return (new_a, new_b, Some(over));
            }
        } else if a[0] < b[0] { //      a tail to left
            over[0] = b[0];
            if a[1] == b[1] { //        a tail to left and end same
                over[1] = a[1];
                new_a.push([a[0], b[0]-1]);
                return (new_a, new_b, Some(over));
            } else if a[1] < b[1] {//   a tail to left and b tail to right
                over[1] = a[1];
                new_a.push([a[0],   b[0]-1]);
                new_b.push([a[1]+1, b[1]  ]);
                return (new_a, new_b, Some(over));
            } else { // b[1] < a[1]     a tail to left and a tail to right (b inside a)
                over[1] = b[1];
                new_a.push([a[0],   b[0]-1]);
                new_a.push([b[1]+1, a[1]  ]);
                return (new_a, new_b, Some(over));
            }
        } else { // b[0] < a[0]         b tail to left
            over[0] = a[0];
            if a[1] == b[1] {//         b tail to left and end same
                over[1] = a[1];
                new_b.push([b[0], a[0]-1]);
                return (new_a, new_b, Some(over));
            } else if a[1] < b[1] {//   b tail to left and b tail to right (a inside b)
                over[1] = a[1];
                new_b.push([b[0],   a[0]-1]);
                new_b.push([a[1]+1, b[1]  ]);
                return (new_a, new_b, Some(over));
            } else { // b[1] < a[1]     b tail to left and a tail to right
                over[1] = b[1];
                new_b.push([b[0],   a[0]-1]);
                new_a.push([b[1]+1, a[1]  ]);
                return (new_a, new_b, Some(over));
            }
        }
    }
}

#[test]
fn test_decompose_i() {
    // dont overlap case 1
    let a : Interval = [-10, 10];
    let b : Interval = [ 20, 30];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 1        );
    assert!(new_a[0]    == [-10, 10]);
    assert!(new_b.len() == 1        );
    assert!(new_b[0]    == [ 20, 30]);
    assert!(over        == None);


    // dont overlap case 2
    let a : Interval = [-10,  10];
    let b : Interval = [-30, -20];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 1         );
    assert!(new_a[0]    == [-10, 10 ]);
    assert!(new_b.len() == 1         );
    assert!(new_b[0]    == [-30, -20]);
    assert!(over        == None);


    // start and end the same
    let a : Interval = [-10,  10];
    let b : Interval = [-10,  10];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 0               );
    assert!(new_b.len() == 0               );
    assert!(over        == Some([-10,  10]));


    // start same and b tail to right
    let a : Interval = [-10,  10];
    let b : Interval = [-10,  20];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 0               );
    assert!(new_b.len() == 1               );
    assert!(new_b[0]    == [11, 20]        );
    assert!(over        == Some([-10,  10]));


    // b[1] < a[1] start same and a tail to right
    let a : Interval = [-10,  20];
    let b : Interval = [-10,  10];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 1               );
    assert!(new_a[0]    == [11, 20]        );
    assert!(new_b.len() == 0               );
    assert!(over        == Some([-10,  10]));

    // a tail to left and end same
    let a : Interval = [-20,  10];
    let b : Interval = [-10,  10];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 1               );
    assert!(new_a[0]    == [-20, -11]      );
    assert!(new_b.len() == 0               );
    assert!(over        == Some([-10,  10]));

    // a tail to left and b tail to right
    let a : Interval = [-20,  10];
    let b : Interval = [-10,  20];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 1               );
    assert!(new_a[0]    == [-20, -11]      );
    assert!(new_b.len() == 1               );
    assert!(new_b[0]    == [11, 20]        );
    assert!(over        == Some([-10,  10]));

    // a tail to left and a tail to right (b inside a)
    let a : Interval = [-20,  20];
    let b : Interval = [-10,  10];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 2               );
    assert!(new_a.contains(&[-20, -11])    );
    assert!(new_a.contains(&[ 11,  20])    );
    assert!(new_b.len() == 0               );
    assert!(over        == Some([-10,  10]));

    // b tail to left and end same
    let a : Interval = [-10,  10];
    let b : Interval = [-20,  10];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 0               );
    assert!(new_b.len() == 1               );
    assert!(new_b[0]    == [-20, -11]      );
    assert!(over        == Some([-10,  10]));

    // b tail to left and b tail to right (a inside b)
    let a : Interval = [-10,  10];
    let b : Interval = [-20,  20];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 0               );
    assert!(new_b.len() == 2               );
    assert!(new_b.contains(&[-20, -11])    );
    assert!(new_b.contains(&[ 11,  20])    );
    assert!(over        == Some([-10,  10]));

    // b tail to left and a tail to right
    let a : Interval = [-10,  20];
    let b : Interval = [-20,  10];
    let (new_a, new_b, over) = decompose_i(&a, &b);
    assert!(new_a.len() == 1               );
    assert!(new_a[0]    == [ 11,  20]      );
    assert!(new_b.len() == 1               );
    assert!(new_b[0]    == [-20, -11]      );
    assert!(over        == Some([-10,  10]));
}

fn overlap(a: &Rect3D, b: &Rect3D) -> bool {
    if (a[0][1] < b[0][0]) || (b[0][1] < a[0][0]) {
        false
    }
    else if (a[1][1] < b[1][0]) || (b[1][1] < a[1][0]) {
        false
    }
    else if (a[2][1] < b[2][0]) || (b[2][1] < a[2][0]) {
        false
    }
    else {
        true
    }
}

fn insert_on( old: &[Rect3D], ins: &Rect3D ) ->  Vec::<Rect3D> {
    if old.len() == 0 {
        return vec![*ins];
    } else {
        let mut res = Vec::<Rect3D>::new();
        let (_, b, _) = decompose_r(&old[0], ins);
        for rec in b {
            res.extend(insert_on(&old[1..], &rec).iter());
        }
        return res;
    }
}

fn insert_off( old: &[Rect3D], ins: &Rect3D ) ->  Vec::<Rect3D> {
    let mut res = Vec::<Rect3D>::new();

    for rec in old {
        let (a, _, _) = decompose_r(rec, ins);
        for remain in a {
            res.push(remain);
        }
    }

    return res;
}

fn area(rect: &Rect3D) -> i64 {
    (rect[0][1] - rect[0][0] + 1) as i64 *
    (rect[1][1] - rect[1][0] + 1) as i64 *
    (rect[2][1] - rect[2][0] + 1) as i64
}

fn total_area(rects: &[Rect3D]) -> i64 {
    rects.iter().fold(0i64, |acc, x| acc + area(x))
}

fn main() {
    println!("Starting...");

    let file_name = "./input/day22.txt";

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

    let mut on_region = Vec::<Rect3D>::new();

    for line in input_lines.iter() {
        let line : Vec<&str> = line
            .trim()
            .split(|x| " ,.=xyz".contains(x))
            .filter(|x| x.len() > 0)
            .collect();

        let x_from : i32    = line[1].parse().expect("invalid input");
        let x_to   : i32    = line[2].parse().expect("invalid input");
        let y_from : i32    = line[3].parse().expect("invalid input");
        let y_to   : i32    = line[4].parse().expect("invalid input");
        let z_from : i32    = line[5].parse().expect("invalid input");
        let z_to   : i32    = line[6].parse().expect("invalid input");

        if x_from > 50 || x_from < -50 ||
           x_to   > 50 || x_to   < -50 ||
           y_from > 50 || y_from < -50 ||
           y_to   > 50 || y_to   < -50 ||
           z_from > 50 || z_from < -50 ||
           z_to   > 50 || z_to   < -50 {
            continue;
        }

        let rect   : Rect3D = [
            [x_from, x_to],
            [y_from, y_to],
            [z_from, z_to],
        ];

        match *line.get(0).expect("invalid input") {
            "on"  => {
                on_region.extend(insert_on(&on_region, &rect).iter());
            },
            "off" => {
                on_region = insert_off(&on_region, &rect);
            },
            _     => {
                panic!("invalid input");
            },
        }
    }

    println!("Result part 1: {}", total_area(&on_region));
    let mut on_region = Vec::<Rect3D>::new();

    for line in input_lines.iter() {
        let line : Vec<&str> = line
        .trim()
        .split(|x| " ,.=xyz".contains(x))
        .filter(|x| x.len() > 0)
        .collect();

        let x_from : i32    = line[1].parse().expect("invalid input");
        let x_to   : i32    = line[2].parse().expect("invalid input");
        let y_from : i32    = line[3].parse().expect("invalid input");
        let y_to   : i32    = line[4].parse().expect("invalid input");
        let z_from : i32    = line[5].parse().expect("invalid input");
        let z_to   : i32    = line[6].parse().expect("invalid input");

            let rect   : Rect3D = [
                [x_from, x_to],
                [y_from, y_to],
                [z_from, z_to],
            ];

            match *line.get(0).expect("invalid input") {

                "on"  => {
                    on_region.extend(insert_on(&on_region, &rect).iter());
                },
                "off" => {
                    on_region = insert_off(&on_region, &rect);
                },
                _     => {
                    panic!("invalid input");
                },
            }
    }





    println!("Result part 2: {}", total_area(&on_region));
}
