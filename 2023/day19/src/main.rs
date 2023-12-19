use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Items {
    x: std::ops::Range<usize>,
    m: std::ops::Range<usize>,
    a: std::ops::Range<usize>,
    s: std::ops::Range<usize>,
}

fn apply_action(
  workflows: &HashMap<&str, Vec<&str>>, 
  accepted: &mut Vec<Items>,
  item: &mut Items, 
  act: &str
) {
  match act {
    "A" => {
      accepted.push(item.clone());
      return;
    }, 
    "R" => {return}
    _ => {}
  }
  let workflow = workflows.get(act).unwrap();
  loop {
    for action in workflow {
      let mut next_action = None;
      match action.split_once(':') {
          Some((cond, act)) => {
            match cond.chars().nth(1) {
              Some('<') => {
                let val = cond[2..].parse::<usize>().unwrap();
                let old = item.clone();
                let mut new_item = item.clone();
                match cond.chars().nth(0).unwrap() {
                    'x' => {
                        item.x = val..old.x.end;
                        new_item.x = old.x.start..val;
                    }
                    'm' => {
                        item.m = val..old.m.end;
                        new_item.m = old.m.start..val;
                    }
                    'a' => {
                        item.a = val..old.a.end;
                        new_item.a = old.a.start..val;
                    }
                    's' => {
                        item.s = val..old.s.end;
                        new_item.s = old.s.start..val;
                    }
                    _ => panic!()
                };
                apply_action(workflows, accepted, &mut new_item, act);
              },
              Some('>') => {
                let val = cond[2..].parse::<usize>().unwrap()+1;
                let old = item.clone();
                let mut new_item = item.clone();
                match cond.chars().nth(0).unwrap() {
                    'x' => {
                        new_item.x = val..old.x.end;
                        item.x = old.x.start..val;
                    }
                    'm' => {
                        new_item.m = val..old.m.end;
                        item.m = old.m.start..val;
                    }
                    'a' => {
                        new_item.a = val..old.a.end;
                        item.a = old.a.start..val;
                    }
                    's' => {
                        new_item.s = val..old.s.end;
                        item.s = old.s.start..val;
                    }
                    _ => panic!()
                };
                apply_action(workflows, accepted, &mut new_item, act);
              },
              _ => panic!()
            }
          }
          None => {
            next_action = Some(action);
          }
      }
      match next_action {
          Some(&"A") => {
            accepted.push(item.clone());
            return;
          }, 
          Some(&"R") => {return}
          Some(act) => {
            apply_action(workflows, accepted, item, act);
            return
          }
          None => {
            continue
          }
      }
    }
  }
}

fn part_b(input: &str) -> usize {
  let mut ans_a = 0;
  let mut accepted: Vec<Items> = Vec::new();
  let mut workflows: HashMap<&str, Vec<&str>> = HashMap::new();
  let mut lines = input.lines();
  loop {
    let line = lines.next().unwrap();
    if line.is_empty() {
      break;
    }

    let (key, rest) = line.split_once('{').unwrap();
    let rest = rest.split_once('}').unwrap().0;
    let actions: Vec<&str> = rest.split(',').collect();
    workflows.insert(key, actions);
  }
  let mut item = Items{
    x: 1..4001,
    m: 1..4001,
    a: 1..4001,
    s: 1..4001,
  };

  apply_action(
    &workflows, 
    &mut accepted,
    &mut item, 
    "in"
  );

  for item in accepted {
    ans_a += item.x.len() * item.m.len() * item.a.len() * item.s.len();
  }
return ans_a
}



fn main() {
    let input = include_str!("input.txt");
    let ans_b = part_b(input);
    println!("Part B: {}", ans_b); 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), 167409079868000);
    }
}