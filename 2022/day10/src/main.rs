use std::collections::HashMap;



fn part_a(input: &str) -> u32 {
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut cycle = 0;
    let mut value = 1;

    let cycles_to_check = vec![20, 60, 100, 140, 180, 220];
    let mut signal_strenghts = Vec::new();
    for line in data {
        cycle +=1;
        println!("{:?}: {}", cycle, value);
        if cycles_to_check.contains(&cycle){
            signal_strenghts.push(value)
        }
        if line == "noop" {
            continue
        }
        let (op, val) = line.split_once(" ").unwrap();
        let val: i32 = val.parse().unwrap();
        cycle += 1;
        println!("{:?}: {}", cycle, value);
        if cycles_to_check.contains(&cycle){
            signal_strenghts.push(value)
        }
        value += val;
    }
    println!("{:?}", signal_strenghts);
    for (idx, cycle)  in cycles_to_check.iter().enumerate() {
        score += signal_strenghts.get(idx).unwrap()*cycle;
    }
    //score = signal_strenghts.iter().sum();
    return score.try_into().unwrap();
}


fn part_b(input: &str) -> u32{
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    let mut cycle: i32 = 0;
    let mut value: i32 = 1;

    let mut image: Vec<char> = Vec::new();
    for line in data {
        cycle +=1;
        println!("{:?}: {}", cycle, value);
        if (cycle%40 - value).abs() < 2 {
            image.push('#')
        } else {
            image.push('.')
        }
        if line == "noop" {
            continue
        }
        let (op, val) = line.split_once(" ").unwrap();
        let val: i32 = val.parse().unwrap();
        cycle += 1;
        value += val;
        println!("{:?}: {}", cycle, value);
        if (cycle%40 - value).abs() < 2 {
            image.push('#')
        } else {
            image.push('.')
        }
    }
    for (idx, sign) in image.iter().enumerate() {
        if idx%40 == 0 {
            println!()
        }
        print!("{}", sign);
        
    }
    println!();
    //score = signal_strenghts.iter().sum();
    return score.try_into().unwrap();
}



fn main(){
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: u32 = 21;
    const EXAMPLE_B: u32 = 0;
    const SOLVE_A: u32 = 0;
    const SOLVE_B: u32 = 0;
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