use std::{fs, str::FromStr, usize};

struct Vec2 {
    x: u32,
    y: u32,
}

enum Cell {
    PLAYER,
    WALL,
    BOX,
    EMPTY,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        format!(
            "{}",
            match self {
                Self::EMPTY => "",
                Self::BOX => "O",
                Self::PLAYER => "@",
                Self::WALL => "#",
            }
        )
    }
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(input: &str) -> Result<Cell, Self::Err> {
        match input {
            "#" => Ok(Cell::WALL),
            "@" => Ok(Cell::PLAYER),
            "O" => Ok(Cell::BOX),
            " " => Ok(Cell::EMPTY),
            _ => Err(()),
        }
    }
}

struct Grid {
    size: Vec2,
    data: Vec<Cell>,
    player_position: Vec2,
}

impl Grid {
    fn calc_index(&self, x: u32, y: u32) -> usize {
        let index = y * self.size.y + x;
        index as usize
    }

    fn get(&self, x: u32, y: u32) -> Result<&Cell, &'static str> {
        if x >= self.size.x || y >= self.size.y {
            Err("Index out of grid bounds")
        } else {
            let cell = self.data.get(self.calc_index(x, y)).unwrap();
            Ok(cell)
        }
    }

    fn set(&mut self, x: u32, y: u32, value: Cell) -> bool {
        if x >= self.size.x || y >= self.size.y {
            false
        } else {
            self.data[self.calc_index(x, y)] = value;
            true
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Grid, Self::Err> {
        let lines = input.split("\n");

        Ok(Grid {
            size: Vec2 { x: 0, y: 0 },
            data: Vec::new(),
            player_position: Vec2 { x: 0, y: 0 },
        })
    }
}

fn main() {
    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");

    let mut split = contents.split("\n\n");

    let board_part = split.next().unwrap();
    let moves_part = split.next().unwrap();

    print!("{}", board_part);
    print!("{}", moves_part);
}
