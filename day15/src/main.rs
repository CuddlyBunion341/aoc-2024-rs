use std::fs;

fn main() {
    let contents = fs::read_to_string("./input")
        .expect("Should have been able to read the file");

    let mut split = contents.split("\n\n");

    let board_part = split.next().unwrap();
    let moves_part = split.next().unwrap();

    print!("{}", board_part);
    print!("{}", moves_part);
}
