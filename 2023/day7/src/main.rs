use std::collections::{BTreeSet, HashSet, HashMap};




#[derive(Debug, PartialEq, Eq)]
struct Hand {
    // Your type fields go here
    cards: Vec<char>,
    bet: usize,
}

fn ch_to_num_a(ch: &char) -> u8 {
    if *ch == 'A' {
        return 14;
    } else if *ch == 'K' {
        return 13;
    } else if *ch == 'Q' {
        return 12;
    } else if *ch == 'J' {
        return 11;
    } else if *ch == 'T' {
        return 10;
    } else {
        return ch.to_digit(10).unwrap().try_into().unwrap();
    }
}

fn ch_to_num(ch: &char) -> u8 {
    if *ch == 'A' {
        return 14;
    } else if *ch == 'K' {
        return 13;
    } else if *ch == 'Q' {
        return 12;
    } else if *ch == 'J' {
        return 1;   // now worth the least
    } else if *ch == 'T' {
        return 10;
    } else {
        return ch.to_digit(10).unwrap().try_into().unwrap();
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Implement your comparison logic here
        if evaluate_type(self) != evaluate_type(other) {
            return evaluate_type(self).cmp(&evaluate_type(other))
        }
        for (idx, card) in self.cards.iter().enumerate() {
            let self_val = ch_to_num(card);
            let other_val = ch_to_num(other.cards.get(idx).unwrap());
            if self_val != other_val {
                return self_val.cmp(&other_val);
            }
        }
        panic!()
    }
}




fn evaluate_type(hand: &Hand) -> u8{
    println!("hand {:?}", hand);
    let mut cards = hand.cards.clone();
    cards.retain(|&item| item != 'J');
    let most_common = cards
        .iter()
        .fold(HashMap::new(), |mut counts, &c| {
            *counts.entry(c).or_insert(0) += 1;
            counts
        })
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(character, _)| character);

    let num_remain: usize = cards.len();
    for i in 0..5-num_remain {
        cards.push(match most_common {
            Some(c) => c,
            None => match cards.get(0) {
                Some(a) => *a,
                None => 'K', 
            },
        })
    }

    let mut set: HashSet<_> = cards.clone().into_iter().collect();
    let nr_unique: usize = set.len();
    if nr_unique == 1 {
        return 6; // 6: 5 of a kind
    } else if nr_unique == 5 {
        return 0; // 0: high card
    } else if nr_unique == 4 {
        return 1; // 1: one pair
    } else if nr_unique == 2 {
        let mut set_iter = set.iter();
        let first = set_iter.next().unwrap(); 
        let nr_first = cards.iter().filter(|&c| c == first).count();
        if nr_first == 1 || nr_first == 4 {
            return 5; // 5: 4 of a kind, 2
        } else {
            return 4; // 4: full-house, 2
        }
    } else if nr_unique == 3 {
        let mut set_iter = set.iter();
        let first = set_iter.next().unwrap(); 
        let second = set_iter.next().unwrap(); 
        let nr_first = cards.iter().filter(|&c| c == first).count();
        let nr_second = cards.iter().filter(|&c| c == second).count();
        if nr_first == 2 || nr_second== 2 {
            return 2;  // 2: two pair, 3
        } else {
            return 3;  // 3: 3 of a kind, 3
        }
    } else {
        panic!()
    }
}

fn evaluate_type_a(hand: &Hand) -> u8{
    let mut cards = hand.cards.clone();
    let mut set: HashSet<_> = hand.cards.clone().into_iter().collect();
    let nr_unique: usize = set.len();
    if nr_unique == 1 {
        return 6; // 6: 5 of a kind
    } else if nr_unique == 5 {
        return 0; // 0: high card
    } else if nr_unique == 4 {
        return 1; // 1: one pair
    } else if nr_unique == 2 {
        let mut set_iter = set.iter();
        let first = set_iter.next().unwrap(); 
        let nr_first = cards.iter().filter(|&c| c == first).count();
        if nr_first == 1 || nr_first == 4 {
            return 5; // 5: 4 of a kind, 2
        } else {
            return 4; // 4: full-house, 2
        }
    } else if nr_unique == 3 {
        let mut set_iter = set.iter();
        let first = set_iter.next().unwrap(); 
        let second = set_iter.next().unwrap(); 
        let nr_first = cards.iter().filter(|&c| c == first).count();
        let nr_second = cards.iter().filter(|&c| c == second).count();
        if nr_first == 2 || nr_second== 2 {
            return 2;  // 2: two pair, 3
        } else {
            return 3;  // 3: 3 of a kind, 3
        }
    } else {
        panic!()
    }
}

fn part_a(input: &str) -> usize {
    let mut score = 0;

    let mut all_hands = Vec::new();
    for (line_nr, data) in input.trim().split("\r\n").enumerate() {
        let (cards, bet) = data.split_once(' ').unwrap();
        let hand: Hand = Hand {
            cards: cards.chars().collect(),
            bet: bet.parse::<usize>().unwrap(),
        };
        println!("{:?} {}", hand, evaluate_type(&hand));
        all_hands.push(hand);
    }

    all_hands.sort();

    for (idx, hand) in all_hands.into_iter().enumerate() {
        score += hand.bet * (idx+1);
        
    }
    
    return score;
}



fn part_b(input: &str) -> u64 {
    let mut score = 1;

    return score;
}


fn main() {
    
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    println!("Part A: {:?}", ans_a);
    // 248131423 low, 248179786
    
    let ans_b = part_b(input);
    println!("Part B: {:?}", ans_b);
    // 248800039 high, 
}
