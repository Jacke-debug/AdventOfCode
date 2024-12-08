fn solve(rest: &str) -> usize {
    let mut rest = rest;
    let mut ans = 0;
    while let Some((_, rem)) = rest.split_once("mul(") {
        rest = rem;
        let (n1, rem) = match rem.split_once(",") {
            Some((num, rem)) => (
                match num.parse::<u64>() {
                    Ok(n) => n,
                    Err(_) => {
                        continue;
                    }
                },
                rem,
            ),
            None => {
                rest = rem;
                continue;
            }
        };
        rest = rem;
        let (n2, rem) = match rem.split_once(")") {
            Some((num, rem)) => (
                match num.parse::<u64>() {
                    Ok(n) => n,
                    Err(_) => {
                        continue;
                    }
                },
                rem,
            ),
            None => {
                rest = rem;
                continue;
            }
        };
        rest = rem;
        ans += (n1 * n2) as usize;
    }
    ans
}

fn part_a(input: &str) -> usize {
    let inp = input
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_numeric() || ['m', 'u', 'l', '(', ')', ','].contains(c))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let rest = &input[inp[0]..=*inp.last().unwrap()];
    solve(rest)
}

fn clean_string(mut input: String) -> String {
    while let (Some(start), Some(mut end)) = (input.find("don't()"), input.find("do()")) {
        while end < start {
            let new_end = input[end + 1..].find("do()").unwrap();
            end += new_end + 1;
        }
        input.replace_range(start..end + "do()".len(), "");
    }
    input
}

fn part_b(input: &str) -> usize {
    let input = clean_string(String::from(input));
    let inp = input
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_numeric() || ['m', 'u', 'l', '(', ')', ','].contains(c))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let rest = &input[inp[0]..=*inp.last().unwrap()];
    solve(rest)
}

fn main() {
    let input = include_str!("input.txt");
    let ans = part_a(input);
    assert_eq!(ans, 159833790);

    let ans = part_b(input);
    assert_eq!(ans, 89349241);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = include_str!("example.txt");
        let ans = part_a(input);
        assert_eq!(ans, 161);
    }

    #[test]
    fn example_b() {
        let input = include_str!("example_b.txt");
        let ans = part_b(input);
        assert_eq!(ans, 48);
    }
}
