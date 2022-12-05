use std::{collections::{HashMap, HashSet}, ops::Add, char, hash::Hash};

fn count(p1: char, p2: char, reactions: & HashMap<(char, char), char>, memo: &mut HashMap<(char, char, usize), HashMap<char, usize>>, iterations: usize) -> HashMap<char, usize> {
    if iterations == 0 {
        let mut counts = HashMap::new();
        *counts.entry(p1).or_default() += 1;
        *counts.entry(p2).or_default() += 1;
        return counts
    }
    if let Some(result) = memo.get(&(p1, p2, iterations)) {
        return result.clone();
    }
    if let Some(c) = reactions.get(&(p1, p2)) {
        let mut counts = HashMap::new();
        for (c, count) in count(p1, *c, reactions, memo, iterations-1) {
            *counts.entry(c).or_default() += count;
        }
        for (c, count) in count(*c, p2, reactions, memo, iterations-1) {
            *counts.entry(c).or_default() += count;
        }
        *counts.entry(*c).or_default() -=1;
        memo.insert((p1, p2, iterations), counts.clone());
        counts
    } else {
        let mut counts = HashMap::new();
        *counts.entry(p1).or_default() += 1;
        *counts.entry(p2).or_default() += 1;
        return counts
    }
}

fn main() {

    let input = include_str!("input.txt");
    let mut data = input.trim().split("\r\n");
    
    let mut rules = HashMap::new();
    let mut template: Vec<char> = data.next().unwrap().chars().collect();

    data.next();
    for line in data {
        let (key, c) = line.split_once(" -> ").unwrap();
        let mut pattern = key.chars();
        let p1 = pattern.next().unwrap();
        let p2 = pattern.next().unwrap();
        let c = c.chars().next().unwrap();
        rules.insert((p1, p2), c);
    }
    
    let mut memo: HashMap<(char,char, usize), HashMap<char, usize>> = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();
    for i in 1..template.len() {
        let p1 = template[i-1];
        let p2 = template[i];
        for (c, count) in count(p1, p2, &rules, &mut memo, 40) {
            *counts.entry(c).or_default() += count;
        }
        if i != 1 {
            *counts.entry(p1).or_default() -= 1;
        }
    }

    //  for step in 0..40 {
    //      println!("step: {}", step);
    //      let mut count = 0;
    //      let mut temp = template.clone();
    //      for idx in 0..template.len()-1 {
    //          let combo: &str = &template[idx].to_string().add(&template[idx+1].to_string());
    //          temp.insert(idx + 1 + count, *rules.get(combo).unwrap());
    //          count +=1;
    //      }
    //      template = temp;
    //}


    let max_count =  counts.values().max().map(|x| *x as i64).unwrap();
    let min_count = counts.values().min().map(|x| *x as i64).unwrap();
    println!("score: {}", max_count-min_count);
}
