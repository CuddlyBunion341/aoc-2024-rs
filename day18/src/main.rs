use std::{borrow::Borrow, fs, io::stdout, str::FromStr};

struct Pos {
    x: usize,
    y: usize,
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');

        let x = split.next();
        let y = split.next();

        if x.is_some() && y.is_some() {
            Ok(Pos {
                x: x.unwrap().parse::<usize>().unwrap(),
                y: y.unwrap().parse::<usize>().unwrap(),
            })
        } else {
            Err(format!("Could not extract position from '{}'", s))
        }
    }
}

fn parse_positions(input: &str) -> Vec<Pos> {
    let lines = input.split("\n");
    lines.filter_map(|line| {
        match Pos::from_str(line) {
            Ok(pos) => Some(pos),
            Err(_) => None,
        }
    }).collect()
}

struct Memory {
    safe: Vec<Vec<bool>>,
}

impl Memory {
    pub fn new(width: i32, height: i32) -> Memory {
        let mut safe = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(false);
            }
            safe.push(row);
        }

        Memory {
            safe
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.safe[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.safe[y][x] = value
    }
}

impl FromStr for Memory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").collect();

        let x = lines.len();
        let y = lines.first().unwrap().len();

        Ok(Memory::new(x.try_into().unwrap(),y.try_into().unwrap()))
    }
}

fn main() {
    let file = fs::read_to_string("./input");
    let input = file.unwrap();
    let positions = parse_positions(&input);
    print!("{}",positions.len());

    let memory = Memory::from_str(&input);
}
