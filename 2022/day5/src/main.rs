

fn main() {
    let input = include_str!("input.txt");
    let mut data = input.split("\r\n");
    let mut score = 0;


    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut done = false;
    let mut line = data.next().unwrap();
    println!("{:?}", line);
    let nr_of_rows = (line.len()+1)/4;
    for i in 0..nr_of_rows{
        println!("{:?}", nr_of_rows);
        stacks.push(Vec::new())
    }

    while !done {
        for i in 0..nr_of_rows {
            println!("{}, {}", i*4, (i+1)*3);
            let content: Vec<char> = line[i*4..=(i+1)*4-2].chars().collect();
            println!("{:?}", content);
            let container: char = content[1].into();
            if container != ' ' {
                stacks[i].push(container);
            }
            println!("i= {}: {:?}", i, container);
        }

        line = data.next().unwrap();
        if line.is_empty() {
            for mut stack in &mut stacks {
                stack.pop();
                stack.reverse();
            }
            done = true;
            println!("{:?}", stacks);
        }
    }
    while let Some(line) = data.next() {
        let mut chars: Vec<char> = line.chars().collect();
        let mut parts = line.split_whitespace();
        
        parts.next();
        let amount: u32 = parts.next().unwrap().parse().unwrap();
        parts.next();
        let from: usize = parts.next().unwrap().parse().unwrap();
        let from = from - 1;
        parts.next();
        let to: usize = parts.next().unwrap().parse().unwrap();
        let to = to-1;

        println!("Moving amount: {:?} from {:?}, to {:?}", amount, from, to);

        let mut containers_to_move: Vec<char> = Vec::new();
        for _ in 0..amount {
            containers_to_move.push(stacks[from].pop().unwrap());
        }
        containers_to_move.reverse();
        stacks[to].append(&mut containers_to_move)
    }

    let mut code: String = String::from("");
    for i in 0..nr_of_rows {
        code.push(*stacks[i].last().unwrap())
    }
    println!("{:?}", code);
    println!("Score: {:?}", score);
}



fn part_a() {
    let input = include_str!("input.txt");
    let mut data = input.split("\r\n");
    let mut score = 0;


    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut done = false;
    let mut line = data.next().unwrap();
    println!("{:?}", line);
    let nr_of_rows = (line.len()+1)/4;
    for i in 0..nr_of_rows{
        println!("{:?}", nr_of_rows);
        stacks.push(Vec::new())
    }

    while !done {
        for i in 0..nr_of_rows {
            println!("{}, {}", i*4, (i+1)*3);
            let content: Vec<char> = line[i*4..=(i+1)*4-2].chars().collect();
            println!("{:?}", content);
            let container: char = content[1].into();
            if container != ' ' {
                stacks[i].push(container);
            }
            println!("i= {}: {:?}", i, container);
        }

        line = data.next().unwrap();
        if line.is_empty() {
            for mut stack in &mut stacks {
                stack.pop();
                stack.reverse();
            }
            done = true;
            println!("{:?}", stacks);
        }
    }
    while let Some(line) = data.next() {
        let mut chars: Vec<char> = line.chars().collect();
        let mut parts = line.split_whitespace();
        
        parts.next();
        let amount: u32 = parts.next().unwrap().parse().unwrap();
        parts.next();
        let from: usize = parts.next().unwrap().parse().unwrap();
        let from = from - 1;
        parts.next();
        let to: usize = parts.next().unwrap().parse().unwrap();
        let to = to-1;

        println!("Moving amount: {:?} from {:?}, to {:?}", amount, from, to);

        for _ in 0..amount {
            let container = stacks[from].pop().unwrap();
            stacks[to].push(container)
        }
        
    }

    let mut code: String = String::from("");
    for i in 0..nr_of_rows {
        code.push(*stacks[i].last().unwrap())
    }
    println!("{:?}", code);
    println!("Score: {:?}", score);
}
