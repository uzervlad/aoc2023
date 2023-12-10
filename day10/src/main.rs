use std::fs;

type Position = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeTile {
  Ground,
  Animal,
  Pipe(Position, Position),
}

impl From<char> for PipeTile {
  fn from(value: char) -> Self {
    match value {
      '.' => PipeTile::Ground,
      'S' => PipeTile::Animal,
      '-' => PipeTile::Pipe((-1, 0), (1, 0)),
      '|' => PipeTile::Pipe((0, -1), (0, 1)),
      'L' => PipeTile::Pipe((0, -1), (1, 0)),
      'J' => PipeTile::Pipe((0, -1), (-1, 0)),
      '7' => PipeTile::Pipe((0, 1), (-1, 0)),
      'F' => PipeTile::Pipe((0, 1), (1, 0)),
      _ => panic!()
    }
  }
}

type PipeMap = Vec<Vec<PipeTile>>;

trait PositionTrait where Self: Sized {
  fn reverse(&self) -> Self;
  fn neighbors(&self) -> [Self; 4];
  fn fits(&self, w: usize, h: usize) -> bool;
  fn on_edge(&self, w: usize, h: usize) -> bool;
}

impl PositionTrait for Position {
  fn reverse(&self) -> Self {
    (-self.0, -self.1)
  }

  fn neighbors(&self) -> [Self; 4] {
    [
      (self.0 - 1, self.1),
      (self.0 + 1, self.1),
      (self.0, self.1 - 1),
      (self.0, self.1 + 1),
    ]
  }

  fn fits(&self, w: usize, h: usize) -> bool {
    self.0 >= 0 && self.0 < h as i32 && self.1 >= 0 && self.1 < w as i32
  }

  fn on_edge(&self, w: usize, h: usize) -> bool {
    self.0 == 0 || self.0 + 1 == h as i32 || self.1 == 0 || self.1 + 1 == w as i32
  }
}

fn solve_one(map: &PipeMap, animal: Position) -> u32 {
  let adjacent: Vec<(Position, PipeTile)> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    .iter()
    .filter(|pos| animal.0 + pos.0 >= 0 && animal.1 + pos.1 >= 0)
    .map(|pos| (animal.0 + pos.0, animal.1 + pos.1))
    .map(|pos| (pos, map[pos.1 as usize][pos.0 as usize]))
    .filter(|(pos, tile)| {
      if let PipeTile::Pipe(a, b) = tile {
        [a, b].iter().any(|d| (pos.0 + d.0, pos.1 + d.1) == animal)
      } else {
        false
      }
    })
    .collect();

  let mut moves = 1;
  let mut current_tile = adjacent[0];
  let mut last_move = (current_tile.0.0 - animal.0, current_tile.0.1 - animal.1);

  loop {
    let PipeTile::Pipe(a, b) = current_tile.1 else { panic!() };
    let next_move = if a.reverse() == last_move { b } else { a };
    last_move = next_move;
    let pos = (current_tile.0.0 + next_move.0, current_tile.0.1 + next_move.1);
    current_tile = (pos, map[pos.1 as usize][pos.0 as usize]);
    moves += 1;
    if current_tile.1 == PipeTile::Animal {
      break
    }
  }

  moves / 2
}

fn solve_two(map: &PipeMap, animal: Position) -> u32 {
  let (h, w) = (map.len(), map[0].len());
  let mut checked: Vec<Vec<bool>> = map.iter().map(|row| row.iter().map(|_| false).collect()).collect();
  checked[animal.1 as usize][animal.0 as usize] = true;

  // Same as part one
  {
    let adjacent: Vec<(Position, PipeTile)> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
      .iter()
      .filter(|pos| animal.0 + pos.0 >= 0 && animal.1 + pos.1 >= 0)
      .map(|pos| (animal.0 + pos.0, animal.1 + pos.1))
      .map(|pos| (pos, map[pos.1 as usize][pos.0 as usize]))
      .filter(|(pos, tile)| {
        if let PipeTile::Pipe(a, b) = tile {
          [a, b].iter().any(|d| (pos.0 + d.0, pos.1 + d.1) == animal)
        } else {
          false
        }
      })
      .collect();

    let mut current_tile = adjacent[0];
    let mut last_move = (current_tile.0.0 - animal.0, current_tile.0.1 - animal.1);

    loop {
      checked[current_tile.0.1 as usize][current_tile.0.0 as usize] = true;
      let PipeTile::Pipe(a, b) = current_tile.1 else { panic!() };
      let next_move = if a.reverse() == last_move { b } else { a };
      last_move = next_move;
      let pos = (current_tile.0.0 + next_move.0, current_tile.0.1 + next_move.1);
      current_tile = (pos, map[pos.1 as usize][pos.0 as usize]);
      if current_tile.1 == PipeTile::Animal {
        break
      }
    }
  }

  let main_loop: Vec<(usize, usize)> = (0..h)
    .flat_map(|j| (0..w).map(|i| (i, j)).collect::<Vec<(usize, usize)>>())
    .filter(|p| checked[p.1][p.0]).collect();

  let mut enclosed = 0;

  while checked.iter().any(|row| row.iter().any(|a| !a)) {
    let start = {
      let mut r = (0, 0);
      'a: for i in 0..h {
        for j in 0..w {
          if !checked[i][j] {
            r = (j as i32, i as i32);
            break 'a
          }
        }
      }
      r
    };

    checked[start.1 as usize][start.0 as usize] = true;

    let mut traversed = vec![start];
    let mut to_check = vec![start];

    loop {
      let mut exit = true;
      let mut next_to_check = vec![];
      for tile in to_check.iter() {
        for neighbor in tile.neighbors() {
          if neighbor.fits(h, w) && !checked[neighbor.1 as usize][neighbor.0 as usize] {
            traversed.push(neighbor);
            next_to_check.push(neighbor);
            checked[neighbor.1 as usize][neighbor.0 as usize] = true;
            exit = false;
          }
        }
      }
      if exit || next_to_check.is_empty() {
        break
      }
      to_check = next_to_check;
    }

    if {
      let mut intersections = 0;
      let mut point = traversed[0];

      while point.0 >= 0 {
        point.0 -= 1;
        if main_loop.contains(&(point.0 as usize, point.1 as usize)) {
          match map[point.1 as usize][point.0 as usize] {
            PipeTile::Pipe((0, -1), (0, 1)) => intersections += 1,
            PipeTile::Pipe(a, _) => {
              loop {
                point.0 -= 1;
                match map[point.1 as usize][point.0 as usize] {
                  PipeTile::Pipe((-1, 0), (1, 0)) => {}
                  PipeTile::Pipe(b, _) => {
                    if a.reverse() == b {
                      intersections += 1;
                    }
                    break
                  },
                  _ => {}
                }
              }
            },
            _ => {}
          }
        }
      }

      intersections % 2 == 1
    } {
      enclosed += traversed.len() as u32;
    }
  }

  enclosed
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();

  let mut animal = (0, 0);

  let map: PipeMap = input.lines()
    .enumerate()
    .map(|(i, line)| {
      line.chars()
        .enumerate()
        .map(|(j, c)| {
          if c == 'S' {
            animal = (j as i32, i as i32);
          }
          c.into()
        })
        .collect()
    })
    .collect();

  let one = solve_one(&map, animal);
  let two = solve_two(&map, animal);

  println!("Part One: {:?}", one);
  println!("Part Two: {:?}", two);
}
