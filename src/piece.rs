use crate::{
    TermPos,
    board::{self, Board, BoardPos},
};
use crossterm::{
    QueueableCommand, cursor,
    style::{Color, PrintStyledContent, Stylize},
};
use std::io::{Result, Write};

#[derive(Copy, Clone)]
pub enum Rotation {
    N,
    E,
    S,
    W,
}

#[derive(Clone)]
pub struct Shape {
    grid: [[u8; 4]; 4],
    pub size: usize,
    pub color: Color,
}

#[derive(Clone)]
pub struct Piece {
    shape: &'static Shape,
    pub board_pos: BoardPos,
    pub rotation: Rotation,
}

pub const O_SHAPE: Shape = Shape {
    size: 2,
    grid: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: Color::Yellow,
};
pub const S_SHAPE: Shape = Shape {
    size: 3,
    grid: [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: Color::Green,
};
pub const Z_SHAPE: Shape = Shape {
    size: 3,
    grid: [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: Color::Red,
};
pub const T_SHAPE: Shape = Shape {
    size: 3,
    grid: [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: Color::Magenta,
};
pub const L_SHAPE: Shape = Shape {
    size: 3,
    grid: [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: Color::DarkYellow,
};
pub const J_SHAPE: Shape = Shape {
    size: 3,
    grid: [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: Color::Blue,
};
pub const I_SHAPE: Shape = Shape {
    size: 4,
    grid: [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: Color::Cyan,
};

impl Shape {
    fn get(&self, l: usize, c: usize, rotation: Rotation) -> bool {
        let last = self.size - 1;
        0 != match rotation {
            Rotation::N => self.grid[l][c],
            Rotation::E => self.grid[c][last - l],
            Rotation::S => self.grid[last - l][last - c],
            Rotation::W => self.grid[last - c][l],
        }
    }

    pub fn draw(&self, stdout: &mut impl Write, (x, y): TermPos, rotation: Rotation) -> Result<()> {
        for i in 0..self.size {
            stdout.queue(cursor::MoveTo(x, y + i as u16))?;
            for j in 0..self.size {
                if self.get(i, j, rotation) {
                    stdout.queue(PrintStyledContent("[]".with(self.color)))?;
                } else {
                    stdout.queue(cursor::MoveRight(2))?;
                }
            }
        }
        Ok(())
    }
}

impl Piece {
    pub fn new(shape: &'static Shape) -> Self {
        Self {
            shape,
            board_pos: (0, (board::WIDTH - shape.size) as i16 / 2),
            rotation: Rotation::N,
        }
    }

    pub fn draw(&self, stdout: &mut impl Write, board: &Board) -> Result<()> {
        let (x, y) = board.pos;
        let (l, c) = self.board_pos;
        self.shape.draw(
            stdout,
            ((x as i16 + c * 2) as u16, (y as i16 + l) as u16),
            self.rotation,
        )
    }

    pub fn get(&self, l: usize, c: usize) -> bool {
        self.shape.get(l, c, self.rotation)
    }

    pub fn rotate_right(&mut self) {
        self.rotation = match self.rotation {
            Rotation::N => Rotation::E,
            Rotation::E => Rotation::S,
            Rotation::S => Rotation::W,
            Rotation::W => Rotation::N,
        };
    }

    pub fn rotate_left(&mut self) {
        self.rotation = match self.rotation {
            Rotation::N => Rotation::W,
            Rotation::E => Rotation::N,
            Rotation::S => Rotation::E,
            Rotation::W => Rotation::S,
        };
    }

    pub fn down(&mut self) {
        self.board_pos.0 += 1;
    }

    pub fn left(&mut self) {
        self.board_pos.1 -= 1;
    }

    pub fn right(&mut self) {
        self.board_pos.1 += 1;
    }

    pub fn try_move(&mut self, f: impl FnOnce(&mut Self) -> (), board: &mut Board) -> bool {
        let mut aux = self.clone();
        f(&mut aux);

        if board.check_piece(&aux) {
            *self = aux;
            true
        } else {
            false
        }
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }
}
