use std::collections::HashMap;



fn part_a(input: &str) -> u64 {
    let mut score = 1;

    let mut lines = input.trim().split("\r\n");
    let (_, time) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distance) = lines.next().unwrap().split_once(':').unwrap();
    let times: Vec<f64> = time.split_whitespace().map(|u| u.parse().unwrap()).collect();
    let distances: Vec<f64> = distance.split_whitespace().map(|u| u.parse().unwrap()).collect();

    for (i, time) in times.iter().enumerate() {
        let record = distances[i] + 1.0;
        // score = (time-x)*x > record
        // -x^2 + time*x - record > 0  
        //  0 > x^2 - time*x + record
        // (x-time/2)^2 - time^2/4 + record < 0
        // x = time/2  +/- sqrt((time/2)^2 - record) + 
        let upper = (time/2.0) + ((time*time/4.0 - record) as f64).sqrt(); 
        let lower = (time/2.0) - ((time*time/4.0 - record) as f64).sqrt(); 

        let nr_sols = upper.floor() as u64 - lower.ceil() as u64 + 1;
        score *= nr_sols as u64;
    }

    return score;
}



fn part_b(input: &str) -> u64 {
    let mut score = 1;

    let mut lines = input.trim().split("\r\n");
    let (_, time) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distance) = lines.next().unwrap().split_once(':').unwrap();
    let time: String = time.chars().filter(|&c| c.is_numeric()).collect();
    let distance: String = distance.chars().filter(|&c| c.is_numeric()).collect();
    let times: Vec<f64> = time.split_whitespace().map(|u| u.parse().unwrap()).collect();
    let distances: Vec<f64> = distance.split_whitespace().map(|u| u.parse().unwrap()).collect();

    for (i, time) in times.iter().enumerate() {
        let record = distances[i] + 1.0;
        let upper = (time/2.0) + ((time*time/4.0 - record) as f64).sqrt(); 
        let lower = (time/2.0) - ((time*time/4.0 - record) as f64).sqrt(); 

        let nr_sols = upper.floor() as u64 - lower.ceil() as u64 + 1;
        score *= nr_sols as u64;
    }
    return score;
}


fn main() {
    
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {:?}", ans_a);
    // 916 low
    
    let ans_b = part_b(input);
    println!("Part B: {:?}", ans_b);
    // 5923918
}
