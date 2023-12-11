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

fn expand_universe(map: &Vec<Vec<i64>>) -> Vec<i64> {
    let mut expanded_indices = Vec::new();
    for (idx, row) in map.iter().enumerate() {
        if row.iter().all(|x| *x == 0) {
            expanded_indices.push(idx as i64)
        }
    }
    return expanded_indices
} 

fn part_a(input: &str, expanded_distance: i64) -> i64 {
    let mut answer = 0;
    let mut map = Vec::new();
    let mut galaxies: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in input.trim().split("\r\n").enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    row.push(1);
                    galaxies.insert((x as i64, y as i64));
                },
                _ => row.push(0),
            }
        }
        map.push(row);
    }
    let expanded_rows = expand_universe(&map);
    let expanded_cols = expand_universe(&transpose(&map));
    
    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            if g1 != g2 {
                let mut x_dist = 0;
                let mut y_dist = 0;
                for x in g2.0.min(g1.0)..g2.0.max(g1.0) {
                    if expanded_cols.contains(&x) {
                        x_dist += expanded_distance-1
                    }
                    x_dist += 1
                }
                for y in g2.1.min(g1.1).. g2.1.max(g1.1) {
                    if expanded_rows.contains(&y) {
                        y_dist += expanded_distance-1
                    }
                    y_dist += 1
                }
                answer += x_dist + y_dist
            }
        }
    }
    return answer/2
}


fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input, 1);
    let ans_b = part_a(input, 1000000);
    println!("Part A: {:?}", ans_a);
    println!("Part B: {:?}", ans_b);
}
