use crate::piece::Piece;
use crossterm::{
    QueueableCommand, cursor,
    style::{self, Color, Stylize},
};
use std::io::{self, Write};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub struct Board {
    grid: [[Option<Color>; WIDTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[None; WIDTH]; HEIGHT],
        }
    }

    pub fn draw(&self, buff: &mut impl Write) -> io::Result<()> {
        for l in 0..HEIGHT {
            for c in 0..WIDTH {
                let x = (c * 2) as u16;
                let y = l as u16;
                draw_cell(x, y, buff, self.grid[l][c])?;
            }
        }
        Ok(())
    }

    pub fn check_piece(&self, piece: &Piece) -> bool {
        let (l, c) = piece.pos();
        for i in 0..piece.size() {
            for j in 0..piece.size() {
                if let Some(_) = self.grid[l + i][c + j]
                    && piece.get(i, j)
                {
                    return false;
                }
            }
        }
        true
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        for i in 0..piece.size() {
            for j in 0..piece.size() {
                if piece.get(i, j) {
                    let (l, c) = piece.pos();
                    self.grid[l + i][c + j] = Some(piece.color());
                }
            }
        }
    }

    pub fn remove_piece(&mut self, piece: &Piece) {
        let (l, c) = piece.pos();
        for i in 0..piece.size() {
            for j in 0..piece.size() {
                if piece.get(i, j) {
                    self.grid[l + i][c + j] = None;
                }
            }
        }
    }

    fn check_line(&self, l: usize) -> bool {
        for cell in self.grid[l] {
            if let None = cell {
                return false;
            }
        }
        true
    }

    fn prune_line(&mut self, l: usize) {
        self.grid[0] = [None; WIDTH];
        for i in (1..l + 1).rev() {
            self.grid[i] = self.grid[i - 1].clone();
        }
    }

    pub fn prune(&mut self) {
        for i in 0..HEIGHT {
            if self.check_line(i) {
                self.prune_line(i);
            }
        }
    }
}

pub fn draw_cell(x: u16, y: u16, buff: &mut impl Write, color: Option<Color>) -> io::Result<()> {
    buff.queue(cursor::MoveTo(x, y))?;
    if let Some(color) = color {
        buff.queue(style::PrintStyledContent("[]".with(color)))?;
    } else {
        buff.queue(style::Print(" ."))?;
    }
    Ok(())
}
