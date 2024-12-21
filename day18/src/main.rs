use std::{fs, io::stdout, str::FromStr};

#[derive(Clone)]
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
    pub fn new(width: usize, height: usize) -> Memory {
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
        if y > self.safe.len() {
            false
        } else if x > self.safe.first().unwrap().len() {
            false
        } else {
            self.safe[y][x]
        }
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
                    false => "#",
                }
            }

            output += "\n"
        }
        output
    }
}

fn solve_maze(start: &Pos, stop: &Pos, memory: &Memory) -> Vec<Pos> {
    let current = Pos {
        x: start.x,
        y: start.y,
    };
    let visited = Vec::new();

    solve(visited, &current, &stop, &memory)
}

fn vec_includes_pos(vec: Vec<Pos>, pos: &Pos) -> bool {
    vec.into_iter().fold(false, |acc,val| {
        acc || (val.x == pos.x && val.y == pos.y)
    })
}

fn solve(visited: Vec<Pos>, current: &Pos, stop: &Pos, memory: &Memory) -> Vec<Pos> {
    if current.x == stop.x && current.y == stop.y {
        return visited;
    }

    if memory.get(current.x, current.y) {
        return visited;
    }

    let mut visited = visited.clone();
    visited.push(current.clone());

    solve(
        visited.clone(),
        &Pos {
            x: current.x + 1,
            y: current.y,
        },
        stop,
        memory,
    );

    solve(
        visited.clone(),
        &Pos {
            x: current.x,
            y: current.y + 1,
        },
        stop,
        memory,
    );

    Vec::new()
}

fn main() {
    let file = fs::read_to_string("./input_smol");
    let input = file.unwrap();
    let positions = parse_positions(&input);
    println!("{}", positions.len());

    let width = 7;
    let height = 7;

    let starting_position = Pos { x: 0, y: 0 };

    let exit_position = Pos {
        x: width - 1,
        y: height - 1,
    };

    let mut memory = Memory::new(width, height);

    positions.into_iter().for_each(|position| {
        memory.set(position.x, position.y, false);
    });

    println!("{}", memory.to_string());

    let steps = solve_maze(&starting_position, &exit_position, &memory);

    steps.into_iter().for_each(|step| {
        print!("{}x{}", step.x, step.y);
    });
}
