use crossterm::style::Color;

use crate::board::Board;

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
    l: i8,
    c: i8,
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
                    false, false, false, false,
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

    pub fn pos(&self) -> (i8, i8) {
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

    pub fn clockwise(&mut self) {
        let aux = self.clone();

        for i in 0..self.size {
            for j in 0..self.size {
                self.set(i, j, aux.get(self.size - j - 1, i));
            }
        }
    }

    pub fn counterclockwise(&mut self) {
        let aux = self.clone();

        for i in 0..self.size {
            for j in 0..self.size {
                self.set(i, j, aux.get(j, self.size() - i - 1));
            }
        }
    }

    pub fn down(&mut self) {
        self.l += 1;
    }

    pub fn left(&mut self) {
        self.c -= 1;
    }

    pub fn right(&mut self) {
        self.c += 1;
    }

    pub fn try_moving(&mut self, f: impl FnOnce(&mut Self) -> (), board: &mut Board) -> bool {
        let mut aux = self.clone();
        f(&mut aux);

        board.remove_piece(self);
        if board.check_piece(&aux) {
            *self = aux;
            board.add_piece(self);
            true
        } else {
            board.add_piece(self);
            false
        }
    }
}
