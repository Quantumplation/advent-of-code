use std::str::FromStr;
use std::{collections::{VecDeque}};
use std::iter::FromIterator;

pub mod part1 {
  use super::*;
  use anyhow::*;

  pub fn solve(game: CrabCombat) -> Result<usize> {
    let mut game = game;
    return Ok(game.run().1);
  }
}

pub struct Deck(VecDeque<u32>);
pub struct CrabCombat(Deck, Deck);
#[derive(Clone, Debug)]
pub enum Player { Player1, Player2 }

impl FromStr for Deck {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Deck(
      VecDeque::from_iter(s.lines()
       .map(|s| s.parse::<u32>())
       .filter(|s| s.is_ok())
       .map(|s| s.unwrap()))
    ))
  }
}

impl FromStr for CrabCombat {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split("\n\n");
    let (p1, p2) = (split.next().unwrap(), split.next().unwrap());
    Ok(CrabCombat(p1.parse::<Deck>().unwrap(), p2.parse::<Deck>().unwrap()))
  }
}

impl CrabCombat {
  pub fn round(&mut self) -> Option<Player> {
    let (mut p1, mut p2) = (&mut self.0, &mut self.1);
    {
      let (top1, top2) = (p1.0.pop_front().unwrap(), p2.0.pop_front().unwrap());
      let (winner, win, lose) = if top1 > top2 { (&mut p1, top1, top2) } else { (&mut p2, top2, top1) };
      winner.0.push_back(win);
      winner.0.push_back(lose);
    }
    if p1.0.len() == 0 { Some(Player::Player2) } else if p2.0.len() == 0 { Some(Player::Player1) } else { None }
  }
  pub fn score(&self, p: &Player) -> usize {
    let deck = match p {
      Player::Player1 => &self.0,
      Player::Player2 => &self.1,
    };
    deck.0.iter().rev().enumerate().map(|(idx, c)| (idx + 1) * (*c as usize)).sum()
  }
  pub fn run(&mut self) -> (Player, usize) {
    let winner = loop {
      if let Some(winner) = self.round() {
        break winner;
      }
    };
    (winner.clone(), self.score(&winner))
  }
}

#[cfg(test)]
mod test {
  use matches::assert_matches;

    use super::*;
  #[test]
  fn example() {
    let mut input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10".parse::<CrabCombat>().unwrap();
    let (winner, score) = input.run();
    assert_matches!(winner, Player::Player2);
    assert_eq!(306, score);
  }
}