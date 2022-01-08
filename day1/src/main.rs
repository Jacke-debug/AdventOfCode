use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut count = 0;
    for i in 3..numbers.len() {
        // let mid_sum = numbers[i-1]+numbers[i-2]
        if  numbers[i] > numbers[i-3] {
            count+=1;
            
        }
    }

    println!("{}", count)
}