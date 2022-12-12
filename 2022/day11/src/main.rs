use std::collections::{HashMap, VecDeque};

trait Throw {
    fn throw(&self, worry_level: i64) -> (i64, usize);
    fn throw_b(&self, worry_level: i64) -> (i64, usize);
}

#[derive(Debug)]
struct Monkey<'a> {
    items: VecDeque<i64>,
    inspected_items: i64,
    operation: &'a str,
    operand: i64,
    numerator: i64,
    target_true: usize,
    target_false: usize, 
}

impl Throw for Monkey<'_> {
   fn throw(&self, mut worry_level: i64) -> (i64, usize) {
    match self.operation {
        "self" => worry_level *= worry_level,
        "*" => worry_level *= self.operand,
        "+" => worry_level += self.operand,
        _ => panic!(),
    }
    worry_level /= 3;
    if &worry_level % self.numerator == 0 {
        return (worry_level, self.target_true)
    }
    return (worry_level, self.target_false)
   }

   fn throw_b(&self, mut worry_level: i64) -> (i64, usize) {
    let max = 2*3*5*7*11*13*17*19*23;

    match self.operation {
        "self" => worry_level *= worry_level,
        "*" => worry_level *= self.operand,
        "+" => worry_level += self.operand,
        _ => panic!(),
    }
    worry_level = worry_level%max;
    if &worry_level % self.numerator == 0 {
        return (worry_level, self.target_true)
    }
    return (worry_level, self.target_false)
   }
}

fn multi_largest(monkeys: Vec<Monkey>) -> usize {
    let mut inspected_arr = VecDeque::new();
    for monkey in monkeys {
        inspected_arr.push_back(monkey.inspected_items);
        println!("Inspected items {}", monkey.inspected_items);
    }
    let max = *inspected_arr.iter().max().unwrap();
    let max_idx = inspected_arr.iter().position(|x| *x == max).unwrap();
    inspected_arr.remove(max_idx);
    inspected_arr.iter().max();
    let score = max * inspected_arr.iter().max().unwrap();
    return score.try_into().unwrap();
}

fn part_a(input: &str) -> usize {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;
    let rounds = 20;

    let mut monkeys: Vec<Monkey> = Vec::new();

    while let Some(line) = data.next() {
        let mut line = data.next().unwrap();
        let (_, items_array) = line.split_once(": ").unwrap();
        let starting_items: VecDeque<i64> = items_array.trim().split(", ").map(|word| word.parse::<i64>().unwrap()).collect();
        
        line = data.next().unwrap();
        let (_, split) = line.split_once("old ").unwrap();
        let (mut operation, operand_str) = split.split_once(" ").unwrap();
        let mut operand: i64;
        if operand_str == "old" {
            operation = "self";
            operand = 1;
        } else {
            operand = operand_str.parse().unwrap();
        }

        line = data.next().unwrap();
        let split = line.split_ascii_whitespace();
        let numerator: i64 = split.last().unwrap().parse().unwrap();

        line = data.next().unwrap();
        let split = line.split_ascii_whitespace();
        let target_true: usize = split.last().unwrap().parse().unwrap();

        line = data.next().unwrap();
        let split = line.split_ascii_whitespace();
        let target_false: usize = split.last().unwrap().parse().unwrap();

        let monkey = Monkey {
            items: starting_items,
            inspected_items: 0,
            operation,
            operand,
            numerator,
            target_true,
            target_false,
        };
        monkeys.push(monkey);
        data.next(); // empty line between monkeys
    }

    let mut items_to_add: HashMap<usize, VecDeque<i64>> = HashMap::new();
    for i in 0..monkeys.len() {
        items_to_add.insert(i, VecDeque::new());
    }

    for round in 0..rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            let items_ = items_to_add.get_mut(&idx).unwrap();
            monkey.items.append(items_);
            while let Some(item) = monkey.items.pop_front() {
                monkey.inspected_items += 1;
                let (item, reciever) = monkey.throw(item);
                items_to_add.get_mut(&reciever).unwrap().push_back(item);
            }
        }
    }
    score = multi_largest(monkeys);
    return score
}


fn part_b(input: &str) -> usize{
    let mut data = input.trim().split("\r\n");
    let mut score = 0;
    let rounds = 10000;

    let mut monkeys: Vec<Monkey> = Vec::new();

    while let Some(line) = data.next() {

        let mut line = data.next().unwrap();
        let (_, items_array) = line.split_once(": ").unwrap();
        let starting_items: VecDeque<i64> = items_array.trim().split(", ").map(|word| word.parse::<i64>().unwrap()).collect();
        
        line = data.next().unwrap();
        let (_, split) = line.split_once("old ").unwrap();
        let (mut operation, operand_str) = split.split_once(" ").unwrap();
        let mut operand: i64;
        if operand_str == "old" {
            operation = "self";
            operand = 1;
        } else {
            operand = operand_str.parse().unwrap();
        }

        line = data.next().unwrap();
        let split = line.split_ascii_whitespace();
        let numerator: i64 = split.last().unwrap().parse().unwrap();

        line = data.next().unwrap();
        let split = line.split_ascii_whitespace();
        let target_true: usize = split.last().unwrap().parse().unwrap();

        line = data.next().unwrap();
        let split = line.split_ascii_whitespace();
        let target_false: usize = split.last().unwrap().parse().unwrap();

        let monkey = Monkey {
            items: starting_items,
            inspected_items: 0,
            operation,
            operand,
            numerator,
            target_true,
            target_false,
        };
        monkeys.push(monkey);
        data.next(); // empty line between monkeys
    }

    let mut items_to_add: HashMap<usize, VecDeque<i64>> = HashMap::new();
    for i in 0..monkeys.len() {
        items_to_add.insert(i, VecDeque::new());
    }

    for round in 0..rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            let items_ = items_to_add.get_mut(&idx).unwrap();
            monkey.items.append(items_);
            while let Some(item) = monkey.items.pop_front() {
                monkey.inspected_items += 1;
                let (item, reciever) = monkey.throw_b(item);
                items_to_add.get_mut(&reciever).unwrap().push_back(item);
            }
        }
    }

    score = multi_largest(monkeys);
    return score;
}



fn main(){
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    println!();
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: usize = 10605;
    const SOLVE_A: usize = 69918;

    const EXAMPLE_B: usize = 2713310158;
    const SOLVE_B: usize = 19573408701;

    use super::*;
    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), EXAMPLE_A);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), EXAMPLE_B);
    }

    #[test]
    fn solve_a() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), SOLVE_A);
    }

    #[test]
    fn solve_b() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), SOLVE_B);
    }
}