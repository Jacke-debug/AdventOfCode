use std::collections::HashMap;



fn part_a(input: &str) -> i32 {
    let mut score = 0;

    for data in input.split("\r\n") {
        let (game_id, info) = data.split_once(':').unwrap();
        let (winner, numbers) = info.split_once(" | ").unwrap();
        let my_numbers: Vec<i32> = numbers.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let winners: Vec<i32> = winner.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let nr_winners: u32 = winners.iter().map(|s| my_numbers.contains(s) as u32).sum();

        if nr_winners > 0 {
            let points = (2 as i32).pow(nr_winners-1);
            println!("{}, {}",nr_winners,  points);
            score += points;
        }
        

    }
    return score;
}



fn part_b(input: &str) -> i32 {
    let mut score = 0;

    let mut num_cards: HashMap<usize, i32> = HashMap::new();
    let mut num_lines = 0;
    for (line_nr, data) in input.split("\r\n").enumerate() {
        let (game_id, info) = data.split_once(':').unwrap();
        num_cards.entry(line_nr)
        .and_modify(|existing_value| {
            *existing_value += 1;
        })
        .or_insert(1);

        let (winner, numbers) = info.split_once(" | ").unwrap();
        let my_numbers: Vec<i32> = numbers.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let winners: Vec<i32> = winner.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let nr_winners: u32 = winners.iter().map(|s| my_numbers.contains(s) as u32).sum();

        for idx in 1..=nr_winners {
            let num_copies = num_cards.get(&line_nr).unwrap().clone();
            num_cards.entry(line_nr+idx as usize)
                .and_modify(|existing_value| {
                    *existing_value += num_copies;
                })
                .or_insert(num_copies);
        }
        num_lines = line_nr
    }
    
    for (key, value) in num_cards.iter() {
        if key <= &num_lines {
            println!("{}: {:?}", key+1, value);
            score += value
        }
    }
    return score
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
