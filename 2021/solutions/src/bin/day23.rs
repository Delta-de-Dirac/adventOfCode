use std::fs;
use std::fmt;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

const A_COST : u32 = 1;
const B_COST : u32 = 10;
const C_COST : u32 = 100;
const D_COST : u32 = 1000;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    A,
    B,
    C,
    D,
}

fn char_to_tile(ch: &char) -> Option<Tile> {
    match ch {
        '.' => {Some(Tile::Empty)},
        'A' => {Some(Tile::A)    },
        'B' => {Some(Tile::B)    },
        'C' => {Some(Tile::C)    },
        'D' => {Some(Tile::D)    },
        _   => {None             }
    }
}

fn tile_to_char(t: &Tile) -> char {
    match t {
        Tile::Empty => {'.'},
        Tile::A     => {'A'},
        Tile::B     => {'B'},
        Tile::C     => {'C'},
        Tile::D     => {'D'},
    }
}

fn tile_cost(t :&Tile) -> u32 {
    match t {
        Tile::Empty => {panic!("Empty has no cost!")},
        Tile::A     => {A_COST},
        Tile::B     => {B_COST},
        Tile::C     => {C_COST},
        Tile::D     => {D_COST},
    }
}

type BurrowState  = [Tile; 15];
type BurrowState2 = [Tile; 23];

#[derive(Eq, PartialEq, Clone)]
struct Burrow {
    cost: u32,
    hall_lt: [Tile; 2 ],
    hall_ab: Tile,
    hall_bc: Tile,
    hall_cd: Tile,
    hall_rt: [Tile; 2 ],
    room_a:  [Tile; 2 ],
    room_b:  [Tile; 2 ],
    room_c:  [Tile; 2 ],
    room_d:  [Tile; 2 ],
}

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "#############")?;
        write!(f,   "#")?;
        for t in self.hall_lt.iter(){
            write!(f, "{}", tile_to_char(t))?;
        }
        write!(f,   ".")?;
        write!(f,   "{}", tile_to_char(&self.hall_ab))?;
        write!(f,   ".")?;
        write!(f,   "{}", tile_to_char(&self.hall_bc))?;
        write!(f,   ".")?;
        write!(f,   "{}", tile_to_char(&self.hall_cd))?;
        write!(f,   ".")?;
        for t in self.hall_rt.iter(){
            write!(f, "{}", tile_to_char(t))?;
        }
        writeln!(f, "#")?;
        write!(f,   "###")?;
        write!(f,   "{}", tile_to_char(&self.room_a[0]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_b[0]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_c[0]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_d[0]))?;
        writeln!(f, "###")?;

        write!(f,   "  #")?;
        write!(f,   "{}", tile_to_char(&self.room_a[1]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_b[1]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_c[1]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_d[1]))?;
        writeln!(f, "#")?;
        writeln!(f, "  #########")?;

        Ok(())
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Burrow2 {
    cost: u32,
    hall_lt: [Tile; 2 ],
    hall_ab: Tile,
    hall_bc: Tile,
    hall_cd: Tile,
    hall_rt: [Tile; 2 ],
    room_a:  [Tile; 4 ],
    room_b:  [Tile; 4 ],
    room_c:  [Tile; 4 ],
    room_d:  [Tile; 4 ],
}

impl Ord for Burrow2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Burrow2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Burrow2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "#############")?;
        write!(f,   "#")?;
        for t in self.hall_lt.iter(){
            write!(f, "{}", tile_to_char(t))?;
        }
        write!(f,   ".")?;
        write!(f,   "{}", tile_to_char(&self.hall_ab))?;
        write!(f,   ".")?;
        write!(f,   "{}", tile_to_char(&self.hall_bc))?;
        write!(f,   ".")?;
        write!(f,   "{}", tile_to_char(&self.hall_cd))?;
        write!(f,   ".")?;
        for t in self.hall_rt.iter(){
            write!(f, "{}", tile_to_char(t))?;
        }
        writeln!(f, "#")?;
        write!(f,   "###")?;
        write!(f,   "{}", tile_to_char(&self.room_a[0]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_b[0]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_c[0]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_d[0]))?;
        writeln!(f, "###")?;

        write!(f,   "  #")?;
        write!(f,   "{}", tile_to_char(&self.room_a[1]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_b[1]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_c[1]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_d[1]))?;
        writeln!(f, "#")?;

        write!(f,   "  #")?;
        write!(f,   "{}", tile_to_char(&self.room_a[2]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_b[2]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_c[2]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_d[2]))?;
        writeln!(f, "#")?;

        write!(f,   "  #")?;
        write!(f,   "{}", tile_to_char(&self.room_a[3]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_b[3]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_c[3]))?;
        write!(f,   "#")?;
        write!(f,   "{}", tile_to_char(&self.room_d[3]))?;
        writeln!(f, "#")?;

        writeln!(f, "  #########")?;

        Ok(())
    }
}

fn convertPart2(bur: &Burrow) -> Burrow2 {
    Burrow2 {
        cost: bur.cost,
        hall_lt: bur.hall_lt,
        hall_ab: bur.hall_ab,
        hall_bc: bur.hall_bc,
        hall_cd: bur.hall_cd,
        hall_rt: bur.hall_rt,
        room_a: [
            bur.room_a[0],
            Tile::D,
            Tile::D,
            bur.room_a[1],
        ],
        room_b: [
            bur.room_b[0],
            Tile::C,
            Tile::B,
            bur.room_b[1],
        ],
        room_c: [
            bur.room_c[0],
            Tile::B,
            Tile::A,
            bur.room_c[1],
        ],
        room_d: [
            bur.room_d[0],
            Tile::A,
            Tile::C,
            bur.room_d[1],
        ],
    }
}

