use std::{fs::File, io::{BufReader, BufRead}};

#[allow(dead_code)]
fn get_calibration_value_one(reader: BufReader<File>) -> u32 {
  let mut calibration_value = 0u32;

  for line in reader.lines().flatten() {
    let numbers = line.chars()
      .filter_map(|c| c.to_digit(10))
      .collect::<Vec<u32>>();
    let a = numbers.first().unwrap();
    let b = numbers.last().unwrap();
    calibration_value += a * 10 + b;
  }

  calibration_value
}

#[allow(dead_code)]
fn get_calibration_value_two(reader: BufReader<File>) -> u32 {
  let mut calibration_value = 0u32;

  for line in reader.lines().flatten() {
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
  let file = File::open("./input").unwrap();
  let reader = BufReader::new(file);
  
  // println!("{}", get_calibration_value_one(reader));
  println!("{}", get_calibration_value_two(reader));
}
