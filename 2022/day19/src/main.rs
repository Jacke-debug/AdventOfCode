use core::time;


#[derive(Debug, Clone, Copy)]
struct Bot {
    cost: [u32; 4],
    income: [u32; 4],
}
#[derive(Debug)]
struct Blueprint {
    ore_bot: Bot,
    clay_bot: Bot,
    obsidian_bot: Bot,
    geode_bot: Bot,
}

fn parse_blueprint(line: &str) -> Blueprint {
    let (p1, rest) = line.split_once(" ore.").unwrap();
    let ore_bot: u32 = p1.split_once("costs ").unwrap().1.parse().unwrap();
    let (p1, rest) = rest.split_once(" ore.").unwrap();
    let clay_bot: u32 = p1.split_once("costs ").unwrap().1.parse().unwrap();
    let (p1, rest) = rest.split_once(" ore").unwrap();
    let obsidian_bot_ore: u32 = p1.split_once("costs ").unwrap().1.parse().unwrap();
    let (p1, rest) = rest.split_once(" clay.").unwrap();
    let obsidian_bot_clay: u32 = p1.split_once("and ").unwrap().1.parse().unwrap();
    let (p1, rest) = rest.split_once(" ore").unwrap();
    let geode_bot_ore: u32 = p1.split_once("costs ").unwrap().1.parse().unwrap();
    let (p1, rest) = rest.split_once(" obsidian.").unwrap();
    let geode_bot_obsidian: u32 = p1.split_once("and ").unwrap().1.parse().unwrap();
    Blueprint {
        ore_bot: Bot {
            cost: [ore_bot, 0, 0, 0],
            income: [1, 0, 0, 0],
        },
        clay_bot: Bot {
            cost: [clay_bot, 0, 0, 0],
            income: [0, 1, 0, 0],
        },
        obsidian_bot: Bot {
            cost: [obsidian_bot_ore, obsidian_bot_clay, 0, 0],
            income: [0, 0, 1, 0],
        },
        geode_bot: Bot {
            cost: [geode_bot_ore, 0, geode_bot_obsidian, 0],
            income: [0, 0, 0, 1],
        },
    }
}

fn make_bot(a: &[u32; 4], b: &[u32; 4]) -> Option<[u32; 4]> {
    let mut result = [0; 4];
    for i in 0..4 {
        if a[i] < b[i] {
            return None;
        }
        result[i] = a[i] - b[i];
    }
    Some(result)
}

fn add_arrays(a: &[u32; 4], b: &[u32; 4]) -> [u32; 4] {
    let mut result = [0; 4];
    for i in 0..4 {
        result[i] = a[i] + b[i];
    }
    result
}


impl Blueprint {
    fn get_highest_cost_of_each(&self) -> [u32; 4] {
        let mut r1 = [0; 4];
        let mut r2 = [0; 4];
        let mut result = [0, 0, 0, u32::MAX];
    
        for ((&x, &y), z) in self.ore_bot.cost.iter().zip(self.clay_bot.cost.iter()).zip(r1.iter_mut()) {
            *z = x.max(y);
        }
        for ((&x, &y), z) in self.obsidian_bot.cost.iter().zip(self.geode_bot.cost.iter()).zip(r2.iter_mut()) {
            *z = x.max(y);
        }
        for ((&x, &y), z) in r1.iter().zip(r2.iter()).zip(result.iter_mut()) {
            *z = x.max(y);
        }
        result[3] = u32::MAX;
        result
    }

