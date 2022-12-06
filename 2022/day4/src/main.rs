
fn part_a(input: &str) -> u32 {
    let data = input.trim().split("\r\n");
    let mut score = 0;

    for line in data {
        let (first, second) = line.split_once(',').unwrap();
        let (start1, end1) = first.split_once('-').unwrap();
        let (start2, end2) = second.split_once('-').unwrap();

        let start1_idx: usize = start1.parse().unwrap();
        let start2_idx: usize = start2.parse().unwrap();
        let end1_idx: usize = end1.parse().unwrap();
        let end2_idx: usize = end2.parse().unwrap();

        let within1 = start1_idx <= start2_idx && end1_idx >= end2_idx;
        let within2 = start2_idx <= start1_idx && end2_idx >= end1_idx;
        if within1 || within2 {
            score += 1
        }
    }
    return score
}


fn part_b(input: &str) -> u32{
    let data = input.trim().split("\r\n");
    let mut score = 0;

    for line in data {
        let (first, second) = line.split_once(',').unwrap();
        let (start1, end1) = first.split_once('-').unwrap();
        let (start2, end2) = second.split_once('-').unwrap();

        let start1_idx: usize = start1.parse().unwrap();
        let start2_idx: usize = start2.parse().unwrap();
        let end1_idx: usize = end1.parse().unwrap();
        let end2_idx: usize = end2.parse().unwrap();

        let overlap1 = start2_idx <= end1_idx && end1_idx <= end2_idx;
        let overlap2 = start1_idx <= end2_idx && end2_idx <= end1_idx;

        if overlap1 || overlap2 {
            score += 1
        }
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
    const EXAMPLE_A: u32 = 2;
    const EXAMPLE_B: u32 = 4;
    const SOLVE_A: u32 = 483;
    const SOLVE_B: u32 = 874;
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