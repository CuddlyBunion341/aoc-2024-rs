use std::{fs, str::FromStr};

struct Vec2 {
    x: u32,
    y: u32,
}

enum Cell {
    PLAYER,
    WALL,
    BOX,
    EMPTY
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        format!("{}", match self {
            Self::EMPTY => "",
            Self::BOX => "O",
            Self::PLAYER => "@",
            Self::WALL => "#",
        })
    }
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(input: &str) -> Result<Cell, Self::Err> {
        match input {
            "#"  => Ok(Cell::WALL),
            "@"  => Ok(Cell::PLAYER),
            "O"  => Ok(Cell::BOX),
            " " => Ok(Cell::EMPTY),
            _      => Err(()),
        }
    }
}

struct Grid {
    data: Vec<Vec<Cell>>,
    player_position: Vec2
}

impl Grid {
    fn get(x: u32, y: u32) {

    }
}

fn extract_grid(grid_part: &str) -> Vec<String> {
    let lines: Vec<String> = grid_part.split("\n").collect();

    lines.collect()

}

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("Should have been able to read the file");

    let mut split = contents.split("\n\n");

    let board_part = split.next().unwrap();
    let moves_part = split.next().unwrap();

    print!("{}", board_part);
    print!("{}", moves_part);
}
