use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::vec;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let input: Vec<u32> = vec![59,91,13,82,8,32,74,96,55,51,19,47,46,44,5,21,95,71,48,60,68,81,80,14,23,28,26,78,12,22,49,1,83,88,39,53,84,37,93,24,42,7,56,20,92,90,25,36,34,52,27,50,85,75,89,63,33,4,66,17,98,57,3,9,54,0,94,29,79,61,45,86,16,30,77,76,6,38,70,62,72,43,69,35,18,97,73,41,40,64,67,31,58,11,15,87,65,2,10,99];
    

    let mut idx = 0;
    let mut i = 0;
    let mut boards = Vec::new();
    let mut board = vec![vec![0u32; 5]; 5];
    for line in reader.lines() {
        let line = match line {
            Ok(x) => x,
            Err(e) => continue
        };
        if line.is_empty() {
            continue;
        }
        
        let mut j = 0;
        for num in line.split(' ') {
            let my_num = match num.parse() {
                Ok(x) => x,
                Err(e) => continue,
            };
            board[i][j] = my_num;
            j +=1;
        }

        i += 1;

        if i==5 {
            let mut new_board = vec![vec![0u32; 5]; 5];
            for (ind, row)  in board.iter().enumerate() {
                for (idx, elem) in row.iter().enumerate() {
                    new_board[ind][idx] = *elem+1;  // adding one to all numbers here
                }
            }
            boards.push(new_board);
            idx += 1;
            i = 0;
            continue;
        }
    }

    for board in boards.iter() {
        //println!("{:?}\n", board);
    }
    

    for number in input.iter() {

        let mut num_boards = vec![1u32; boards.len()];
        let mut sum = 99;
        
        for (board_id, board) in boards.iter_mut().enumerate() {
            // update boards
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == *number+1 {
                        board[i][j] = 0;
                    }
                }
            }

            // check if any winner
            let winner = check_winner(board);
            if winner {
                let mut score = 0;
                for i in 0..5 {
                    for j in 0..5 {
                        if board[i][j] != 0 {
                            score += board[i][j]-1;
                        }
                    }
                }
                
                sum = num_boards.iter().sum();
                println!("num_boards: {:?}", num_boards);
                if board_id == num_boards.len() && sum == 1{
                    let code = number*score;
                    println!("num: {}, score: {}, code: {:?}", number, score, code);
                    exit(0);
                }
                num_boards[board_id] = 0;
            }
            
        }
    }
}

fn check_winner(board: &Vec<Vec<u32>>) -> bool {
    // column sum 
    for row in board.iter() {
        let sum: u32 = row.iter().sum();
        if sum == 0 {
            return true
        }
    }
    for i in 0..5 {
        let mut sum = 0;
        for j in 0..5 {
            sum += board[j][i]
        }
        if sum == 0 {
            return true;
        }
    }

    false
}