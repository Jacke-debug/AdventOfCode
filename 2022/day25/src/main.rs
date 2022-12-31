
fn convert_to_snafu(number: i64) -> String {
    // thx JuniorBirdman1115, I ran out of time. 
    let mut ret: Vec<i8> = Default::default();

    let mut n = number;
    let mut borrow: i8 = 0;
    loop {
        let mut rem: i8 = (n % 5).try_into().unwrap();
        rem += borrow;
        n = n / 5;

        if rem >= 3 {
            rem -= 5;
            ret.push(rem);
            borrow = 1;
        } else {
            ret.push(rem);
            borrow = 0;
        }
        if n == 0 {
            break;
        }
    }
    ret.iter().rev().fold(String::new(), |mut acc, x| {
        acc.push(match x {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("unknown!"),
        });
        acc
    })
}
fn main() {
    let input = include_str!("input.txt");
 
    let mut score: i64 = 0;

    // go though each line, get corresponding decmial number
    for line in input.split("\r\n") {
        let line: Vec<char> = line.chars().rev().collect();
        for (nr, char) in line.iter().enumerate() {
            println!("{} {}", nr, char);
            match char {
                '2' => {
                    score += 2*5_i64.pow(nr as u32);
                },
                '1' => {
                    score += 5_i64.pow(nr as u32);
                }
                '0' => {
                }
                '-' => {
                    score -= 5_i64.pow(nr as u32);
                }
                '=' => {
                    score -= 2*5_i64.pow(nr as u32);
                }
                _ => panic!()
            }
        }
    }

    println!("Score {}", score);

    let ans = convert_to_snafu(score as i64);
    println!("ans: {}", ans);   // 2-2=21=0021=-02-1=-0
    
}
