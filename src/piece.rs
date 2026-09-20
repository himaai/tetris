use crossterm::style::Color;

use crate::board::{self, Board};

#[derive(Copy, Clone)]
pub enum Shape {
    O,
    S,
    Z,
    T,
    L,
    J,
    I,
}

#[derive(Clone)]
pub struct Piece {
    size: usize,
    matrix: Box<[bool]>,
    l: usize,
    c: usize,
    color: Color,
}

impl Piece {
    pub fn new(shape: Shape) -> Self {
        match shape {
            Shape::O => Self {
                size: 2,
                matrix: Box::new([true, true, true, true]),
                l: 0,
                c: 4,
                color: Color::Yellow,
            },
            Shape::S => Self {
                size: 3,
                matrix: Box::new([false, true, true, true, true, false, false, false, false]),
                l: 0,
                c: 3,
                color: Color::Green,
            },
            Shape::Z => Self {
                size: 3,
                matrix: Box::new([true, true, false, false, true, true, false, false, false]),
                l: 0,
                c: 3,
                color: Color::Red,
            },
            Shape::T => Self {
                size: 3,
                matrix: Box::new([false, true, false, true, true, true, false, false, false]),
                l: 0,
                c: 3,
                color: Color::Magenta,
            },
            Shape::L => Self {
                size: 3,
                matrix: Box::new([false, false, true, true, true, true, false, false, false]),
                l: 0,
                c: 3,
                color: Color::DarkYellow,
            },
            Shape::J => Self {
                size: 3,
                matrix: Box::new([true, false, false, true, true, true, false, false, false]),
                l: 0,
                c: 3,
                color: Color::Blue,
            },
            Shape::I => Self {
                size: 4,
                matrix: Box::new([
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    true, true, true, true,
                ]),
                l: 0,
                c: 3,
                color: Color::Cyan,
            },
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.l, self.c)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, l: usize, c: usize) -> bool {
        self.matrix[self.size * l + c]
    }

    fn set(&mut self, l: usize, c: usize, set: bool) {
        self.matrix[self.size * l + c] = set;
    }

    pub fn rotate(&mut self) {
        let aux = self.clone();

        for i in 0..self.size {
            for j in 0..self.size {
                self.set(i, j, aux.get(self.size - j - 1, i));
            }
        }
    }

    pub fn try_left(&mut self, board: &mut Board) {
        if self.c == 0 {
            return;
        }
        board.remove_piece(self);
        self.c -= 1;
        if board.check_piece(self) {
            board.add_piece(self);
        } else {
            self.c += 1;
            board.add_piece(self);
        }
    }

    pub fn try_right(&mut self, board: &mut Board) {
        if self.c + self.size == board::WIDTH {
            return;
        }
        board.remove_piece(self);
        self.c += 1;
        if board.check_piece(self) {
            board.add_piece(self);
        } else {
            self.c -= 1;
            board.add_piece(self);
        }
    }

    pub fn try_down(&mut self, board: &mut Board) -> bool {
        if self.l + self.size == board::HEIGHT {
            return false;
        }
        board.remove_piece(self);
        self.l += 1;
        if board.check_piece(self) {
            board.add_piece(self);
            true
        } else {
            self.c -= 1;
            board.add_piece(self);
            false
        }
    }
}
