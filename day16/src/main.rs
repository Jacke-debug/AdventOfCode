use std::{collections::HashMap, char};

fn read_bit (c: &[char]) -> usize {
    let c: String = c.into_iter().collect();
    usize::from_str_radix(&c, 2).unwrap()
}

enum OperatorState {
    Sum(usize), 
    Product(usize),
    Min(usize), 
    Max(usize), 
    Gt(Vec<usize>), 
    Lt(Vec<usize>), 
    Eq(Vec<usize>)
}

impl OperatorState {
    fn insert(&mut self, val: usize) {
        match self {
            OperatorState::Sum(state) => {*state +=val; }
            OperatorState::Product(state) => {*state *=val; }
            OperatorState::Min(state) => {*state = (*state).min(val); }
            OperatorState::Max(state) => {*state = (*state).max(val); }
            OperatorState::Gt(vals) => {vals.push(val)},
            OperatorState::Lt(vals) => {vals.push(val)},
            OperatorState::Eq(vals) => {vals.push(val)},
        }
    }

    fn val(&self) -> usize {
        match  self {
            OperatorState::Sum(val) => *val, 
            OperatorState::Product(val) => *val,
            OperatorState::Min(val) => *val,
            OperatorState::Max(val) => *val,
            OperatorState::Gt(vals) => if vals[0] > vals[1] {1} else {0},
            OperatorState::Lt(vals) => if vals[0] < vals[1] {1} else {0},
            OperatorState::Eq(vals) => if vals[0] == vals[1] {1} else {0},
        }
    }
}

fn main() {
    let part_a = false;

    let input = include_str!("bits.txt");
    let mut hex_map = HashMap::new();
    for line in input.trim().split("\r\n") {
        let (hex, bit) = line.split_once(" = ").unwrap();
        hex_map.insert(hex.chars().next().unwrap(), bit);
    }

    let mut data: String = "".to_string();
    let input = include_str!("input.txt");
    for c in input.chars() {
        let code = hex_map.get(&c).unwrap();
        data.push_str(code);
    }

    let mut i = 0;
    let mut decoded: Vec<char> = data.chars().collect();

    let mut ver_sum = 0;
    while i + 6 < decoded.len() {
        if decoded.len()-i < 20 && read_bit(&decoded[i..]) == 0 {
            break;
        }
        ver_sum += parse(&mut i, &mut decoded, part_a)
    }
    println!("code: {}", ver_sum);
}

fn parse(i: &mut usize, decoded: &mut [char], part_a: bool) -> usize{
    
    let version = read_bit(&decoded[*i..*i+3]);

    let id = read_bit(&decoded[*i+3..*i+6]);
    eprintln!("{}, {}", version, id);

    *i += 6;
    if id == 4 {
        // Literal
        let mut continues = true;

        let mut val = Vec::new();
        while continues {
            continues = decoded[*i] == '1';
            let mut decoded: Vec<char> = decoded[*i + 1..*i + 5].iter().copied().collect();
            val.append(&mut decoded);

            *i += 5;
        }
        let val = read_bit(&val);
        eprintln!("literal {}", val);
        if part_a {
            return version;
        } else {
            return val;
        }
    
    } else {
        let mut op = if part_a {
            OperatorState::Sum(version)
        } else {
            match id {
                0 => OperatorState::Sum(0),
                1 => OperatorState::Product(1),
                2 => OperatorState::Min(usize::MAX),
                3 => OperatorState::Max(0),
                5 => OperatorState::Gt(vec![]),
                6 => OperatorState::Lt(vec![]),
                7 => OperatorState::Eq(vec![]),
                _ => panic!(),
            }
        };
        // Operator
        let length_id = read_bit(&[decoded[*i]]);
        *i += 1;
        if length_id == 0 {
            let length: usize = read_bit(&decoded[*i..*i+15]);
            *i += 15;
            let stop_at = *i + length;
            while *i < stop_at {
                op.insert(parse(i, decoded, part_a));
            }
            assert!(*i == stop_at);

        } else if length_id == 1 {
            let length = read_bit(&decoded[*i..*i+11]);
            *i += 11;
            for j in 0..length {
                op.insert(parse(i, decoded, part_a));
            }
        }
        return op.val();
    }
}



