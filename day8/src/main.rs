use std::{fs, collections::HashMap};

fn solve_node(start: &str, nodes: &HashMap<&str, (&str, &str)>, steps: &Vec<char>) -> u64 {
  let l = steps.len() as u64;
  let mut current_node = start;
  let mut taken: u64 = 0;
  loop {
    let step = steps[(taken % l) as usize];

    let branches = nodes.get(&current_node).unwrap();
    match step {
      'L' => current_node = branches.0,
      'R' => current_node = branches.1,
      _ => (),
    }

    taken += 1;

    if current_node.ends_with('Z') {
      break
    }
  }

  taken
}

fn part_one(nodes: &HashMap<&str, (&str, &str)>, steps: &Vec<char>) -> u64 {
  solve_node("AAA", nodes, steps)
}

fn part_two(nodes: &HashMap<&str, (&str, &str)>, steps: &Vec<char>) -> u64 {
  let factors: Vec<u64> = nodes.keys()
    .filter(|n| n.ends_with('A'))
    .map(|n| solve_node(*n, nodes, steps))
    .collect();

  let mut lcm = num_integer::lcm(factors[0], factors[1]);
  for i in 2..factors.len() {
    lcm = num_integer::lcm(lcm, factors[i]);
  }

  lcm
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();
  let mut lines = input.lines();

  let steps: Vec<char> = lines.next().unwrap().chars().collect();

  let nodes: HashMap<&str, (&str, &str)> = lines
    .filter(|l| !l.is_empty())
    .map(|line| {
      let (node, branches) = line.split_once(" = ").unwrap();
      let pat: &[_] = &['(', ')'];
      let branches = branches.trim_matches(pat).split_once(", ").unwrap();
      (node, branches)
    })
    .collect();

  let one = part_one(&nodes, &steps);
  let two = part_two(&nodes, &steps);

  println!("Part One: {}", one);
  println!("Part Two: {}", two);
}
