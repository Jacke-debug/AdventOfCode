use std::collections::HashMap;





fn part_a(input: &str) -> u32 {
    let mut data = input.split("\r\n");
    let mut score = 0;

    let mut file_strucutre: HashMap<String, u32> = HashMap::new();
    
    data.next();    // skip first line
    let mut path = vec!["/"];
    file_strucutre.insert("/".to_string(), 0);

    for line in data {
        let mut parts = line.split_ascii_whitespace();
        let first = parts.next().unwrap();

        if first.contains(&"$") {
            let command = parts.next().unwrap();
            if command == "ls" {
                continue
            } else if command == "cd" {
                let dir_name = parts.next().unwrap();

                if dir_name == ".." {
                    path.pop();
                    // go out
                } else {
                    path.push(dir_name);
                    let this_path = path.join("/");
                    file_strucutre.insert(this_path, 0);
                }
            }
        } else if first == "dir" {
            continue
        } else {
            let file_size: u32 = first.parse().unwrap();
            let file_name = parts.next().unwrap();
            let this_path = path.join("/");
            for (key, value) in file_strucutre.iter_mut() {
                if this_path.contains(key) {
                    *value += file_size;
                }
            }
            
        }
        
    }
    //println!("{:?}", file_strucutre);
    for (_, val) in file_strucutre.iter() {
        if val <= &100000 {
            score += val;
        }
    }
    return score
}


fn part_b(input: &str) -> u32{
    let mut data = input.split("\r\n");
    let mut score = 0;
    let total_size = 70000000;
    let required_space = 30000000;

    let mut file_strucutre: HashMap<String, i32> = HashMap::new();
    
    data.next();    // skip first line
    let mut path = vec!["/"];
    file_strucutre.insert("/".to_string(), 0);

    for line in data {
        let mut parts = line.split_ascii_whitespace();
        let first = parts.next().unwrap();

        if first.contains(&"$") {
            let command = parts.next().unwrap();
            if command == "ls" {
                continue
            } else if command == "cd" {
                let dir_name = parts.next().unwrap();

                if dir_name == ".." {
                    path.pop();
                    // go out
                } else {
                    path.push(dir_name);
                    let this_path = path.join("/");
                    file_strucutre.insert(this_path, 0);
                }
            }
        } else if first == "dir" {
            continue
        } else {
            let file_size: i32 = first.parse().unwrap();
            let file_name = parts.next().unwrap();
            let this_path = path.join("/");
            for (key, value) in file_strucutre.iter_mut() {
                if this_path.contains(key) {
                    *value += file_size;
                }
            }
            
        }
        
    }

    let size_to_delete = required_space - (total_size - file_strucutre.get("/").unwrap());
    println!("size_to_delete: {:?}", size_to_delete);
    let mut smallest_diff = total_size;
    for (_, val) in file_strucutre.iter() {
        if val > &size_to_delete && (val - size_to_delete) <  smallest_diff{
            smallest_diff = val - size_to_delete;
            score = *val;        
        }
    }
    return score.try_into().unwrap()
}



fn main(){
    let input = include_str!("input.txt");
    let score_a = part_a(input);
    let score_b = part_b(input);
    println!("Score A: {} \nScore B: {}", score_a, score_b);
}

#[cfg(test)]
mod tests {
    const EXAMPLE_A: u32 = 95437;
    const SOLVE_A: u32 = 1792222;
    const EXAMPLE_B: u32 = 24933642;
    const SOLVE_B: u32 = 1112963;
    use super::*;
    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), EXAMPLE_A);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), EXAMPLE_B);
    }

    #[test]
    fn solve_a() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), SOLVE_A);
    }

    #[test]
    fn solve_b() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), SOLVE_B);
    }
}