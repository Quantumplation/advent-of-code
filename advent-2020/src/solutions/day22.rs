use std::{collections::{HashMap, HashSet}, str::FromStr};
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
pub mod part2 {
  use super::*;
  use anyhow::*;

  pub fn solve(game: CrabCombat) -> Result<usize> {
    let mut game: RecursiveCombat = game.into();
    return Ok(game.run().1);
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Deck(VecDeque<usize>);
pub struct CrabCombat(Deck, Deck);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RecursiveCombat(Deck, Deck);
#[derive(Clone, Debug)]
pub enum Player { Player1, Player2 }

impl FromStr for Deck {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Deck(
      VecDeque::from_iter(s.lines()
       .map(|s| s.parse::<usize>())
       .filter(|s| s.is_ok())
       .map(|s| s.unwrap()))
    ))
  }
}

impl From<CrabCombat> for RecursiveCombat {
    fn from(c: CrabCombat) -> Self {
      RecursiveCombat(c.0, c.1)
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

impl Deck {
  pub fn score(&self) -> usize {
    self.0.iter().rev().enumerate().map(|(idx, c)| (idx + 1) * (*c as usize)).sum()
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
  
  pub fn run(&mut self) -> (Player, usize) {
    let winner = loop {
      if let Some(winner) = self.round() {
        break winner;
      }
    };
    let winning_deck = match winner {
      Player::Player1 => &self.0,
      Player::Player2 => &self.1,
    };
    (winner.clone(), winning_deck.score())
  }
}

impl RecursiveCombat {
  fn subgame(&self, p1: usize, p2: usize) -> RecursiveCombat {
    let Deck(p1_deck) = &self.0;
    let Deck(p2_deck) = &self.1;
    RecursiveCombat(
      Deck(VecDeque::from_iter(p1_deck.iter().cloned().take(p1))),
      Deck(VecDeque::from_iter(p2_deck.iter().cloned().take(p2))),
    )
  }
  fn _run(&mut self, memo: &mut HashMap<RecursiveCombat, Player>) -> Player {
    let mut prev_states: HashSet<RecursiveCombat> = HashSet::new();
    
    let mut game_winner: Option<Player> = None;
    loop {
      if let Some(score) = memo.get(&self) {
        return score.clone();
      }

      if prev_states.contains(&self) {
        game_winner = Some(Player::Player1);
      }

      if let Some(winner) = game_winner {
        for state in prev_states {
          memo.insert(state, winner.clone());
        }
        return winner;
      }

      prev_states.insert(self.clone());

      let p1 = self.0.0.pop_front().unwrap();
      let p2 = self.1.0.pop_front().unwrap();

      let round_winner = if self.0.0.len() < p1 || self.1.0.len() < p2 {
        if p1 > p2 { Player::Player1 } else { Player::Player2 }
      } else {
        self.subgame(p1, p2)._run(memo)
      };
      let round_loser = match round_winner {
        Player::Player1 => { self.0.0.push_back(p1); self.0.0.push_back(p2); &self.1 },
        Player::Player2 => { self.1.0.push_back(p2); self.1.0.push_back(p1); &self.0 },
      };

      if round_loser.0.len() == 0 {
        game_winner = Some(round_winner);
      }
    }
  }

  pub fn run(&mut self) -> (Player, usize) {
    let mut memo = HashMap::new();
    let winner = self._run(&mut memo);
    let deck = match winner {
      Player::Player1 => &self.0,
      Player::Player2 => &self.1,
    };
    return (winner, deck.score());
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

  #[test]
  fn recursive_example() {
    let mut input: RecursiveCombat = "Player 1:
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
10".parse::<CrabCombat>().unwrap().into();
    let (winner, score) = input.run();
    assert_matches!(winner, Player::Player2);
    assert_eq!(291, score);
  }

  #[test]
  fn infinite_loop() {
let mut input: RecursiveCombat = "Player 1:
43
19

Player 2:
2
29
14".parse::<CrabCombat>().unwrap().into();

    let (winner, _) = input.run();
    assert_matches!(winner, Player::Player1);
  }
}