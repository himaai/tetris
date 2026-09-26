use crate::{TermPos, piece::Piece};
use crossterm::{
    QueueableCommand, cursor,
    style::{self, Color, Stylize},
};
use std::io::{self, Write};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub type BoardPos = (i16, i16);

pub struct Board {
    grid: [[Option<Color>; WIDTH]; HEIGHT],
    pub pos: TermPos,
}

impl Board {
    pub fn new(term_size: TermPos) -> Self {
        Self {
            grid: [[None; WIDTH]; HEIGHT],
            pos: Self::calculate_pos(term_size),
        }
    }

    pub fn calculate_pos((term_w, term_h): TermPos) -> TermPos {
        (
            (term_w - WIDTH as u16 * 2) / 2,
            (term_h - HEIGHT as u16) / 2,
        )
    }

    pub fn draw(&self, buff: &mut impl Write) -> io::Result<()> {
        let (x, y) = (self.pos.0, self.pos.1);
        buff.queue(cursor::MoveTo(x - 1, y - 1))?;
        buff.queue(style::Print("╔════════════════════╗"))?;

        for l in 0..HEIGHT {
            buff.queue(cursor::MoveTo(x - 1, y + l as u16))?;
            buff.queue(style::Print("║"))?;
            for c in 0..WIDTH {
                draw_cell(buff, self.grid[l][c])?;
            }
            buff.queue(style::Print("║"))?;
        }

        buff.queue(cursor::MoveTo(x - 1, y + HEIGHT as u16))?;
        buff.queue(style::Print("╚════════════════════╝"))?;

        Ok(())
    }

    fn valid(l: i16, c: i16) -> bool {
        l >= 0 && l < HEIGHT as i16 && c >= 0 && c < WIDTH as i16
    }

    pub fn check_piece(&self, piece: &Piece) -> bool {
        let size = piece.shape().size;
        let (l, c) = piece.board_pos;
        for i in 0..size {
            for j in 0..size {
                if !piece.get(i, j) {
                    continue;
                }

                if !Board::valid(l + i as i16, c + j as i16) {
                    return false;
                }

                if let Some(_) = self.grid[(l + i as i16) as usize][(c + j as i16) as usize] {
                    return false;
                }
            }
        }
        true
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        let size = piece.shape().size;
        let (l, c) = piece.board_pos;
        for i in 0..size {
            for j in 0..size {
                if piece.get(i, j) {
                    self.grid[(l + i as i16) as usize][(c + j as i16) as usize] =
                        Some(piece.shape().color);
                }
            }
        }
    }

    pub fn prune(&mut self) -> u64 {
        let mut j = HEIGHT;
        let mut counter = 0;
        for i in (0..HEIGHT).rev() {
            if self.grid[i].contains(&None) {
                j -= 1;
                self.grid[j] = self.grid[i];
            } else {
                counter += 1;
            }
        }
        counter
    }
}

pub fn draw_cell(buff: &mut impl Write, color: Option<Color>) -> io::Result<()> {
    if let Some(color) = color {
        buff.queue(style::PrintStyledContent("[]".with(color)))?;
    } else {
        buff.queue(style::Print(" ."))?;
    }
    Ok(())
}
