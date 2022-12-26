use std::collections::HashMap;


fn solve(input: &str, decryption_key: isize, rounds: usize) -> isize {

    let mut sequence: Vec<(usize, isize)> = input.lines()
        .map(|s| s.parse::<isize>().unwrap()*decryption_key)
        .enumerate().collect();

    for _ in 0..rounds {
        for move_nr in 0..sequence.len() {
            let index = sequence.iter().position(|x| x.0 == move_nr).unwrap();

            let value = sequence[index].1; // get value of this entry

            let new_index = index as isize +value;
            let new_index = new_index.rem_euclid(sequence.len() as isize -1);

            let tmp = sequence.remove(index);
            sequence.insert(new_index as usize, tmp);
        }
    } 
    let pos_zero = sequence.iter().position(|x| x.1 == 0).unwrap();
    let x1 = sequence[(pos_zero+ 1_000) % sequence.len()].1;
    let x2 = sequence[(pos_zero+ 2_000) % sequence.len()].1;
    let x3 = sequence[(pos_zero+ 3_000) % sequence.len()].1;
    return x1+x2+x3
}

fn main() {
    let input = include_str!("input.txt");
    let score_a = solve(input, 1, 1);
    println!("Score A {}", score_a);
    let score_b = solve(input, 811589153, 10);
    println!("Score B {}", score_b);
}
