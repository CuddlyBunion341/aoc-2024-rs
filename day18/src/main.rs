use std::{fs, str::FromStr};

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
    lines
        .filter_map(|line| match Pos::from_str(line) {
            Ok(pos) => Some(pos),
            Err(_) => None,
        })
        .collect()
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
                row.push(true);
            }
            safe.push(row);
        }

        Memory { safe }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.safe[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.safe[y][x] = value
    }
}

impl ToString for Memory {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for y in 0..self.safe.len() {
            for x in 0..self.safe.first().unwrap().len() {
                let cell = self.get(x, y);
                output += match cell {
                    true => ".",
                    false => "#"
                }
            }

            output += "\n"
        }
        output
    }
}

fn main() {
    let file = fs::read_to_string("./input_smol");
    let input = file.unwrap();
    let positions = parse_positions(&input);
    print!("{}", positions.len());

    let memory = Memory::new(6, 6);
    print!("{}", memory.to_string())
}
