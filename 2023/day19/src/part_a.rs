

// x - cool
// m - musical
// a - aerodynamic
// s - shiny

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Item {
    x: usize, 
    m: usize,
    a: usize,
    s: usize,
}

fn apply_action(
  action_map: &HashMap<&str, Vec<&str>>, 
  accepted: &mut Vec<Item>,
  item: Item, 
  act: &str
) {
  let actions = action_map.get(act).unwrap();
  loop {
    for action in actions {
      let mut next_action = None;
      match action.split_once(':') {
          Some((cond, act)) => {
            let property = match cond.chars().nth(0).unwrap() {
              'x' => item.x,
              'm' => item.m,
              'a' => item.a,
              's' => item.s,
              _ => panic!()
            };
            match cond.chars().nth(1) {
              Some('<') => {
                let val = cond[2..].parse::<usize>().unwrap();
                if property < val {
                  next_action = Some(act);
                }
              },
              Some('>') => {
                let val = cond[2..].parse::<usize>().unwrap();
                if property > val {
                  next_action = Some(act);
                }
              },
              _ => panic!()
            }
          }
          None => {
            next_action = Some(action);
          }
      }
      match next_action {
          Some("A") => {
            accepted.push(item);
            return;
          }, 
          Some("R") => {return}
          Some(act) => {
            apply_action(action_map, accepted, item, act);
            return
          }
          None => continue
      }
    }
  }
}

fn parse_item(line: &str) -> Option<Item> {
  // Extract values from the line using string manipulation
  let mut values = line
      .trim_start_matches('{')
      .trim_end_matches('}')
      .split(',')
      .map(|pair| {
          let mut iter = pair.split('=');
          let key = iter.next()?.trim();
          let value = iter.next()?.trim();
          Some((key, value))
      })
      .collect::<Option<Vec<(&str, &str)>>>()?;

  // Parse values into an Item struct
  Some(Item {
      x: values.iter().find_map(|&(key, value)| if key == "x" { value.parse().ok() } else { None })?,
      m: values.iter().find_map(|&(key, value)| if key == "m" { value.parse().ok() } else { None })?,
      a: values.iter().find_map(|&(key, value)| if key == "a" { value.parse().ok() } else { None })?,
      s: values.iter().find_map(|&(key, value)| if key == "s" { value.parse().ok() } else { None })?,
  })
}


fn part_a(input: &str) -> usize {
    let mut ans_a = 0;
    let mut accepted: Vec<Item> = Vec::new();
    let mut workflows = HashMap::new();
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

    while let line = lines.next() {
      let line = match line {
        Some(l) => l, 
        None => break,
      };
      let item = parse_item(line).unwrap();
      apply_action(
        &workflows, 
        &mut accepted,
        item, 
        "in"
      );
    }
    
    println!("{:?}", accepted);
    for item in accepted {
      ans_a += item.x + item.m +item.a + item.s;
    }
  return ans_a
}


fn part_b(input: &str) -> usize {
  let mut ans_a = 0;
  let mut accepted: Vec<Item> = Vec::new();
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
  
  item.x = 1..=4000;
  item.m = 1..=4000;
  item.a = 1..=4000;
  item.s = 1..=4000;
  apply_action(
    &workflows, 
    &mut accepted,
    item, 
    "in"
  );

  for item in accepted {
    ans_a += item.x + item.m +item.a + item.s;
  }
return ans_a
}



fn main() {
    let input = include_str!("input.txt");
    let ans_a = part_a(input);
    let ans_b = part_b(input);
    println!("Part A: {}", ans_a);
    println!("Part B: {}", ans_b); 
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_a_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_a(input), 495298);
    }
    #[test]
    fn test_b_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_b(input), 167409079868000);
    }
}