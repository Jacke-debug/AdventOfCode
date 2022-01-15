use std::{collections::HashMap};

fn parse_image(image: HashMap<(i32, i32), char>, decoder: &HashMap<isize, char>) -> HashMap<(i32, i32), char>{
    let mut new_map = HashMap::new();
    
    let (min_x, max_x, min_y, max_y) = size(&image);
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            let mut bin_num = String::new();
            //println!("{:?}", image.get(&(i, j)));
            for x in i-1..=i+1 {
                for y in j-1..=j+1 {
                    if let Some(c) = image.get(&(x, y)) {
                        if *c == '#' {
                            bin_num.push('1');
                        } else  {
                            bin_num.push('0');
                        }
                    } else {
                        bin_num.push('0');
                    }
                }
            }
            let intval = isize::from_str_radix(&bin_num, 2).unwrap();
            let new_symbol = decoder.get(&intval).unwrap();
            new_map.insert((i, j), *new_symbol);
        }
    }
    new_map
}

fn size(image: & HashMap<(i32, i32), char>) -> (i32, i32, i32, i32) {
    let mut max_x: i32 = 0;
    let mut max_y:i32 = 0;
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    for (x, y) in image.keys() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    (min_x, max_x, min_y, max_y)
}

fn main() {
    let input = include_str!("input.txt");

    let mut image = HashMap::new();
    let mut decoder = HashMap::new();
    let mut count = 0;
    let mut lines = input.lines();
    let enhancements = 50;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        for c in line.chars() {
            decoder.insert(count, c);
            count += 1;
        }
    }

    for (i, line) in lines.into_iter().enumerate() {
        for (j,c) in line.chars().enumerate() {
            image.insert((i as i32, j as i32), c);
        }
    }

    // Pad data with . symbols. 
    let (min_x, max_x, min_y, max_y) = size(&image);
    let (min_x_0, max_x_0, min_y_0, max_y_0) = (min_x, max_x, min_y, max_y);
    for x in min_x-enhancements*2..=max_x+enhancements*2 {
        for y in min_y-enhancements*2..=max_y+enhancements*2 {
            if let None = image.get(&(x, y)) {
                image.insert((x, y), '.');
            }
        }
    }
    let (min_x, max_x, min_y, max_y) = size(&image);


    for x in min_y..=max_y {
        let mut vec = String::new();
        for y in min_x..=max_x {
            vec.push(*image.get(&(x,y)).unwrap());
        } 
    }

    for i in 0..enhancements {
        println!("Applying enhancement nr:{}", i);
        image = parse_image(image, &decoder);
    }
    let mut count = 0; 
    for (_, val) in &image{
        
    };

    for x in min_x_0-enhancements..=max_x_0+enhancements {
        let mut vec = String::new();
        for y in min_y_0-enhancements..=max_y_0+enhancements {
            if let Some(val) = image.get(&(x, y)){
                if *val == '#' {
                    count += 1;
                }
            }
            vec.push(*image.get(&(x,y)).unwrap());
        } 
        println!("{:?}", vec)
    }
    println!("{}", count);
    // Part A: 5622
    // part B: 20395
}
