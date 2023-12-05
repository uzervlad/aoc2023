use std::{ops::Range, fs};

struct Map {
  src: Range<u64>,
  dest: u64,
}

impl Map {
  fn new(dest: u64, src: u64, len: u64) -> Self {
    Self {
      src: src..src+len,
      dest
    }
  }

  fn map(&self, n: u64) -> Option<u64> {
    if self.src.contains(&n) {
      Some(self.dest + (n - self.src.start))
    } else {
      None
    }
  }
}

fn map_seed(seed: u64, maps: &Vec<Vec<Map>>) -> u64 {
  let mut seed = seed;

  for map in maps {
    seed = map.iter().flat_map(|m| m.map(seed)).next().unwrap_or(seed);
  }

  seed
}

fn min_one(seeds: &Vec<u64>, maps: &Vec<Vec<Map>>) -> u64 {
  seeds.iter()
    .map(|s| map_seed(*s, maps))
    .min().unwrap()
}

fn min_two(seeds: &Vec<u64>, maps: &Vec<Vec<Map>>) -> u64 {
  seeds.chunks(2)
    .map(|c| c[0]..c[0]+c[1])
    .flatten()
    .into_iter()
    .map(|s| map_seed(s, maps))
    .min()
    .unwrap()
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();

  let mut lines = input.lines();

  let seeds: Vec<u64> = lines.next().unwrap()
    .split_whitespace().skip(1)
    .flat_map(|s| s.parse::<u64>())
    .collect();

  let mut maps: Vec<Vec<Map>> = Vec::new();

  for line in lines {
    if line.ends_with("map:") {
      maps.push(Vec::new())
    } else if !line.is_empty() {
      let n: Vec<u64> = line.split(" ").flat_map(|s| s.parse::<u64>()).collect();
      let map = Map::new(n[0], n[1], n[2]);
      maps.last_mut().unwrap().push(map);
    }
  }

  let one = min_one(&seeds, &maps);
  let two = min_two(&seeds, &maps);
  
  println!("Part One: {}", one);
  println!("Part Two: {}", two);
}
