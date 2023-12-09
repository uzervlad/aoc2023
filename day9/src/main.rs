use std::fs;

fn extrapolate_sequence(sequence: &Vec<i32>, backwards: bool) -> i32 {
  let next_sequence: Vec<i32> = (1..sequence.len())
    .map(|i| sequence[i] - sequence[i - 1])
    .collect();

  if next_sequence.iter().all(|n| *n == 0) {
    sequence[0]
  } else if backwards {
    sequence[0] - extrapolate_sequence(&next_sequence, backwards)
  } else {
    sequence[sequence.len() - 1] + extrapolate_sequence(&next_sequence, backwards)
  }
}

fn solve(sequences: &Vec<Vec<i32>>, backwards: bool) -> i32 {
  sequences.iter()
    .map(|sequence| extrapolate_sequence(sequence, backwards))
    .sum::<i32>()
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();

  let sequences: Vec<Vec<i32>> = input.lines()
    .map(|line| {
      line.split_whitespace()
        .flat_map(|n| n.parse::<i32>())
        .collect()
    })
    .collect();

  let one = solve(&sequences, false);
  let two = solve(&sequences, true);

  println!("Part One: {}", one);
  println!("Part Two: {}", two);
}
