use std::{fs, io::Chain, str::FromStr, usize};

const INPUT_FILE_PATH: &str = "./input_smol";

#[derive(Debug)]
struct Move {
    repr: char,
    x: i32,
    y: i32,
}

#[derive(Clone)]
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

    fn attempt_move_player(&mut self, player_move: Move) {
        let new_pos = Board::add_vec_to_move(&self.player_position, &player_move);
        let next_cell = self.get(new_pos.x, new_pos.y).unwrap();

        match next_cell {
            Cell::PLAYER => panic!("Something went terribly wrong"),
            Cell::EMPTY => {
                self.set(self.player_position.x, self.player_position.y, Cell::EMPTY);
                self.set(new_pos.x, new_pos.y, Cell::PLAYER);
                self.player_position = new_pos;

                println!("Moved player")
            }
            Cell::BOX => {
                let chain_length = self.get_movable_box_chain_length(&new_pos, &player_move, 0);

                if chain_length > 0 {
                    self.set(self.player_position.x, self.player_position.y, Cell::EMPTY);
                    self.set(new_pos.x, new_pos.y, Cell::PLAYER);
                    self.player_position = new_pos.clone();
                }

                let mut end_of_chain = new_pos.clone();

                for _ in 0..chain_length {
                    end_of_chain = Board::add_vec_to_move(&end_of_chain, &player_move);
                }

                self.set(end_of_chain.x, end_of_chain.y, Cell::BOX);

                println!("Moving chain: {}", chain_length)
            }
            Cell::WALL => {
                println!("Wall collision")
            }
        }
    }

    fn add_vec_to_move(vec: &Vec2, mv: &Move) -> Vec2 {
        Vec2 {
            x: (vec.x as i32 + mv.x) as usize,
            y: (vec.y as i32 + mv.y) as usize,
        }
    }

    fn get_movable_box_chain_length(&self, current: &Vec2, move_vector: &Move, count: i32) -> i32 {
        let current_cell = self.get(current.x, current.y).unwrap();

        let next_position = Board::add_vec_to_move(&current, &move_vector);

        match current_cell {
            Cell::WALL => 0,
            Cell::PLAYER => panic!("Something went terribly wrong"),
            Cell::BOX => self.get_movable_box_chain_length(&next_position, &move_vector, count + 1),
            Cell::EMPTY => count,
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
                grid.set(x, y, cell.clone());

                if cell == Cell::PLAYER {
                    grid.player_position.x = x;
                    grid.player_position.y = y;
                }
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

fn parse_moves(string: &str) -> Vec<Move> {
    string
        .chars()
        .into_iter()
        .filter_map(|char| match parse_move(char) {
            Ok(player_move) => Some(player_move),
            Err(..) => None,
        })
        .collect()
}

fn parse_move(char: char) -> Result<Move, &'static str> {
    match char {
        '>' => Ok(Move {
            repr: '>',
            x: 1,
            y: 0,
        }),
        '<' => Ok(Move {
            repr: '<',
            x: -1,
            y: 0,
        }),
        'v' => Ok(Move {
            repr: 'v',
            x: 0,
            y: 1,
        }),
        '^' => Ok(Move {
            repr: '^',
            x: 0,
            y: -1,
        }),
        _ => Err("Could not parse move"),
    }
}

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE_PATH).expect("Should have been able to read the file");

    let mut split = contents.split("\n\n");

    let board_part = split.next().unwrap();
    let moves_part = split.next().unwrap();

    let mut board = Board::from_str(board_part).unwrap();
    let moves = parse_moves(moves_part);

    println!("");
    println!("Initial state:");
    board.print();

    moves.into_iter().for_each(|player_move| {
        println!("");
        println!("Move {}:", player_move.repr);
        board.attempt_move_player(player_move);
        board.print();
    });
}
