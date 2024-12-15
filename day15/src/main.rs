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

struct Board {
    size: Vec2,
    data: Vec<Cell>,
    player_position: Vec2,
}

impl Board {
    fn calc_index(&self, x: usize, y: usize) -> usize {
        let index = (y as u32) * self.size.y + (x as u32);
        index as usize
    }

    fn get(&self, x: usize, y: usize) -> Result<&Cell, &'static str> {
        if (x as u32) >= self.size.x || (y as u32) >= self.size.y {
            Err("Index out of grid bounds")
        } else {
            let cell = self.data.get(self.calc_index(x, y)).unwrap();
            Ok(cell)
        }
    }

    fn set(&mut self, x: usize, y: usize, value: Cell) -> bool {
        if (x as u32) >= self.size.x || (y as u32) >= self.size.y {
            false
        } else {
            let index = self.calc_index(x, y);
            self.data[index] = value;
            true
        }
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(input: &str) -> Result<Board, Self::Err> {
        let lines = input.split("\n");

        let mut grid = Board {
            size: Vec2 { x: 0, y: 0 },
            data: Vec::new(),
            player_position: Vec2 { x: 0, y: 0 },
        };

        for (y, line) in lines.into_iter().enumerate() {
            for x in 0..line.len() {
                let cell_string = line.chars().nth(x).unwrap().to_string();
                grid.set(x,y,Cell::from_str(&cell_string).unwrap());
            }
        }

        Ok(grid)
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut string = String::from("");

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                string += &self.get(x as usize, y as usize).unwrap().to_string();
            }
            string += "\n";
        }

        string
    }
}

fn main() {
    let contents = fs::read_to_string("./input").expect("Should have been able to read the file");

    let mut split = contents.split("\n\n");

    let board_part = split.next().unwrap();
    let moves_part = split.next().unwrap();

    let board = Board::from_str(board_part);

    print!("{}", board_part);
    print!("{}", moves_part);
}
