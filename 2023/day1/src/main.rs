
fn part_a(input: &str) -> i32 {
    let mut result = 0;
    for (y, line) in input.trim().split("\r\n").enumerate() {
        let mut res = String::new();

        for (x, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                res.push(ch);
                break
            }
        }
        for ch in line.chars().rev() {
            if ch.is_numeric() {
                res.push(ch);
                break
            }
        }
        result = result + res.parse::<i32>().unwrap();
    }
    
    return result
}


fn check_word(string: &str) -> Option<char> {
    if string.contains("one") {
        return  Some('1')
    } else if string.contains("two") {
        return  Some('2')
    }else if string.contains("three") {
        return  Some('3')
    }else if string.contains("four") {
        return  Some('4')
    }else if string.contains("five") {
        return  Some('5')
    }else if string.contains("six") {
        return  Some('6')
    }else if string.contains("seven") {
        return  Some('7')
    }else if string.contains("eight") {
        return  Some('8')
    }else if string.contains("nine") {
        return  Some('9')
    } else {
        return None
    }
}
fn part_b(input: &str) -> i32 {
    let mut result = 0;

    for line in input.trim().split("\r\n") {
        
        let mut res = String::new();
        let mut string = String::new();

        for ch in line.chars() {
            if ch.is_numeric() {
                res.push(ch);
                break
            }
            string.push(ch);
            match check_word(&string) {
                Some(ch) => {
                    res.push(ch);
                    break
                }
                None => continue,
            }
        }
        let mut string = String::new();
        for ch in line.chars().rev() {
            if ch.is_numeric() {
                res.push(ch);
                break
            }
            string.insert(0, ch);
            match check_word(&string) {
                Some(ch) => {
                    res.push(ch);
                    break
                }
                None => continue,
            }
        }
        result = result + res.parse::<i32>().unwrap();
    }
    
    return result
}

fn main() {
    
    let input = include_str!("input.txt");
    // let ans_a = part_a(input);
    // println!("Part A: {:?}", ans_a);
    
    let ans_b = part_b(input);
    println!("Part B: {:?}", ans_b);
}
