use std::{fs, str::FromStr, usize};

struct Vec2 {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq)]
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
                Self::EMPTY => ".",
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
            "." => Ok(Cell::EMPTY),
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
    fn new(width: usize, height: usize) -> Board {
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, Cell::EMPTY);

        Board {
            data,
            size: Vec2 {
                x: width,
                y: height,
            },
            player_position: Vec2 { x: 0, y: 0 },
        }
    }

    fn print(&self) {
        print!("{}", self.to_string());
    }

    fn calc_index(&self, x: usize, y: usize) -> usize {
        let index = y * self.size.y + x;
        index as usize
    }

    fn get(&self, x: usize, y: usize) -> Result<&Cell, &'static str> {
        if x >= self.size.x || y >= self.size.y {
            Err("Index out of grid bounds")
        } else {
            let index = self.calc_index(x, y);
            let cell = self.data.get(index).unwrap();
            Ok(cell)
        }
    }

    fn set(&mut self, x: usize, y: usize, value: Cell) -> bool {
        if x >= self.size.x || y >= self.size.y {
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

        let lines2: Vec<&str> = lines.clone().into_iter().collect();
        let height = lines2.len();

        let width = lines2.first().unwrap().len();

        let mut grid = Board::new(width, height);

        for (y, line) in lines.into_iter().enumerate() {
            for x in 0..line.len() {
                let cell_string = line.chars().nth(x).unwrap().to_string();
                let cell = Cell::from_str(&cell_string).unwrap();
                grid.set(x, y, cell);
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
                string += &self.get(x, y).unwrap().to_string();
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

    let board = Board::from_str(board_part).unwrap();
    board.print();

    // print!("{}", board_part);
    // print!("{}", moves_part);
}
