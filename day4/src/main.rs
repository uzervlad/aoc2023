use std::{fs, collections::HashSet};

fn get_score(line: &str) -> u32 {
  let (_, numbers) = line.split_once(": ").unwrap();
  let (win, scratch) = numbers.split_once(" | ").unwrap();
  let win: HashSet<u32> = HashSet::from_iter(win.split(" ").filter(|s| !s.is_empty()).flat_map(str::parse::<u32>));
  let scratch: HashSet<u32> = HashSet::from_iter(scratch.split(" ").filter(|s| !s.is_empty()).flat_map(str::parse::<u32>));
  win.intersection(&scratch).count() as u32
}

fn get_one(input: &str) -> u32 {
  input
    .lines()
    .map(get_score)
    .map(|score| if score > 0 { 1 << score - 1 } else { 0 })
    .sum()
}

fn get_two(input: &str) -> u32 {
  let scores: Vec<_> = input.lines().map(get_score).collect();

  let mut cards = vec![1; scores.len()];

  for i in (0..cards.len()).rev() {
    for j in 1..=scores[i] as usize {
      cards[i] += cards[i + j];
    }
  }

  cards.iter().sum()
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();

  let one = get_one(&input);
  let two = get_two(&input);

  println!("Part One: {}", one);
  println!("Part Two: {}", two);
}
