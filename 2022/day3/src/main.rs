
fn convert_to_value(letter: char) -> u32 {
    let is_lower = letter.is_lowercase();
    let num: u32 = letter.into();
    
    let mut m: u32 = 'a'.into();    // 97 
    if !is_lower {
        m = 'A'.into();
        m = m - 26;
    } 
    let score: u32 = (num - m) + 1;
    println!("Letter {}, number {:?}", letter, score);
    return score
}

fn part_a() {
    let mut input = include_str!("input.txt");
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    for line in data {
        let chars: Vec<char> = line.chars().collect();
        let length = line.len();
        let first_compartment: Vec<char> = chars[0..length/2].to_vec();
        let second_compartment: Vec<char> = chars[length/2..length].to_vec();
        //println!("{:?}", first_compartment);
        //println!("{:?}", second_compartment);
        
        for letter in first_compartment {
            if second_compartment.contains(&letter) {
                score +=convert_to_value(letter);
                break
            }
        }
    }
    println!("Score: {}", score);
}


fn part_b() {
    let mut input = include_str!("input.txt");
    let mut data = input.trim().split("\r\n");
    let mut score = 0;

    while let Some(first_elf) = data.next() {
        let first_elf: Vec<char> = first_elf.chars().collect();
        let second_elf: Vec<char> = data.next().unwrap().chars().collect();
        let third_elf: Vec<char> = data.next().unwrap().chars().collect();

        for letter in first_elf {
            if second_elf.contains(&letter) && third_elf.contains(&letter) {
                score +=convert_to_value(letter);
                break
            }
        }
    }
    println!("Score: {}", score);
}

fn main() {
    part_b()
}