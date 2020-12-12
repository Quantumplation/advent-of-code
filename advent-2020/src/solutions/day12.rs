use std::str::FromStr;
use anyhow::*;

pub mod part1 {
    use super::*;
    pub fn solve(instructions: Vec<Instruction>) -> Result<u32> {
        let mut boat = Boat { x: 0, y: 0, dir: Instruction::East(0) };
        for i in instructions {
            boat.advance(i, None);
        }
        return Ok((boat.x.abs() + boat.y.abs()) as u32);
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(instructions: Vec<Instruction>) -> Result<u32> {
        let mut boat = BoatWithWaypoint { x: 0, y: 0, wx: 10, wy: 1 };
        for i in instructions {
            boat.advance(i);
        }
        return Ok((boat.x.abs() + boat.y.abs()) as u32);
    }
}

#[derive(Clone)]
pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

pub struct Boat {
    x: i32,
    y: i32,
    dir: Instruction,
}

pub struct BoatWithWaypoint {
    x: i32,
    y: i32,
    wx: i32,
    wy: i32,
}

impl Boat {
    pub fn advance(&mut self, i: Instruction, dist: Option<i32>) {
        match i {
            Instruction::North(d) => {
                self.y += dist.unwrap_or(d);
            },
            Instruction::South(d) => {
                self.y -= dist.unwrap_or(d);
            },
            Instruction::East(d) => {
                self.x += dist.unwrap_or(d);
            },
            Instruction::West(d) => {
                self.x -= dist.unwrap_or(d);
            },
            Instruction::Forward(d) => {
                self.advance(self.dir.clone(), Some(d));
            }
            Instruction::Right(d) => {
                self.dir = match (d, &self.dir) {
                    (90, Instruction::East(_)) | (180, Instruction::North(_)) | (270, Instruction::West(_)) => Instruction::South(0),
                    (90, Instruction::South(_)) | (180, Instruction::East(_)) | (270, Instruction::North(_)) => Instruction::West(0),
                    (90, Instruction::West(_)) | (180, Instruction::South(_)) | (270, Instruction::East(_)) => Instruction::North(0),
                    (90, Instruction::North(_)) | (180, Instruction::West(_)) | (270, Instruction::South(_)) => Instruction::East(0),
                    _ => panic!(),
                }
            }
            Instruction::Left(d) => {
                self.dir = match (d, &self.dir) {
                    (90, Instruction::East(_)) | (180, Instruction::South(_)) | (270, Instruction::West(_)) => Instruction::North(0),
                    (90, Instruction::South(_)) | (180, Instruction::West(_)) | (270, Instruction::North(_)) => Instruction::East(0),
                    (90, Instruction::West(_)) | (180, Instruction::North(_)) | (270, Instruction::East(_)) => Instruction::South(0),
                    (90, Instruction::North(_)) | (180, Instruction::East(_)) | (270, Instruction::South(_)) => Instruction::West(0),
                    _ => panic!()
                }
            }
        }
    }
}

impl BoatWithWaypoint {
    pub fn advance(&mut self, i: Instruction) {
        match i {
            Instruction::North(d) => {
                self.wy += d;
            },
            Instruction::South(d) => {
                self.wy -= d;
            },
            Instruction::East(d) => {
                self.wx += d;
            },
            Instruction::West(d) => {
                self.wx -= d;
            },
            Instruction::Forward(d) => {
                self.x += d * self.wx;
                self.y += d * self.wy;
            }
            Instruction::Left(90) | Instruction::Right(270) => {
                // (2, 1) => (-1, 2)
                let (wx, wy) = (self.wx, self.wy);
                self.wx = -wy;
                self.wy = wx;
            },
            Instruction::Left(180) | Instruction::Right(180) => {
                let (wx, wy) = (self.wx, self.wy);
                self.wx = -wx;
                self.wy = -wy;        
            },
            Instruction::Left(270) | Instruction::Right(90) => {
                // (2, 1) => (1, -2)
                let (wx, wy) = (self.wx, self.wy);
                self.wx = wy;
                self.wy = -wx;
            }
            _ => panic!(),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = (&s[0..1], s[1..].parse::<i32>().unwrap());
        Ok(match dir {
            "N" => Instruction::North(dist),
            "S" => Instruction::South(dist),
            "E" => Instruction::East(dist),
            "W" => Instruction::West(dist),
            "L" => Instruction::Left(dist),
            "R" => Instruction::Right(dist),
            "F" => Instruction::Forward(dist),
            _ => panic!()
        })
    }
}

