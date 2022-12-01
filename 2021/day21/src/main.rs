use std::collections::HashMap;

static mut DIE: i32 = 0;

fn roll_deterministic() {
    unsafe{
        DIE = DIE+1;
        if DIE > 100 {
            DIE = 1;
        } 
    }
}

fn take_turn(pos: &mut i32, rolls: &mut i32) -> i32 {
    let mut step = 0;
    for _ in 0..3 {
        roll_deterministic();
        unsafe {
            step += DIE;
        }
    }
    *rolls +=3;

    *pos = (*pos + step)%10;
    if *pos == 0{
        10
    } else {
        *pos
    }
}

fn part_a() {
    let mut p1_pos = 4;
    let mut p2_pos = 8;   

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut rolls = 0;
    let mut p1_win = false;
    loop {
        p1_score += take_turn(&mut p1_pos, &mut rolls);
        
        if p1_score >= 1000 {
            p1_win = true;
            break;
        }

        p2_score += take_turn(&mut p2_pos, &mut rolls);
        
        if p2_score >= 1000 {
            break;
        }

        println!("score p1: {}, p2: {}", p1_score, p2_score);
    }

    if p1_win {
        println!("{}, {}", p2_score*rolls, rolls);
    } else {
        println!("{}, {}", p1_score*rolls, rolls);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    is_p1_turn: bool,
    p1_state: i64, 
    p1_score: i64, 
    p2_state: i64,
    p2_score: i64,
}

impl State {
    fn p1_wins(&self) -> bool {
        self.p1_score >=21
    }

    fn p2_wins(&self) -> bool {
        self.p2_score >=21
    }

    fn p1_roll(&self) -> Vec<State> {
        let mut new_states = Vec::new(); {
            for i in 1..=3 {
                for j in 1..=3 {
                    for k in 1..=3 {
                        let p1_state = (self.p1_state + i + j + k -1)%10 + 1;
                        let p1_score = self.p1_score + p1_state;
                        new_states.push(State{
                            p1_state,
                            p1_score, 
                            p2_state: self.p2_state,
                            p2_score: self.p2_score, 
                            is_p1_turn: false,
                        });
                    }
                }
            }
        }
        new_states
    }

    fn p2_roll(&self) -> Vec<State> {
        let mut new_states = Vec::new(); {
            for i in 1..=3 {
                for j in 1..=3 {
                    for k in 1..=3 {
                        let p2_state = (self.p2_state + i + j + k -1)%10 + 1;
                        let p2_score = self.p2_score + p2_state;
                        new_states.push(State{
                            p1_state: self.p1_state,
                            p1_score: self.p1_score, 
                            p2_state,
                            p2_score, 
                            is_p1_turn: true,
                        });
                    }
                }
            }
        }
        new_states
    }

    fn roll(&self) -> Vec<State> {
        if self.is_p1_turn{
            self.p1_roll()
        } else {
            self.p2_roll()
        }
    }
}

fn play(memo: &mut HashMap<State, (usize, usize)>, state: State) -> (usize, usize) {
    if let Some((p1_wins, p2_wins)) = memo.get(&state) {
        return(*p1_wins, *p2_wins); 
    }
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for case in state.roll() {
        if case.p1_wins() {
            p1_wins += 1;
        } else if case.p2_wins() {
            p2_wins += 1;
        } else {
            let (sub_p1, sub_p2) = play(memo, case);
            p1_wins += sub_p1;
            p2_wins += sub_p2;
        }
    }

    memo.insert(state.clone(), (p1_wins, p2_wins));
    return (p1_wins, p2_wins)
}

fn main() {
    let state = State {
        p1_state: 6,
        p1_score: 0,
        p2_state: 3,
        p2_score: 0,
        is_p1_turn: true,
    };
    let mut memo: HashMap<State, (usize, usize)> = HashMap::new();
    let (p1_wins, p2_wins) = play(&mut memo, state);
    println!("{}", p1_wins.max(p2_wins));
}