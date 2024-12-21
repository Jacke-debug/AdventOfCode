use std::collections::btree_map::Keys;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn new(x: isize, y: isize) -> Pos {
        Pos { x: x, y: y }
    }

    fn numpad_map(&self, end: &Pos) -> Vec<char> {
        let diff = end - self;
        let mut instructions = Vec::new();
        let x_count = diff.x.abs();
        let y_count = diff.y.abs();
        if self.x == 3 && end.y == 0 {
            // Must move up first
            if diff.y < 0 {
                instructions.extend(std::iter::repeat('^').take(y_count as usize));
            }
            if diff.x < 0 {
                instructions.extend(std::iter::repeat('<').take(x_count as usize));
            }
        } else if self.y == 0 && end.x == 3 {
            // Must move right first
            if diff.x > 0 {
                instructions.extend(std::iter::repeat('>').take(x_count as usize));
            }
            if diff.y > 0 {
                instructions.extend(std::iter::repeat('v').take(y_count as usize));
            }
        } else {
            // Left first
            if diff.x < 0 {
                instructions.extend(std::iter::repeat('<').take(x_count as usize));
            }
            if diff.y > 0 {
                instructions.extend(std::iter::repeat('v').take(y_count as usize));
            }
            if diff.y < 0 {
                instructions.extend(std::iter::repeat('^').take(y_count as usize));
            }
            if diff.x > 0 {
                instructions.extend(std::iter::repeat('>').take(x_count as usize));
            }
        }
    
        
        // Move right before up or down
        // Move up or down before left
        instructions
    }

    fn keypad_map(&self, end: &Pos, keypad: &HashMap<char, Pos>) -> Vec<char> {
        let diff = end - self;

        let mut instructions = Vec::new();
        let x_count = diff.x.abs();
        let y_count = diff.y.abs();
        if self.y == 0 && end.x == 0 {
            // Must move down first
            if diff.y > 0 {
                instructions.extend(std::iter::repeat('v').take(y_count as usize));
            }
            if diff.x < 0 {
                instructions.extend(std::iter::repeat('<').take(x_count as usize));
            } 
        } else if self.x == 0 && end.y == 0 {
            // Must move right first
            if diff.x > 0 {
                instructions.extend(std::iter::repeat('>').take(x_count as usize));
            }
            if diff.y < 0 {
                instructions.extend(std::iter::repeat('^').take(y_count as usize));
            }
        } else {
            if diff.x < 0 {
                instructions.extend(std::iter::repeat('<').take(x_count as usize));
            } 
            if diff.y > 0 {
                instructions.extend(std::iter::repeat('v').take(y_count as usize));
            }
            if diff.y < 0 {
                instructions.extend(std::iter::repeat('^').take(y_count as usize));
            }
            if diff.x > 0 {
                instructions.extend(std::iter::repeat('>').take(x_count as usize));
            }
        }
        
        if !instructions.is_empty() {
            println!("Moving from {} to {} with {}", find_char_by_pos(keypad, self), 
            find_char_by_pos(keypad, end), instructions.iter().collect::<String>());
        }

        // Move right before up or down
        // Move up or down before left
        instructions
    }
}
impl Sub for &Pos {
    type Output = Pos;

    fn sub(self, other: &Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Add for &Pos {
    type Output = Pos;

    fn add(self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn find_char_by_pos(keypad: &HashMap<char, Pos>, pos: &Pos) -> char {
    keypad.iter().find_map(|(&key, value)| if value == pos { Some(key) } else { None }).unwrap()
}

fn extract_numeric_part(vec: &Vec<char>) -> usize {
    let numeric_string: String = vec
        .into_iter()
        .filter(|c| c.is_digit(10))
        .collect();

    numeric_string.parse::<usize>().unwrap()
}

fn solve(input: &str) -> usize {
    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let numpad = HashMap::from([
        ('7', Pos::new(0, 0)),
        ('8', Pos::new(1, 0)),
        ('9', Pos::new(2, 0)),
        ('4', Pos::new(0, 1)),
        ('5', Pos::new(1, 1)),
        ('6', Pos::new(2, 1)),
        ('1', Pos::new(0, 2)),
        ('2', Pos::new(1, 2)),
        ('3', Pos::new(2, 2)),
        ('0', Pos::new(1, 3)),
        ('A', Pos::new(2, 3)),
    ]);

    let keypad = HashMap::from([
        ('<', Pos::new(0, 1)),
        ('^', Pos::new(1, 0)),
        ('v', Pos::new(1, 1)),
        ('>', Pos::new(2, 1)),
        ('A', Pos::new(2, 0)),
    ]);

    let mut ans_a = 0;
    let mut numpad_pos = numpad.get(&'A').unwrap();
    let mut radiation_pos = keypad.get(&'A').unwrap();
    let mut could_pos = keypad.get(&'A').unwrap();
    for code in codes {
        println!("Code: {}", code.iter().collect::<String>());
        let mut robot_1 = Vec::new();
        let numeric_code = extract_numeric_part(&code);
        
        for button in code {
            let new_pos = numpad.get(&button).unwrap();
            robot_1.extend(numpad_pos.numpad_map(new_pos));
            robot_1.push('A'); // Add a press of the button 
            numpad_pos = new_pos;
        }
        println!("robot_1: {}", robot_1.iter().collect::<String>());

        let mut robot_2 = Vec::new();
        for button in robot_1 {
            let new_pos = keypad.get(&button).unwrap();
            robot_2.extend(radiation_pos.keypad_map(new_pos, &keypad));
            robot_2.push('A'); // Add a press of the button
            radiation_pos = new_pos;
        }
        println!("robot_2: {}", robot_2.iter().collect::<String>());

        let mut human_sequence = Vec::new();
        for button in robot_2 {
            let new_pos = keypad.get(&button).unwrap();
            human_sequence.extend(could_pos.keypad_map(new_pos, &keypad));
            human_sequence.push('A'); // Add a press of the button
            could_pos = new_pos;
        }
        
        
        println!("Human sequence: {}", human_sequence.iter().collect::<String>());
        println!("Ans: {} * {}", human_sequence.len(), numeric_code);
        ans_a += human_sequence.len() * numeric_code;
    }
    
    ans_a
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans_a = solve(input);
    assert_eq!(ans_a, 1393);
    let ans_b = solve(input);
    assert_eq!(ans_b, 990096);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a1() {
        let input = "029A";
        let ans = solve(input);
        println!("Human answer  : <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
        assert!(ans == 68*291);
    }
    
    #[test]
    fn example_a2() {
        let input = "980A";
        let ans = solve(input);
        assert!(ans == 60*980);
    }

    #[test]
    fn example_a3() {
        let input = "179A";
        let ans = solve(input);
        assert!(ans == 68*179);
    }

    #[test]
    fn example_a4() {
        let input = "456A";
        let ans = solve(input);
        assert!(ans == 64*456);
    }

    #[test]
    fn example_a5() {
        let input = "379A";
        let ans = solve(input);
        println!("Human answer  : <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
        assert!(ans == 64*379);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        let ans = solve(input);
        assert_eq!(ans, 0);
    }
}
