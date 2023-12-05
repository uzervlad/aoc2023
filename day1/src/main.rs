use std::fs;

fn get_calibration_value_one(input: &str) -> u32 {
  let mut calibration_value = 0u32;

  for line in input.lines() {
    let numbers = line.chars()
      .filter_map(|c| c.to_digit(10))
      .collect::<Vec<u32>>();
    let a = numbers.first().unwrap();
    let b = numbers.last().unwrap();
    calibration_value += a * 10 + b;
  }

  calibration_value
}

fn get_calibration_value_two(input: &str) -> u32 {
  let mut calibration_value = 0u32;

  for line in input.lines() {
    let numbers = line.chars()
      .enumerate()
      .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
      .collect::<Vec<(usize, u32)>>();
    let str_first_indices = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter()
      .map(|s| line.find(s))
      .collect::<Vec<Option<usize>>>();
    let str_last_indices = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter()
      .map(|s| line.rfind(s))
      .collect::<Vec<Option<usize>>>();
    let a = {
      let mut i = line.len();
      let mut a = 0;
      for j in 1..=9 {
        if let Some(k) = str_first_indices[j - 1] {
          if i > k {
            i = k;
            a = j;
          }
        }
      }
      match numbers.first() {
        Some((j, a)) if *j < i => *a,
        _ => a as u32
      }
    };
    let b = {
      let mut i = 0;
      let mut b = 0;
      for j in 1..=9 {
        if let Some(k) = str_last_indices[j - 1] {
          if i < k {
            i = k;
            b = j;
          }
        }
      }
      match numbers.last() {
        Some((j, b)) if *j >= i => *b,
        _ => b as u32
      }
    };
    calibration_value += a * 10 + b;
  }

  calibration_value
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();
  
  let one = get_calibration_value_one(&input);
  let two = get_calibration_value_two(&input);

  println!("Part One: {}", one);
  println!("Part Two: {}", two);
}
