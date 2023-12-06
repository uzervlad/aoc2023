use std::{fs, iter::zip};

fn part_one(input: &str) -> u32 {
  let mut lines = input.lines();

  let times = lines.next().unwrap()
    .split_once(":").unwrap()
    .1.split(" ").filter(|s| !s.is_empty())
    .flat_map(|s| s.parse::<u32>());

  let distances = lines.next().unwrap()
    .split_once(":").unwrap()
    .1.split(" ").filter(|s| !s.is_empty())
    .flat_map(|s| s.parse::<u32>());

  let races = zip(times, distances);

  let mut mul = 1;

  for race in races {
    for i in 1..race.0 {
      if i * (race.0 - i) > race.1 {
        mul *= race.0 - i * 2 + 1;
        break
      }
    }
  }

  mul
}

fn part_two(input: &str) -> u64 {
  let mut lines = input.lines();

  let time = lines.next().unwrap()
    .split_once(":").unwrap()
    .1.replace(" ", "").trim().parse::<u64>().unwrap();

  let distance = lines.next().unwrap()
    .split_once(":").unwrap()
    .1.replace(" ", "").trim().parse::<u64>().unwrap();

  for i in 1..time {
    if i * (time - i) > distance {
      return time - i * 2 + 1
    }
  }

  0
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();

  let one = part_one(&input);
  let two = part_two(&input);

  println!("Part One: {}", one);
  println!("Part Two: {}", two);
}
