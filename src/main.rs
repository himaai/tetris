mod board;
mod piece;

use board::Board;
use std::io::{Write, stdout};

fn main() {
    let board = Board::new();
    let mut stdout = stdout();
    board.draw(&mut stdout).unwrap();
    stdout.flush().unwrap();
}
