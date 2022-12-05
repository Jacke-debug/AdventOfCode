use core::num;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn find_key_for_value<'a>(map: &'a HashMap<&str, HashSet<char>>, value: &'a HashSet<char>) -> Option<&'a str> {
    map.iter()
        .find_map(|(&key, val)| if val == value { Some(key) } else { None })
}

fn compare_sets(ref_set: & HashSet<char>, test_set: & HashSet<char>) -> bool {
    for c in ref_set {
        if !test_set.contains(&*c) {
            return  false;
        }
    }
    return true;
}

fn main() {
    let input = include_str!("input.txt");
    let mut number_count: i64 = 0;

    for line in input.trim().split("\r\n") {
    let (schiffer, code) = line.split_once(" | ").unwrap();
        let mut decoder: HashMap<&str, HashSet<char>> = HashMap::new();

        // the obvious numbers
        for str in schiffer.split_whitespace() {
            let mut my_set = HashSet::new();
            for c in str.chars() {
                my_set.insert(c);
            }
            if str.len() == 2 {
                decoder.insert("1", my_set);
            } else if str.len() == 3 {
                decoder.insert("7", my_set);
            } else if str.len() == 4 {
                decoder.insert("4", my_set);
            } else if str.len() == 7 {
                decoder.insert("8", my_set);
            }
        }

        // zero, six and nine
        for str in schiffer.split_whitespace() {
            let mut test_set = HashSet::new();
            for c in str.chars() {
                test_set.insert(c);
            }
    
            if str.len() == 6 {
                if compare_sets(decoder.get("1").unwrap(), & test_set) {
                    if compare_sets(decoder.get("4").unwrap(), & test_set) {
                        decoder.insert("9", test_set);
                    } else {
                        decoder.insert("0", test_set);
                    }
                    
                } else {
                    decoder.insert("6", test_set);
                }
            }
        }
        
        // two, five and three
        for str in schiffer.split_whitespace() {
            let mut test_set = HashSet::new();
            for c in str.chars() {
                test_set.insert(c);
            }
            if str.len() == 5 {
                if compare_sets(decoder.get("1").unwrap(), & test_set) {
                    decoder.insert("3", test_set); 
                } else {
                    if compare_sets(& test_set, decoder.get("9").unwrap()) {
                        decoder.insert("5", test_set);
                    } else {
                        decoder.insert("2", test_set);
                    }
                }
            }
        }

        let mut my_str: String = "".to_owned();
        for data in code.split_whitespace() {
            let mut test_set = HashSet::new();
            for c in data.chars() {
                test_set.insert(c);
            }
            if let Some(x) = find_key_for_value(&decoder, & test_set) {
                my_str = my_str + x;
            }
        }
        number_count += my_str.parse::<i64>().unwrap();
        println!("{}", my_str);
    }
    println!("number count: {}", number_count);
}
