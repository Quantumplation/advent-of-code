use std::{rc::Rc, str::FromStr};
use intrusive_collections::{KeyAdapter, LinkedListLink, RBTreeLink, intrusive_adapter};
use intrusive_collections::{LinkedList, RBTree};

pub mod part1 {
  use super::*;
  use anyhow::*;
  pub fn solve(game: CupGame) -> Result<String> {
    let mut game = game;
    game.run(100);
    let cups: Vec<_> = game.take_from(1, game.size).iter().skip(1).map(|i| i.to_string()).collect();
    Ok(cups.join(""))
  }
}

pub mod part2 {
  use super::*;
  use anyhow::*;
  pub fn solve(game: String) -> Result<usize> {
    let mut game = CupGame::from(game.as_str(), 1000000);
    game.run(10000000);
    let cups: Vec<_> = game.take_from(1, 3).iter().cloned().skip(1).collect();
    Ok(cups[0] * cups[1])
  }
}

#[derive(Debug)]
pub struct Cup {
  val: usize,
  cup_neighbors: LinkedListLink,
  cup_tree_pos: RBTreeLink,
}

intrusive_adapter!(CupRing = Rc<Cup>: Cup { cup_neighbors: LinkedListLink });
intrusive_adapter!(CupTree = Rc<Cup>: Cup { cup_tree_pos: RBTreeLink });
impl<'a> KeyAdapter<'a> for CupTree {
  type Key = usize;
  fn get_key(&self, x: &'a Cup) -> usize { x.val }
}

#[derive(Debug)]
pub struct CupGame {
  cup_ring: LinkedList<CupRing>,
  cups: RBTree<CupTree>,
  size: usize,
}

impl FromStr for CupGame {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(CupGame::from(s, 0))
  }
}

impl CupGame {

  pub fn from(s: &str, cup_lower_bound: usize) -> Self {
    let mut list = LinkedList::new(CupRing::new());
    let mut tree = RBTree::new(CupTree::new());

    let mut size = 0;
    let mut max = 0;
    for digit in s.split("").filter(|&s| !s.is_empty()).map(|s| s.parse::<usize>().unwrap()) {
      size += 1;
      if max < digit {
        max = digit;
      }
      let cup = Rc::new(Cup {
        val: digit,
        cup_neighbors: LinkedListLink::new(),
        cup_tree_pos: RBTreeLink::new(),
      });

      list.push_back(cup.clone());
      tree.insert(cup.clone());
    }

    while size < cup_lower_bound {
      size += 1;
      max += 1;
      let cup = Rc::new(Cup {
        val: max,
        cup_neighbors: LinkedListLink::new(),
        cup_tree_pos: RBTreeLink::new(),
      });
      list.push_back(cup.clone());
      tree.insert(cup.clone());
    }

    CupGame {
      cup_ring: list,
      cups: tree,
      size,
    }
  }

  pub fn step(&mut self) {
    let mut cursor = self.cup_ring.front_mut();
    let mut dest = cursor.get().unwrap().val;
    cursor.move_next();
    let a = cursor.remove().unwrap();
    let b = cursor.remove().unwrap();
    let c = cursor.remove().unwrap();

    loop {
      dest = if dest == 1 { self.size } else { dest - 1 };
      if dest != a.val && dest != b.val && dest != c.val {
        break;
      }
    }
    let dest_cursor = self.cups.find(&dest);
    // SAFETY(pi): cursor_mut_from_ptr needs something *in* the collection, and the above loop ensures we
    // don't look for something we've removed
    let mut dest_cursor = unsafe { self.cup_ring.cursor_mut_from_ptr(dest_cursor.get().unwrap()) };

    // Insert in reverse order because the cursor doesn't move
    dest_cursor.insert_after(c);
    dest_cursor.insert_after(b);
    dest_cursor.insert_after(a);

    // Move the current element to the back
    let curr = self.cup_ring.pop_front().unwrap();
    self.cup_ring.push_back(curr);

  }

  pub fn run(&mut self, steps: usize) {
    for _ in 0..steps {
      self.step();
    }
  }

  pub fn take_from(&self, start: usize, count: usize) -> Vec<usize> {
    let mut result = vec![];
    let start = self.cups.find(&start);
    // SAFETY(pi): CupTree and CupRing should always stay in sync
    let mut current = unsafe { self.cup_ring.cursor_from_ptr(start.get().unwrap()) };
    for _ in 0..count {
      result.push(current.get().unwrap().val);
      current.move_next();
      if current.is_null() {
        current.move_next();
      }
    }
    return result;
  }
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;

  use super::*;
  #[test]
  pub fn example() {
    let input = "389125467".parse::<CupGame>();
    assert_matches!(input, Ok(_));
    let mut input = input.unwrap();
    assert_eq!(vec![3,8,9,1,2,5,4,6,7], input.take_from(3, 9));
    input.step();
    assert_eq!(vec![2,8,9,1,5,4,6,7,3], input.take_from(2, 9));
    input.run(9);
    assert_eq!(vec![1,9,2,6,5,8,3,7,4], input.take_from(1, 9));
    input.run(90);
    assert_eq!(vec![1,6,7,3,8,4,5,2,9], input.take_from(1, 9));
  }
}