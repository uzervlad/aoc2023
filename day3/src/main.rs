use std::fs;

fn include((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
  (x1 as isize - x2 as isize).abs().max((y1 as isize - y2 as isize).abs()) == 1
}

fn get_sum_one(input: &str) -> u32 {
  let mut sum = 0;

  let mut symbols: Vec<(usize, usize)> = Vec::new();

  for (x, line) in input.lines().enumerate() {
    for (y, ch) in line.chars().enumerate() {
      if !ch.is_numeric() && ch != '.' {
        symbols.push((x, y));
      }
    }
  }

  for (x, line) in input.lines().enumerate() {
    let mut part = 0;
    let mut included = false;

    for (y, ch) in line.chars().enumerate() {
      if ch.is_numeric() {
        part = part * 10 + ch.to_digit(10).unwrap();
        if symbols.iter().any(|s| include(*s, (x, y))) {
          included = true;
        }
      } else {
        if included {
          sum += part;
        }
        part = 0;
        included = false;
      }
    }

    if included {
      sum += part;
    }
  }

  sum
}

fn get_sum_two(input: &str) -> u32 {
  #[derive(Default)]
  struct Gear {
    coords: (usize, usize),
    parts: Vec<u32>,
  }

  let mut gears: Vec<Gear> = Vec::new();

  for (x, line) in input.lines().enumerate() {
    for (y, ch) in line.chars().enumerate() {
      if ch == '*' {
        gears.push(Gear { coords: (x, y), ..Gear::default() });
      }
    }
  }

  for (x, line) in input.lines().enumerate() {
    let mut part = 0;
    let mut gear: Option<usize> = None;

    for (y, ch) in line.chars().enumerate() {
      if ch.is_numeric() {
        part = part * 10 + ch.to_digit(10).unwrap();
        if let Some((idx, _)) = gears.iter().enumerate().find(|(_, g)| include(g.coords, (x, y))) {
          gear = Some(idx);
        }
      } else {
        if let Some(gear) = gear {
          gears[gear].parts.push(part);
        }
        part = 0;
        gear = None;
      }
    }

    if let Some(gear) = gear {
      gears[gear].parts.push(part);
    }
  }

  gears.iter()
    .filter(|gear| gear.parts.len() == 2)
    .map(|gear| gear.parts[0] * gear.parts[1])
    .sum()
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();

  let one = get_sum_one(&input);
  let two = get_sum_two(&input);

  println!("Part One: {}", one);
  println!("Part Two: {}", two);
}
