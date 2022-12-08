use std::collections::HashSet;

fn part_a(input: &str) -> u32 {
    let mut data = input.trim().chars();
    let mut score = 0;
    let num_charac = 4;

    let mut marker = Vec::new();
    for _ in 0..num_charac {
        marker.push(data.next().unwrap());
        score += 1;
    }

    while let Some(char) = data.next() {
        let mut marker_unique: HashSet<char> = HashSet::from_iter(marker.iter().cloned());
        if marker_unique.len() == num_charac {
            break
        } 
        marker.remove(0);
        marker.push(char);
        score += 1;
        
        //println!("{:?}", marker);
    }
    return score
}


fn part_b(input: &str) -> u32{
    let mut data = input.trim().chars();
    let mut score = 0;
    let num_charac = 14;

    let mut marker = Vec::new();
    for _ in 0..num_charac {
        marker.push(data.next().unwrap());
        score += 1;
    }

    while let Some(char) = data.next() {
        let mut marker_unique: HashSet<char> = HashSet::from_iter(marker.iter().cloned());
        if marker_unique.len() == num_charac {
            break
        } 
        marker.remove(0);
        marker.push(char);
        score += 1;
        
        //println!("{:?}", marker);
    }
    return score
}



fn main(){
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: u32 = 0;
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