use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn a() {
    let file = File::open("input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut num_lines = 0;
    let mut count = vec![0; 12];
    for line in reader.lines() {
        num_lines += 1;
        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                count[i] += 1;
            }
        }
    }

    let mut num = vec![false; 12];
    num = count.iter().map(|x| (x-num_lines/2) > 0).collect();

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    println!("{:?}", num);
    for (idx, n) in num.iter().enumerate() {
        println!("idx {}, n {}", idx, n);
        if *n {
            gamma_rate += 2_i32.pow(((11-idx)).try_into().unwrap());
            println!("gamma_rate: {}", gamma_rate);
        } else {
            epsilon_rate += 2_i32.pow(((11-idx)).try_into().unwrap());
            println!("epsilon_rate: {}", epsilon_rate);
        }
    }
    
    println!("{}, {}, {}", gamma_rate, epsilon_rate, gamma_rate*epsilon_rate);

}
    


fn main() {
    let file = File::open("input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut my_matrix: Vec<Vec<char>> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let chars: Vec<_> = line.chars().collect();
        my_matrix.push(chars);
    }
    
    
    for j in 0..12 {
        let mut idx1 = Vec::new();
        let mut count1 = 0;
        for (i, row) in my_matrix.iter().enumerate() {
            if row[j] == '1' {
                idx1.push(true);
                count1 +=1;
            } else {
                idx1.push(false);
            }
        }

        if count1 >= my_matrix.len()-count1 {
            println!("a.len(): {:?}", my_matrix.len());
        } else {
            println!("b.len(): {:?}", my_matrix.len());
            idx1 = idx1.iter().map(|x| !x).collect();
        }

        let mut new_matrix: Vec<Vec<char>> = Vec::new();
        for (i, row) in my_matrix.iter().enumerate() {
            if idx1[i] {   // swap here
                new_matrix.push(row.to_vec());
            }
        }

        //println!("new_matrix: {} iter  {:?}", j, new_matrix); 
        if new_matrix.len().eq(&1) {
            println!("done: {:?}", new_matrix); 
            my_matrix = new_matrix;
            break
        } else {
            my_matrix = new_matrix;
        }
        
    }

    println!("end{:?}", my_matrix);
    // a = 100110010111 2455
    // b = 10101101111 1391
    // a*b = 3414905
}
    