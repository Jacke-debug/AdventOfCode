fn get_derivative(data: &[i64]) -> Vec<i64> {
    let mut derivative = Vec::new();
    for i in 1..data.len() {
        let diff = data[i] - data[i - 1];
        derivative.push(diff);
    }
    derivative
}

fn part_a(input: &str) -> (i64, i64) {
    let mut score_a = 0;
    let mut score_b = 0;
    let lines = input.trim().split("\r\n");
    
    for line in lines {
        let sequence: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut derivatives = get_derivative(&sequence);
        let mut last_elements = vec![*sequence.last().unwrap()];
        let mut first_elements= vec![*sequence.first().unwrap()];
        while !derivatives.iter().all(|x| *x == 0) {
            last_elements.push(*derivatives.last().unwrap());
            first_elements.push(*derivatives.first().unwrap());
            derivatives = get_derivative(&derivatives);
        }
        score_a += last_elements.iter().sum::<i64>();
        score_b += first_elements.iter().rev().fold(0, |acc, &elem| elem - acc);
    }
    return (score_a, score_b);
}


fn main() {
    let input = include_str!("input.txt");
    let (ans_a, ans_b) = part_a(input);
    println!("Part A: {:?}", ans_a);
    println!("Part B: {:?}", ans_b);
}
