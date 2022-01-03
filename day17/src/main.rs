use std::{process::exit, cmp::max, collections::HashMap};

fn step(pos: &mut [i32; 2], vel_x: &mut i32, vel_y: &mut i32) {
    pos[0] += *vel_x;
    pos[1] += *vel_y;

    if vel_x > &mut 0 {
        *vel_x -= 1;
    } else if pos[0] < 0 {
        *vel_x += 1;
    }

    *vel_y -= 1;
}

fn main() {

    let target_x = 20..30;
    let target_y = -10..-5;

    let target_x = 150..171;
    let target_y = -129..-70;
    
    let mut pos = [0, 0];
    // let mut vel0 = [0, -20];
    let mut vel_x0 = 0;
    let mut vel_y0 = -500;  // sensitive

    let mut res = HashMap::new();
    let mut count = 0;

    loop {
        let mut vel_x = vel_x0;
        let mut vel_y = vel_y0;
        let mut max_y = 0;

        while pos[0] <= target_x.end && pos[1] >= target_y.start {
            step(&mut pos, &mut vel_x, &mut vel_y);
            max_y = max(max_y, pos[1]);

            //if target_x.contains(&pos[0]) && target_y.contains(&pos[1]) {
            if pos[0] >= 150 && pos[0] <= 171 && pos[1] >= -129 && pos[1] <= -70 {
                //println!("success, max height: {}", max_y);
                res.insert((vel_x0, vel_y0), max_y);
                break;
            }
        }

        pos = [0, 0];
        count += 1;
        if vel_x < target_x.end {
            vel_x0 += 1;
            count = 0;
        } else if count >= 4000 {
            count = 0;
            vel_x0 = 0;
            vel_y0 += 1;
        } 

        if vel_y0 > 5000 {
            println!("{:?}", res);
            let key_with_max_value = res.iter().max_by_key(|entry | entry.1).unwrap();
            println!("{:?}", res.len());
            // 2326
            break;
        }
    }
}
