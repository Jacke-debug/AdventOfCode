use std::collections::HashSet;

fn transpose(map: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let num_rows = map.len();
    let num_columns = map[0].len();
    (0..num_columns)
        .map(|col_index| {
            (0..num_rows)
                .map(|row_index| map[row_index][col_index])
                .collect()
        })
        .collect()
}

fn expand_universe(map: &Vec<Vec<i64>>) -> (Vec<i64>, Vec<i64>) {
    let mut rows = Vec::new();
    let mut cols = Vec::new();
    for (idx, row) in map.iter().enumerate() {
        if row.iter().all(|x| *x == 0) {
            rows.push(idx as i64)
        }
    }
    let transposed_map = transpose(map);
    for (idx, row) in transposed_map.iter().enumerate() {
        if row.iter().all(|x| *x == 0) {
            cols.push(idx as i64)
        }
    }
    return (rows, cols);
} 

fn part_a(input: &str, expanded_distance: i64) -> i64 {
    let mut score = 0;
    let lines = input.trim().split("\r\n");
    let mut map = Vec::new();
    let mut galaxy_count = 1;
    for (y, line) in lines.enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    row.push(galaxy_count);
                    galaxy_count += 1;
                },
                _ => row.push(0),
            }
        }
        map.push(row);
    }
    let (expanded_rows, expanded_cols) = expand_universe(&map);

    let mut galaxies = HashSet::new();
    for (x, row) in map.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            if *item > 0 {
                galaxies.insert((x as i64, y as i64));
            }
        }
    }

    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            if g1 != g2 {
                let mut x_dist = 0;
                let mut y_dist = 0;
                for x in g2.0.min(g1.0)..g2.0.max(g1.0) {
                    if expanded_rows.contains(&x) {
                        x_dist += expanded_distance
                    } else {
                        x_dist += 1
                    }
                    
                }
                for y in g2.1.min(g1.1).. g2.1.max(g1.1) {
                    if expanded_cols.contains(&y) {
                        y_dist += expanded_distance
                    } else {
                        y_dist += 1
                    }
                    
                }
                score += x_dist + y_dist
            }
        }
    }
    return score/2
}


fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input, 1);
    let ans_b = part_a(input, 1000000);
    println!("Part A: {:?}", ans_a);
    println!("Part B: {:?}", ans_b);
}
