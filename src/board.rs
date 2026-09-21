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

    fn valid(l: i8, c: i8) -> bool {
        l >= 0 && l < HEIGHT as i8 && c >= 0 && c < WIDTH as i8
    }

    pub fn check_piece(&self, piece: &Piece) -> bool {
        let (l, c) = piece.pos();
        let size = piece.size();
        for i in 0..size {
            for j in 0..size {
                if !piece.get(i, j) {
                    continue;
                }

                if !Board::valid(l + i as i8, c + j as i8) {
                    return false;
                }

                if let Some(_) = self.grid[(l + i as i8) as usize][(c + j as i8) as usize] {
                    return false;
                }
            }
        }
        true
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        let (l, c) = piece.pos();
        let size = piece.size();
        for i in 0..size {
            for j in 0..size {
                if piece.get(i, j) {
                    self.grid[(l + i as i8) as usize][(c + j as i8) as usize] = Some(piece.color());
                }
            }
        }
    }

    pub fn remove_piece(&mut self, piece: &Piece) {
        let (l, c) = piece.pos();
        let size = piece.size();
        for i in 0..size {
            for j in 0..size {
                if piece.get(i, j) {
                    self.grid[(l + i as i8) as usize][(c + j as i8) as usize] = None;
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

    pub fn prune(&mut self) -> u8 {
        let mut counter = 0;
        for i in 0..HEIGHT {
            if self.check_line(i) {
                self.prune_line(i);
                counter += 1;
            }
        }
        counter
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
