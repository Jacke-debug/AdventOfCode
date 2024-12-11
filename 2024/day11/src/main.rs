use std::time::Instant;

fn solve(input: &str, iters: usize) -> usize {
    let mut stones: Vec<usize> = input.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    for i in 0..iters {
        let mut to_add = Vec::new();
        println!("{i}");
        println!("{:?}", stones);
        for (i, stone) in stones.iter_mut().enumerate() {
            match *stone {
                0 => {
                    *stone = 1;
                },
                n => {
                    let n_digits = n.to_string().len();
                    if n_digits % 2 == 0{
                        let a: usize = n.to_string()[0..n_digits/2].parse().unwrap();
                        let b: usize = n.to_string()[n_digits/2..n_digits].parse().unwrap();
                        *stone = a;
                        to_add.push((i+1, b));
                    } else {
                        *stone = *stone*2024;
                    }
                }
            }
        }
        let mut extra = 0;
        for (t, a) in &to_add {
            stones.insert(t+extra, *a);
            extra+=1;
        }
    }
    stones.len()
}

fn main() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ans = solve(input, 25);
    assert_eq!(ans, 183248);

    let ans = solve(input, 75);
    assert_eq!(ans, 1182);

    println!("Time: {} ms", start.elapsed().as_millis());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a_1() {
        let input = include_str!("example.txt");
        let ans = solve(input, 10);
        assert_eq!(ans, 22);
    }

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = solve(input, 25);
        assert_eq!(ans, 55312);
    }
}