fn get_state(bur: &Burrow) -> BurrowState {
    [
        bur.hall_lt[0],
        bur.hall_lt[1],
        bur.hall_ab,
        bur.hall_bc,
        bur.hall_cd,
        bur.hall_rt[0],
        bur.hall_rt[1],
        bur.room_a[0],
        bur.room_a[1],
        bur.room_b[0],
        bur.room_b[1],
        bur.room_c[0],
        bur.room_c[1],
        bur.room_d[0],
        bur.room_d[1],
    ]
}

fn get_state2(bur: &Burrow2) -> BurrowState2 {
    [
        bur.hall_lt[0],
        bur.hall_lt[1],
        bur.hall_ab,
        bur.hall_bc,
        bur.hall_cd,
        bur.hall_rt[0],
        bur.hall_rt[1],
        bur.room_a[0],
        bur.room_a[1],
        bur.room_a[2],
        bur.room_a[3],
        bur.room_b[0],
        bur.room_b[1],
        bur.room_b[2],
        bur.room_b[3],
        bur.room_c[0],
        bur.room_c[1],
        bur.room_c[2],
        bur.room_c[3],
        bur.room_d[0],
        bur.room_d[1],
        bur.room_d[2],
        bur.room_d[3],
    ]
}

fn is_solved(bur: &Burrow) -> bool {

    if bur.room_a[0] != Tile::A {
        return false;
    }
    if bur.room_a[1] != Tile::A {
        return false;
    }

    if bur.room_b[0] != Tile::B {
        return false;
    }
    if bur.room_b[1] != Tile::B {
        return false;
    }

    if bur.room_c[0] != Tile::C {
        return false;
    }
    if bur.room_c[1] != Tile::C {
        return false;
    }

    if bur.room_d[0] != Tile::D {
        return false;
    }
    if bur.room_d[1] != Tile::D {
        return false;
    }

    return true;
}

fn is_solved2(bur: &Burrow2) -> bool {

    for i in 0..4 {
        if bur.room_a[i] != Tile::A {
            return false;
        }
    }
    for i in 0..4 {
        if bur.room_b[i] != Tile::B {
            return false;
        }
    }
    for i in 0..4 {
        if bur.room_c[i] != Tile::C {
            return false;
        }
    }
    for i in 0..4 {
        if bur.room_d[i] != Tile::D {
            return false;
        }
    }

    return true;
}

