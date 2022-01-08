use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::num::ParseIntError;


fn main() {
    let file = File::open("input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (line) in reader.lines() {
        let line = line.unwrap(); // Ignore errors.

        let mut iter = line.split_whitespace();
        let direction = iter.next().unwrap();
        let distance = iter.next().unwrap().parse::<i32>().unwrap();
        if direction.eq("up") {
            //depth -= distance;
            aim -= distance;
        } else if direction.eq("down") {
            //depth += distance;
            aim += distance;
        } else {
            horizontal += distance;
            depth += aim*distance;
        }

        
        //let items: Result<Vec<i32>, _> = parts.collect();
    }

    println!("{}, {}, {}", depth, horizontal, depth*horizontal);
    
}
