use std::{fs, cmp::Ordering, collections::HashMap, time::Instant};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
  FiveOfAKind,
  FourOfAKind,
  FullHouse,
  ThreeOfAKind,
  TwoPair,
  OnePair,
  HighCard,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum HandCard {
  J,
  N2,
  N3,
  N4,
  N5,
  N6,
  N7,
  N8,
  N9,
  T,
  Q,
  K,
  A
}

impl From<char> for HandCard {
  fn from(value: char) -> Self {
    match value {
      '2' => Self::N2,
      '3' => Self::N3,
      '4' => Self::N4,
      '5' => Self::N5,
      '6' => Self::N6,
      '7' => Self::N7,
      '8' => Self::N8,
      '9' => Self::N9,
      'T' => Self::T,
      'J' => Self::J,
      'Q' => Self::Q,
      'K' => Self::K,
      'A' => Self::A,
      _ => panic!("")
    }
  }
}

#[derive(Debug)]
struct Hand {
  cards: Vec<HandCard>,
  bid: u32,
  hand_type: HandType,
}

fn main() {
  let input = fs::read_to_string("./input").unwrap();

  let mut hands: Vec<Hand> = input.lines()
    .map(|line| {
      let (hand, bid) = line.split_once(" ").unwrap();
      let hand_vals: Vec<HandCard> = hand.chars().map(|c| HandCard::from(c)).collect();
      let mut hand_map: HashMap<HandCard, usize> = HashMap::new();
      let mut jokers = 0;
      for card in hand_vals.iter() {
        match card {
          HandCard::J => jokers += 1,
          _ => {
            hand_map.entry(*card).and_modify(|v| *v += 1).or_insert(1);
          }
        }
      }
      let hand_type: HandType = {
        let vals: Vec<usize> = hand_map.into_iter().map(|(_, c)| c).sorted().rev().take(2).collect();
        let first = vals.get(0).unwrap_or(&0);
        let second = vals.get(1).unwrap_or(&0);
        match (*first + jokers, *second) {
          (5, _) => HandType::FiveOfAKind,
          (4, _) => HandType::FourOfAKind,
          (3, 2) => HandType::FullHouse,
          (3, _) => HandType::ThreeOfAKind,
          (2, 2) => HandType::TwoPair,
          (2, _) => HandType::OnePair,
          _ => HandType::HighCard
        }
      };
      Hand {
        cards: hand_vals,
        bid: bid.parse().unwrap(),
        hand_type
      }
    }).collect();

  hands.sort_by(|a, b| {
    match a.hand_type.cmp(&b.hand_type) {
      Ordering::Equal => for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
        match card_b.cmp(card_a) {
          Ordering::Equal => continue,
          ordering => return ordering
        }
      },
      ordering => return ordering
    };
    Ordering::Equal
  });

  let winnings = hands.iter().enumerate()
    .map(|(i, hand)| hand.bid * (hands.len() - i) as u32)
    .sum::<u32>();

  println!("Part Two: {}", winnings);
}