    fn evaluate(&self, time0: &i32, resources0: &[u32; 4], income0: &[u32; 4], highest_cost: &[u32; 4], possible_bot_to_make: &Vec<Bot>, most: &mut u32){
        // chose one robot to save up to, run until we can buy it
        match make_bot(&income0, &self.geode_bot.cost) {
            Some(_) => {
                //println!("making new Geode each turn");
                // we can afford to make a new geode_bot every turn
                let time_left = *time0 as u32;
                let mut new_goedes = (1..=time_left).sum();
                new_goedes += income0[3]*time_left + resources0[3];
                *most = std::cmp::max(*most, new_goedes);
                return
            }
            None => {
                // Don't evaluate any branch that can not be better than our current best. 
                // This is critial to save time
                let time_left = *time0 as u32 - 1;
                let mut new_goedes: u32 = (1..=time_left).sum();
                new_goedes += income0[3]*time_left + resources0[3];
                if new_goedes < *most {
                    return;
                }
            }
        }

        // check if income is larger than highest_cost for any of the minerals, only make the other types of bots
        for bot_to_make in possible_bot_to_make.iter() {
            let mut time = time0.clone();
            let mut resources = resources0.clone();
            let income = income0.clone();
            while time > 0 {
                time -= 1;
                match make_bot(&resources, &bot_to_make.cost) {
                    Some(mut new_res) => {
                        new_res = add_arrays( &new_res, &income);
                        let new_income = &add_arrays(&income, & bot_to_make.income);
                        let possible_bot_to_make: Vec<Bot> = (0..4)
                            .filter(|&i| new_income[i] < highest_cost[i])
                            .map(|i| [self.ore_bot, self.clay_bot, self.obsidian_bot, self.geode_bot][i])
                            .collect();
                        //println!("Making {:?}, income {:?}", bot_to_make, new_income);
                        self.evaluate(&time, &new_res, new_income, highest_cost, &possible_bot_to_make, most)
                    },
                    None => {
                        resources = add_arrays( &resources, &income);
                        continue;
                    },
                }
            }
            *most = std::cmp::max(*most, resources[3]);
        }        
    }
}

fn part_a(input: &str) {
    let max_time = 24;
    let mut data = input.split("\r\n");
    let mut blueprints: Vec<Blueprint> = Vec::new();
    while let Some(line) = data.next(){
        blueprints.push(parse_blueprint(line));
    }
    let mut quality_level = Vec::new();
    for (nr, blueprint) in blueprints.iter().enumerate() {
        println!("Evaluating blueprint nr {}/{}", nr+1, blueprints.len());
        let mut most = 0;
        let highest_cost = blueprint.get_highest_cost_of_each();
        let possible_bots_to_make: Vec<Bot> = vec![blueprint.ore_bot, blueprint.clay_bot, blueprint.obsidian_bot, blueprint.geode_bot];
        blueprint.evaluate(&max_time, &[0, 0, 0, 0],  &[1, 0, 0, 0], &highest_cost, &possible_bots_to_make,&mut most);
        quality_level.push((nr as u32 +1)*most);
    }

    let score: u32 = quality_level.iter().sum();
    println!("Score A {}", score)
}

fn part_b(input: &str) {
    let max_time = 32;
    let mut data = input.split("\r\n");
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for _ in 0..3 {
        match data.next() {
            Some(line) => {
                blueprints.push(parse_blueprint(line));
            },
            None => {
                break;
            },
        }
    }
    let mut quality_level = Vec::new();
    for (nr, blueprint) in blueprints.iter().enumerate() {
        println!("Evaluating blueprint nr {}/{}", nr+1, blueprints.len());
        let mut most = 0;
        let highest_cost = blueprint.get_highest_cost_of_each();
        let possible_bots_to_make: Vec<Bot> = vec![blueprint.ore_bot, blueprint.clay_bot, blueprint.obsidian_bot, blueprint.geode_bot];
        blueprint.evaluate(&max_time, &[0, 0, 0, 0],  &[1, 0, 0, 0], &highest_cost, &possible_bots_to_make,&mut most);
        quality_level.push(most);
    }

    let score: u32 = quality_level.iter().product();
    println!("Score B {}", score)
}

fn main() {
    let input = include_str!("input.txt");
    //part_a(input);    // 1766
    part_b(input);      // 30780
}