fn neighbor_list(bur: &Burrow) -> Vec<Burrow> {
    let mut res: Vec<Burrow> = Vec::new();

    for (i, t) in bur.hall_lt.iter().enumerate() {
        if *t == Tile::Empty {
            continue;
        }
        if i == 0 && bur.hall_lt[1] != Tile::Empty {
            continue;
        }
        //room A
        if *t == Tile::A {
            if bur.room_a[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 0 {4*A_COST} else {3*A_COST};
                new_bur.hall_lt[i] = Tile::Empty;
                new_bur.room_a[1] = Tile::A;
                new_bur.cost += m_cost;
                res.push(new_bur);
            } else if bur.room_a[0] == Tile::Empty && bur.room_a[1] == Tile::A{
                let mut new_bur = bur.clone();
                let m_cost = if i == 0 {3*A_COST} else {2*A_COST};
                new_bur.hall_lt[i] = Tile::Empty;
                new_bur.room_a[0] = Tile::A;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //room B
        if *t == Tile::B && bur.hall_ab == Tile::Empty {
            if bur.room_b[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 0 {6*B_COST} else {5*B_COST};
                new_bur.hall_lt[i] = Tile::Empty;
                new_bur.room_b[1] = Tile::B;
                new_bur.cost += m_cost;
                res.push(new_bur);
            } else if bur.room_b[0] == Tile::Empty && bur.room_b[1] == Tile::B{
                let mut new_bur = bur.clone();
                let m_cost = if i == 0 {5*B_COST} else {4*B_COST};
                new_bur.hall_lt[i] = Tile::Empty;
                new_bur.room_b[0] = Tile::B;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //room C
        if *t == Tile::C && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty {
            if bur.room_c[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 0 {8*C_COST} else {7*C_COST};
                new_bur.hall_lt[i] = Tile::Empty;
                new_bur.room_c[1] = Tile::C;
                new_bur.cost += m_cost;
                res.push(new_bur);
            } else if bur.room_c[0] == Tile::Empty && bur.room_c[1] == Tile::C{
                let mut new_bur = bur.clone();
                let m_cost = if i == 0 {7*C_COST} else {6*C_COST};
                new_bur.hall_lt[i] = Tile::Empty;
                new_bur.room_c[0] = Tile::C;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //room D
        if *t == Tile::D && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            if bur.room_d[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 0 {10*D_COST} else {9*D_COST};
                new_bur.hall_lt[i] = Tile::Empty;
                new_bur.room_d[1] = Tile::D;
                new_bur.cost += m_cost;
                res.push(new_bur);
            } else if bur.room_d[0] == Tile::Empty && bur.room_d[1] == Tile::D{
                let mut new_bur = bur.clone();
                let m_cost = if i == 0 {9*D_COST} else {8*D_COST};
                new_bur.hall_lt[i] = Tile::Empty;
                new_bur.room_d[0] = Tile::D;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
    }
    //room A
    if bur.hall_ab == Tile::A {
        if bur.room_a[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 3*A_COST;
            new_bur.hall_ab = Tile::Empty;
            new_bur.room_a[1] = Tile::A;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_a[0] == Tile::Empty && bur.room_a[1] == Tile::A{
            let mut new_bur = bur.clone();
            let m_cost = 2*A_COST;
            new_bur.hall_ab = Tile::Empty;
            new_bur.room_a[0] = Tile::A;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room B
    if bur.hall_ab == Tile::B {
        if bur.room_b[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 3*B_COST;
            new_bur.hall_ab = Tile::Empty;
            new_bur.room_b[1] = Tile::B;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_b[0] == Tile::Empty && bur.room_b[1] == Tile::B{
            let mut new_bur = bur.clone();
            let m_cost = 2*B_COST;
            new_bur.hall_ab = Tile::Empty;
            new_bur.room_b[0] = Tile::B;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room C
    if bur.hall_ab == Tile::C && bur.hall_bc == Tile::Empty {
        if bur.room_c[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 5*C_COST;
            new_bur.hall_ab = Tile::Empty;
            new_bur.room_c[1] = Tile::C;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_c[0] == Tile::Empty && bur.room_c[1] == Tile::C{
            let mut new_bur = bur.clone();
            let m_cost = 4*C_COST;
            new_bur.hall_ab = Tile::Empty;
            new_bur.room_c[0] = Tile::C;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room D
    if bur.hall_ab == Tile::D && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
        if bur.room_d[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 7*D_COST;
            new_bur.hall_ab = Tile::Empty;
            new_bur.room_d[1] = Tile::D;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_d[0] == Tile::Empty && bur.room_d[1] == Tile::D{
            let mut new_bur = bur.clone();
            let m_cost = 6*D_COST;
            new_bur.hall_ab = Tile::Empty;
            new_bur.room_d[0] = Tile::D;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room A
    if bur.hall_bc == Tile::A && bur.hall_ab == Tile::Empty {
        if bur.room_a[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 5*A_COST;
            new_bur.hall_bc = Tile::Empty;
            new_bur.room_a[1] = Tile::A;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_a[0] == Tile::Empty && bur.room_a[1] == Tile::A{
            let mut new_bur = bur.clone();
            let m_cost = 4*A_COST;
            new_bur.hall_bc = Tile::Empty;
            new_bur.room_a[0] = Tile::A;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room B
    if bur.hall_bc == Tile::B {
        if bur.room_b[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 3*B_COST;
            new_bur.hall_bc = Tile::Empty;
            new_bur.room_b[1] = Tile::B;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_b[0] == Tile::Empty && bur.room_b[1] == Tile::B{
            let mut new_bur = bur.clone();
            let m_cost = 2*B_COST;
            new_bur.hall_bc = Tile::Empty;
            new_bur.room_b[0] = Tile::B;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room C
    if bur.hall_bc == Tile::C {
        if bur.room_c[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 3*C_COST;
            new_bur.hall_bc = Tile::Empty;
            new_bur.room_c[1] = Tile::C;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_c[0] == Tile::Empty && bur.room_c[1] == Tile::C{
            let mut new_bur = bur.clone();
            let m_cost = 2*C_COST;
            new_bur.hall_bc = Tile::Empty;
            new_bur.room_c[0] = Tile::C;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room D
    if bur.hall_bc == Tile::D && bur.hall_cd == Tile::Empty {
        if bur.room_d[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 5*D_COST;
            new_bur.hall_bc = Tile::Empty;
            new_bur.room_d[1] = Tile::D;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_d[0] == Tile::Empty && bur.room_d[1] == Tile::D{
            let mut new_bur = bur.clone();
            let m_cost = 4*D_COST;
            new_bur.hall_bc = Tile::Empty;
            new_bur.room_d[0] = Tile::D;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }

    //room A
    if bur.hall_cd == Tile::A && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty {
        if bur.room_a[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 7*A_COST;
            new_bur.hall_cd = Tile::Empty;
            new_bur.room_a[1] = Tile::A;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_a[0] == Tile::Empty && bur.room_a[1] == Tile::A{
            let mut new_bur = bur.clone();
            let m_cost = 6*A_COST;
            new_bur.hall_cd = Tile::Empty;
            new_bur.room_a[0] = Tile::A;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room B
    if bur.hall_cd == Tile::B && bur.hall_bc == Tile::Empty {
        if bur.room_b[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 6*B_COST;
            new_bur.hall_cd = Tile::Empty;
            new_bur.room_b[1] = Tile::B;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_b[0] == Tile::Empty && bur.room_b[1] == Tile::B{
            let mut new_bur = bur.clone();
            let m_cost = 5*B_COST;
            new_bur.hall_cd = Tile::Empty;
            new_bur.room_b[0] = Tile::B;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room C
    if bur.hall_cd == Tile::C {
        if bur.room_c[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 3*C_COST;
            new_bur.hall_cd = Tile::Empty;
            new_bur.room_c[1] = Tile::C;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_c[0] == Tile::Empty && bur.room_c[1] == Tile::C{
            let mut new_bur = bur.clone();
            let m_cost = 2*C_COST;
            new_bur.hall_cd = Tile::Empty;
            new_bur.room_c[0] = Tile::C;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }
    //room D
    if bur.hall_cd == Tile::D {
        if bur.room_d[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = 3*D_COST;
            new_bur.hall_cd = Tile::Empty;
            new_bur.room_d[1] = Tile::D;
            new_bur.cost += m_cost;
            res.push(new_bur);
        } else if bur.room_d[0] == Tile::Empty && bur.room_d[1] == Tile::D{
            let mut new_bur = bur.clone();
            let m_cost = 2*D_COST;
            new_bur.hall_cd = Tile::Empty;
            new_bur.room_d[0] = Tile::D;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
    }

    for (i, t) in bur.hall_rt.iter().enumerate() {
        if *t == Tile::Empty {
            continue;
        }
        if i == 1 && bur.hall_rt[0] != Tile::Empty {
            continue;
        }
        //room A
        if *t == Tile::A && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            if bur.room_a[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {10*A_COST} else {9*A_COST};
                new_bur.hall_rt[i] = Tile::Empty;
                new_bur.room_a[1] = Tile::A;
                new_bur.cost += m_cost;
                res.push(new_bur);
            } else if bur.room_a[0] == Tile::Empty && bur.room_a[1] == Tile::A{
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {9*A_COST} else {8*A_COST};
                new_bur.hall_rt[i] = Tile::Empty;
                new_bur.room_a[0] = Tile::A;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //room B
        if *t == Tile::B && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            if bur.room_b[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {8*B_COST} else {7*B_COST};
                new_bur.hall_rt[i] = Tile::Empty;
                new_bur.room_b[1] = Tile::B;
                new_bur.cost += m_cost;
                res.push(new_bur);
            } else if bur.room_b[0] == Tile::Empty && bur.room_b[1] == Tile::B{
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {7*B_COST} else {6*B_COST};
                new_bur.hall_rt[i] = Tile::Empty;
                new_bur.room_b[0] = Tile::B;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //room C
        if *t == Tile::C && bur.hall_cd == Tile::Empty {
            if bur.room_c[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {6*C_COST} else {5*C_COST};
                new_bur.hall_rt[i] = Tile::Empty;
                new_bur.room_c[1] = Tile::C;
                new_bur.cost += m_cost;
                res.push(new_bur);
            } else if bur.room_c[0] == Tile::Empty && bur.room_c[1] == Tile::C{
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {5*C_COST} else {4*C_COST};
                new_bur.hall_rt[i] = Tile::Empty;
                new_bur.room_c[0] = Tile::C;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //room D
        if *t == Tile::D {
            if bur.room_d[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {4*D_COST} else {3*D_COST};
                new_bur.hall_rt[i] = Tile::Empty;
                new_bur.room_d[1] = Tile::D;
                new_bur.cost += m_cost;
                res.push(new_bur);
            } else if bur.room_d[0] == Tile::Empty && bur.room_d[1] == Tile::D{
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {3*D_COST} else {2*D_COST};
                new_bur.hall_rt[i] = Tile::Empty;
                new_bur.room_d[0] = Tile::D;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
    }

    for (i, t) in bur.room_a.iter().enumerate() { // from room A
        if *t == Tile::Empty {
            continue;
        }
        if i == 1 && bur.room_a[0] != Tile::Empty {
            continue;
        }
        if *t == Tile::A {
            if i == 1 {
                continue;
            } else {
                if bur.room_a[1] == Tile::A {
                    continue;
                }
            }
        }
        //hall lt
        if bur.hall_lt[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {3*tile_cost(t)} else {2*tile_cost(t)};
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_lt[1] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_lt[0] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {4*tile_cost(t)} else {3*tile_cost(t)};
                new_bur.room_a[i] = Tile::Empty;
                new_bur.hall_lt[0] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //hall_ab
        if bur.hall_ab == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {3*tile_cost(t)} else {2*tile_cost(t)};
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_ab = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_bc
        if bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {5*tile_cost(t)} else {4*tile_cost(t)};
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_bc = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_cd
        if bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {7*tile_cost(t)} else {6*tile_cost(t)};
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_cd = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall rt
        if bur.hall_rt[0] == Tile::Empty && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {9*tile_cost(t)} else {8*tile_cost(t)};
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_rt[0] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_rt[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {10*tile_cost(t)} else {9*tile_cost(t)};
                new_bur.room_a[i] = Tile::Empty;
                new_bur.hall_rt[1] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
    }

    for (i, t) in bur.room_b.iter().enumerate() { // from room B
        if *t == Tile::Empty {
            continue;
        }
        if i == 1 && bur.room_b[0] != Tile::Empty {
            continue;
        }
        if *t == Tile::B {
            if i == 1 {
                continue;
            } else {
                if bur.room_b[1] == Tile::B {
                    continue;
                }
            }
        }
        //hall lt
        if bur.hall_lt[1] == Tile::Empty && bur.hall_ab == Tile::Empty{
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {5*tile_cost(t)} else {4*tile_cost(t)};
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_lt[1] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_lt[0] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {6*tile_cost(t)} else {5*tile_cost(t)};
                new_bur.room_b[i] = Tile::Empty;
                new_bur.hall_lt[0] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //hall_ab
        if bur.hall_ab == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {3*tile_cost(t)} else {2*tile_cost(t)};
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_ab = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_bc
        if bur.hall_bc == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {3*tile_cost(t)} else {2*tile_cost(t)};
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_bc = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_cd
        if bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {5*tile_cost(t)} else {4*tile_cost(t)};
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_cd = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall rt
        if bur.hall_rt[0] == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {7*tile_cost(t)} else {6*tile_cost(t)};
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_rt[0] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_rt[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {8*tile_cost(t)} else {7*tile_cost(t)};
                new_bur.room_b[i] = Tile::Empty;
                new_bur.hall_rt[1] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
    }

    for (i, t) in bur.room_c.iter().enumerate() { // from room C
        if *t == Tile::Empty {
            continue;
        }
        if i == 1 && bur.room_c[0] != Tile::Empty {
            continue;
        }
        if *t == Tile::C {
            if i == 1 {
                continue;
            } else {
                if bur.room_c[1] == Tile::C {
                    continue;
                }
            }
        }
        //hall lt
        if bur.hall_lt[1] == Tile::Empty && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty{
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {7*tile_cost(t)} else {6*tile_cost(t)};
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_lt[1] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_lt[0] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {8*tile_cost(t)} else {7*tile_cost(t)};
                new_bur.room_c[i] = Tile::Empty;
                new_bur.hall_lt[0] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //hall_ab
        if bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {5*tile_cost(t)} else {4*tile_cost(t)};
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_ab = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_bc
        if bur.hall_bc == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {3*tile_cost(t)} else {2*tile_cost(t)};
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_bc = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_cd
        if bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {3*tile_cost(t)} else {2*tile_cost(t)};
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_cd = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall rt
        if bur.hall_rt[0] == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {5*tile_cost(t)} else {4*tile_cost(t)};
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_rt[0] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_rt[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {6*tile_cost(t)} else {5*tile_cost(t)};
                new_bur.room_c[i] = Tile::Empty;
                new_bur.hall_rt[1] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
    }

    for (i, t) in bur.room_d.iter().enumerate() { // from room D
        if *t == Tile::Empty {
            continue;
        }
        if i == 1 && bur.room_d[0] != Tile::Empty {
            continue;
        }
        if *t == Tile::D {
            if i == 1 {
                continue;
            } else {
                if bur.room_d[1] == Tile::D {
                    continue;
                }
            }
        }
        //hall lt
        if bur.hall_lt[1] == Tile::Empty && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty{
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {9*tile_cost(t)} else {8*tile_cost(t)};
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_lt[1] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_lt[0] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {10*tile_cost(t)} else {9*tile_cost(t)};
                new_bur.room_d[i] = Tile::Empty;
                new_bur.hall_lt[0] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //hall_ab
        if bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {7*tile_cost(t)} else {6*tile_cost(t)};
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_ab = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_bc
        if bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {5*tile_cost(t)} else {4*tile_cost(t)};
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_bc = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_cd
        if bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {3*tile_cost(t)} else {2*tile_cost(t)};
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_cd = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall rt
        if bur.hall_rt[0] == Tile::Empty {
            let mut new_bur = bur.clone();
            let m_cost = if i == 1 {3*tile_cost(t)} else {2*tile_cost(t)};
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_rt[0] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_rt[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let m_cost = if i == 1 {4*tile_cost(t)} else {3*tile_cost(t)};
                new_bur.room_d[i] = Tile::Empty;
                new_bur.hall_rt[1] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
    }

    return res;
}

fn neighbor_list2(bur: &Burrow2) -> Vec<Burrow2> {
    let mut res: Vec<Burrow2> = Vec::new();

    for (i, t) in bur.hall_lt.iter().enumerate() {
        if *t == Tile::Empty {
            continue;
        }
        if i == 0 && bur.hall_lt[1] != Tile::Empty {
            continue;
        }
        //room A
        if *t == Tile::A {
            for j in (0..4).rev() {
                if bur.room_a[j] == Tile::Empty {
                    let mut new_bur = bur.clone();
                    let steps = (3+j-i) as u32;
                    let m_cost = steps*A_COST;
                    new_bur.hall_lt[i] = Tile::Empty;
                    new_bur.room_a[j] = Tile::A;
                    new_bur.cost += m_cost;
                    res.push(new_bur);
                }
                else if bur.room_a[j] == Tile::A {
                    continue;
                }
                else {
                    break;
                }
            }
        }
        //room B
        if *t == Tile::B && bur.hall_ab == Tile::Empty {
            for j in (0..4).rev() {
                if bur.room_b[j] == Tile::Empty {
                    let mut new_bur = bur.clone();
                    let steps = (5+j-i) as u32;
                    let m_cost = steps*B_COST;
                    new_bur.hall_lt[i] = Tile::Empty;
                    new_bur.room_b[j] = Tile::B;
                    new_bur.cost += m_cost;
                    res.push(new_bur);
                }
                else if bur.room_b[j] == Tile::B {
                    continue;
                }
                else {
                    break;
                }
            }
        }
        //room C
        if *t == Tile::C && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty {
            for j in (0..4).rev() {
                if bur.room_c[j] == Tile::Empty {
                    let mut new_bur = bur.clone();
                    let steps = (7+j-i) as u32;
                    let m_cost = steps*C_COST;
                    new_bur.hall_lt[i] = Tile::Empty;
                    new_bur.room_c[j] = Tile::C;
                    new_bur.cost += m_cost;
                    res.push(new_bur);
                }
                else if bur.room_c[j] == Tile::C {
                    continue;
                }
                else {
                    break;
                }
            }
        }
        //room D
        if *t == Tile::D && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            for j in (0..4).rev() {
                if bur.room_d[j] == Tile::Empty {
                    let mut new_bur = bur.clone();
                    let steps = (9+j-i) as u32;
                    let m_cost = steps*D_COST;
                    new_bur.hall_lt[i] = Tile::Empty;
                    new_bur.room_d[j] = Tile::D;
                    new_bur.cost += m_cost;
                    res.push(new_bur);
                }
                else if bur.room_d[j] == Tile::D {
                    continue;
                }
                else {
                    break;
                }
            }
        }
    }
    //room A
    if bur.hall_ab == Tile::A {
        for j in (0..4).rev() {
            if bur.room_a[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (2+j) as u32;
                let m_cost = steps*A_COST;
                new_bur.hall_ab = Tile::Empty;
                new_bur.room_a[j] = Tile::A;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_a[j] == Tile::A {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room B
    if bur.hall_ab == Tile::B {
        for j in (0..4).rev() {
            if bur.room_b[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (2+j) as u32;
                let m_cost = steps*B_COST;
                new_bur.hall_ab = Tile::Empty;
                new_bur.room_b[j] = Tile::B;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_b[j] == Tile::B {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room C
    if bur.hall_ab == Tile::C && bur.hall_bc == Tile::Empty {
        for j in (0..4).rev() {
            if bur.room_c[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (4+j) as u32;
                let m_cost = steps*C_COST;
                new_bur.hall_ab = Tile::Empty;
                new_bur.room_c[j] = Tile::C;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_c[j] == Tile::C {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room D
    if bur.hall_ab == Tile::D && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
        for j in (0..4).rev() {
            if bur.room_d[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (6+j) as u32;
                let m_cost = steps*D_COST;
                new_bur.hall_ab = Tile::Empty;
                new_bur.room_d[j] = Tile::D;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_d[j] == Tile::D {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room A
    if bur.hall_bc == Tile::A && bur.hall_ab == Tile::Empty {
        for j in (0..4).rev() {
            if bur.room_a[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (4+j) as u32;
                let m_cost = steps*A_COST;
                new_bur.hall_bc = Tile::Empty;
                new_bur.room_a[j] = Tile::A;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_a[j] == Tile::A {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room B
    if bur.hall_bc == Tile::B {
        for j in (0..4).rev() {
            if bur.room_b[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (2+j) as u32;
                let m_cost = steps*B_COST;
                new_bur.hall_bc = Tile::Empty;
                new_bur.room_b[j] = Tile::B;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_b[j] == Tile::B {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room C
    if bur.hall_bc == Tile::C {
        for j in (0..4).rev() {
            if bur.room_c[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (2+j) as u32;
                let m_cost = steps*C_COST;
                new_bur.hall_bc = Tile::Empty;
                new_bur.room_c[j] = Tile::C;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_c[j] == Tile::C {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room D
    if bur.hall_bc == Tile::D && bur.hall_cd == Tile::Empty {
        for j in (0..4).rev() {
            if bur.room_d[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (4+j) as u32;
                let m_cost = steps*D_COST;
                new_bur.hall_bc = Tile::Empty;
                new_bur.room_d[j] = Tile::D;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_d[j] == Tile::D {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room A
    if bur.hall_cd == Tile::A && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty {
        for j in (0..4).rev() {
            if bur.room_a[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (6+j) as u32;
                let m_cost = steps*A_COST;
                new_bur.hall_cd = Tile::Empty;
                new_bur.room_a[j] = Tile::A;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_a[j] == Tile::A {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room B
    if bur.hall_cd == Tile::B && bur.hall_bc == Tile::Empty {
        for j in (0..4).rev() {
            if bur.room_b[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (4+j) as u32;
                let m_cost = steps*B_COST;
                new_bur.hall_cd = Tile::Empty;
                new_bur.room_b[j] = Tile::B;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_b[j] == Tile::B {
                continue;
            }
            else {
                break;
            }
        }
    }

    //room C
    if bur.hall_cd == Tile::C {
        for j in (0..4).rev() {
            if bur.room_c[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (2+j) as u32;
                let m_cost = steps*C_COST;
                new_bur.hall_cd = Tile::Empty;
                new_bur.room_c[j] = Tile::C;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_c[j] == Tile::C {
                continue;
            }
            else {
                break;
            }
        }
    }


    //room D
    if bur.hall_cd == Tile::D {
        for j in (0..4).rev() {
            if bur.room_d[j] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (2+j) as u32;
                let m_cost = steps*D_COST;
                new_bur.hall_cd = Tile::Empty;
                new_bur.room_d[j] = Tile::D;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
            else if bur.room_d[j] == Tile::D {
                continue;
            }
            else {
                break;
            }
        }
    }

    for (i, t) in bur.hall_rt.iter().enumerate() {
        if *t == Tile::Empty {
            continue;
        }
        if i == 1 && bur.hall_rt[0] != Tile::Empty {
            continue;
        }
        //room A
        if *t == Tile::A && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            for j in (0..4).rev() {
                if bur.room_a[j] == Tile::Empty {
                    let mut new_bur = bur.clone();
                    let steps = (8+j+i) as u32;
                    let m_cost = steps*A_COST;
                    new_bur.hall_rt[i] = Tile::Empty;
                    new_bur.room_a[j] = Tile::A;
                    new_bur.cost += m_cost;
                    res.push(new_bur);
                }
                else if bur.room_a[j] == Tile::A {
                    continue;
                }
                else {
                    break;
                }
            }
        }
        //room B
        if *t == Tile::B && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            for j in (0..4).rev() {
                if bur.room_b[j] == Tile::Empty {
                    let mut new_bur = bur.clone();
                    let steps = (6+j+i) as u32;
                    let m_cost = steps*B_COST;
                    new_bur.hall_rt[i] = Tile::Empty;
                    new_bur.room_b[j] = Tile::B;
                    new_bur.cost += m_cost;
                    res.push(new_bur);
                }
                else if bur.room_b[j] == Tile::B {
                    continue;
                }
                else {
                    break;
                }
            }
        }
        //room C
        if *t == Tile::C && bur.hall_cd == Tile::Empty {
            for j in (0..4).rev() {
                if bur.room_c[j] == Tile::Empty {
                    let mut new_bur = bur.clone();
                    let steps = (4+j+i) as u32;
                    let m_cost = steps*C_COST;
                    new_bur.hall_rt[i] = Tile::Empty;
                    new_bur.room_c[j] = Tile::C;
                    new_bur.cost += m_cost;
                    res.push(new_bur);
                }
                else if bur.room_c[j] == Tile::C {
                    continue;
                }
                else {
                    break;
                }
            }
        }
        //room D
        if *t == Tile::D {
            for j in (0..4).rev() {
                if bur.room_d[j] == Tile::Empty {
                    let mut new_bur = bur.clone();
                    let steps = (2+j+i) as u32;
                    let m_cost = steps*D_COST;
                    new_bur.hall_rt[i] = Tile::Empty;
                    new_bur.room_d[j] = Tile::D;
                    new_bur.cost += m_cost;
                    res.push(new_bur);
                }
                else if bur.room_d[j] == Tile::D {
                    continue;
                }
                else {
                    break;
                }
            }
        }
    }

    for (i, t) in bur.room_a.iter().enumerate() { // from room A
        if *t == Tile::Empty {
            continue;
        }
        if bur.room_a[i..].iter().all(|x| *x == Tile::A) {
            break;
        }
        //hall lt
        if bur.hall_lt[1] == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (2 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_lt[1] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_lt[0] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (3 + i) as u32;
                let m_cost = steps*tile_cost(t);
                new_bur.room_a[i] = Tile::Empty;
                new_bur.hall_lt[0] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //hall_ab
        if bur.hall_ab == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (2 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_ab = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_bc
        if bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (4 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_bc = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_cd
        if bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (6 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_cd = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall rt
        if bur.hall_rt[0] == Tile::Empty && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (8 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_a[i] = Tile::Empty;
            new_bur.hall_rt[0] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_rt[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (9 + i) as u32;
                let m_cost = steps*tile_cost(t);
                new_bur.room_a[i] = Tile::Empty;
                new_bur.hall_rt[1] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        break;
    }

    for (i, t) in bur.room_b.iter().enumerate() { // from room B
        if *t == Tile::Empty {
            continue;
        }
        if bur.room_b[i..].iter().all(|x| *x == Tile::B) {
            break;
        }


        if bur.hall_lt[1] == Tile::Empty && bur.hall_ab == Tile::Empty{
            let mut new_bur = bur.clone();
            let steps = (4 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_lt[1] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_lt[0] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (5 + i) as u32;
                let m_cost = steps*tile_cost(t);
                new_bur.room_b[i] = Tile::Empty;
                new_bur.hall_lt[0] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //hall_ab
        if bur.hall_ab == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (2 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_ab = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }

        //hall_bc
        if bur.hall_bc == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (2 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_bc = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }

        //hall_cd
        if bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (4 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_cd = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall rt
        if bur.hall_rt[0] == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (6 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_b[i] = Tile::Empty;
            new_bur.hall_rt[0] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_rt[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (7 + i) as u32;
                let m_cost = steps*tile_cost(t);
                new_bur.room_b[i] = Tile::Empty;
                new_bur.hall_rt[1] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        break;
    }

    for (i, t) in bur.room_c.iter().enumerate() { // from room C
        if *t == Tile::Empty {
            continue;
        }
        if bur.room_c[i..].iter().all(|x| *x == Tile::C) {
            break;
        }

        //hall lt
        if bur.hall_lt[1] == Tile::Empty && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty{
            let mut new_bur = bur.clone();
            let steps = (6 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_lt[1] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_lt[0] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (7 + i) as u32;
                let m_cost = steps*tile_cost(t);
                new_bur.room_c[i] = Tile::Empty;
                new_bur.hall_lt[0] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }

        //hall_ab
        if bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (4 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_ab = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_bc
        if bur.hall_bc == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (2 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_bc = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall_cd
        if bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (2 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_cd = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }
        //hall rt
        if bur.hall_rt[0] == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (4 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_c[i] = Tile::Empty;
            new_bur.hall_rt[0] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_rt[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (5 + i) as u32;
                let m_cost = steps*tile_cost(t);
                new_bur.room_c[i] = Tile::Empty;
                new_bur.hall_rt[1] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        break;
    }

    for (i, t) in bur.room_d.iter().enumerate() { // from room D
        if *t == Tile::Empty {
            continue;
        }
        if bur.room_d[i..].iter().all(|x| *x == Tile::D) {
            break;
        }

        //hall lt
        if bur.hall_lt[1] == Tile::Empty && bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty{
            let mut new_bur = bur.clone();
            let steps = (8 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_lt[1] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_lt[0] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (9 + i) as u32;
                let m_cost = steps*tile_cost(t);
                new_bur.room_d[i] = Tile::Empty;
                new_bur.hall_lt[0] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        //hall_ab
        if bur.hall_ab == Tile::Empty && bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (6 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_ab = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }

        //hall_bc
        if bur.hall_bc == Tile::Empty && bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (4 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_bc = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }

        //hall_cd
        if bur.hall_cd == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (2 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_cd = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
        }

        //hall rt
        if bur.hall_rt[0] == Tile::Empty {
            let mut new_bur = bur.clone();
            let steps = (2 + i) as u32;
            let m_cost = steps*tile_cost(t);
            new_bur.room_d[i] = Tile::Empty;
            new_bur.hall_rt[0] = *t;
            new_bur.cost += m_cost;
            res.push(new_bur);
            if bur.hall_rt[1] == Tile::Empty {
                let mut new_bur = bur.clone();
                let steps = (3 + i) as u32;
                let m_cost = steps*tile_cost(t);
                new_bur.room_d[i] = Tile::Empty;
                new_bur.hall_rt[1] = *t;
                new_bur.cost += m_cost;
                res.push(new_bur);
            }
        }
        break;
    }

    return res;
}

fn shortest_path(bur: &Burrow) -> Option<u32> {
    let mut dist: HashMap<BurrowState, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(get_state(bur), bur.cost);
    heap.push(bur.clone());
    while let Some(heap_bur) = heap.pop() {
        if is_solved(&heap_bur) { return Some(heap_bur.cost); }
        if let Some(cost_already_at_dist) = dist.get(&get_state(&heap_bur)){
            if heap_bur.cost > *cost_already_at_dist { continue; } /**/
        }
        for next in &neighbor_list(&heap_bur) {
            if let Some(cost_already_at_dist) = dist.get(&get_state(next)){
                if next.cost >= *cost_already_at_dist {
                    continue;
                }
            }
            heap.push(next.clone());
            dist.insert(get_state(next), next.cost);
        }
    }
    None
}

fn shortest_path2(bur: &Burrow2) -> Option<u32> {
    let mut dist: HashMap<BurrowState2, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(get_state2(bur), bur.cost);
    heap.push(bur.clone());
    while let Some(heap_bur) = heap.pop() {
        if is_solved2(&heap_bur) { return Some(heap_bur.cost); }
        if let Some(cost_already_at_dist) = dist.get(&get_state2(&heap_bur)){
            if heap_bur.cost > *cost_already_at_dist { continue; } /**/
        }
        for next in &neighbor_list2(&heap_bur) {
            if let Some(cost_already_at_dist) = dist.get(&get_state2(next)){
                if next.cost >= *cost_already_at_dist {
                    continue;
                }
            }
            heap.push(next.clone());
            dist.insert(get_state2(next), next.cost);
        }
    }
    None
}


fn main() {
    println!("Starting...");

    let file_name = "./input/day23.txt";

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

    let mut bur = Burrow {
        cost: 0,
        hall_lt: [Tile::Empty; 2 ],
        hall_ab:  Tile::Empty,
        hall_bc:  Tile::Empty,
        hall_cd:  Tile::Empty,
        hall_rt: [Tile::Empty; 2 ],
        room_a : [Tile::Empty; 2 ],
        room_b : [Tile::Empty; 2 ],
        room_c : [Tile::Empty; 2 ],
        room_d : [Tile::Empty; 2 ],
    };

    for (i, ch) in input_lines[1][1..=11].chars().enumerate() {
        match  i {
            0..2          => {
                bur.hall_lt[i]   = char_to_tile(&ch).expect("invalid input");
            },
            3             => {
                bur.hall_ab      = char_to_tile(&ch).expect("invalid input");
            },
            5             => {
                bur.hall_bc      = char_to_tile(&ch).expect("invalid input");
            },
            7             => {
                bur.hall_cd      = char_to_tile(&ch).expect("invalid input");
            },
            9..11         => {
                bur.hall_rt[i-9] = char_to_tile(&ch).expect("invalid input");
            }
            2 | 4 | 6 | 8 => {continue;}
            _ => {panic!("invalid input")},
        }

    }

    let ch = &input_lines[2].chars().nth(3).expect("invalid input");
    bur.room_a[0] = char_to_tile(ch).expect("invalid input");

    let ch = &input_lines[3].chars().nth(3).expect("invalid input");
    bur.room_a[1] = char_to_tile(ch).expect("invalid input");

    let ch = &input_lines[2].chars().nth(5).expect("invalid input");
    bur.room_b[0] = char_to_tile(ch).expect("invalid input");

    let ch = &input_lines[3].chars().nth(5).expect("invalid input");
    bur.room_b[1] = char_to_tile(ch).expect("invalid input");

    let ch = &input_lines[2].chars().nth(7).expect("invalid input");
    bur.room_c[0] = char_to_tile(ch).expect("invalid input");

    let ch = &input_lines[3].chars().nth(7).expect("invalid input");
    bur.room_c[1] = char_to_tile(ch).expect("invalid input");

    let ch = &input_lines[2].chars().nth(9).expect("invalid input");
    bur.room_d[0] = char_to_tile(ch).expect("invalid input");

    let ch = &input_lines[3].chars().nth(9).expect("invalid input");
    bur.room_d[1] = char_to_tile(ch).expect("invalid input");


    if let Some(result) = shortest_path(&bur) {
        println!("Result part 1: {}", result);
    } else {
        println!("Result part 1: Could not find path");
    }

    let bur2 = convertPart2(&bur);

    if let Some(result) = shortest_path2(&bur2) {
        println!("Result part 2: {}", result);
    } else {
        println!("Result part 2: Could not find path");
    }

}
