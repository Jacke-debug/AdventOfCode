fn part_a() -> Vec<i32>{
    let input = include_str!("input_a.txt");

    let mut data = input.trim().split("\r\n");

    let mut suppli_count: Vec<i32> = Vec::new();
    let mut count = 0;
    for line in data {
        if line.is_empty() {
            suppli_count.push(count);
            count = 0;
            continue;
        }

        let num:i32 = line.parse().unwrap();
        count += num;
    }
    suppli_count.push(count);
    println!("Max {:?}", suppli_count.iter().max());
    return suppli_count
}


fn part_b() {
    let mut calories = part_a();
    let mut sum_of_n_largest = 0;
    let n = 3;
    for i in 0..n { 
        let max = calories.iter().max().unwrap();
        let idx = calories.iter().position(|elem| elem == max).unwrap();
        sum_of_n_largest += max;
        print!("{}\n", idx);
        calories.remove(idx);
        
    }
    println!("Sum of {} largest: {:?}", n, sum_of_n_largest);
}


fn main(){
    part_b()
}