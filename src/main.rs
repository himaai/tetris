use crossterm::style::ContentStyle;
use crossterm::{
    QueueableCommand, cursor,
    style::{self, ContentStyle, StyledContent},
};
use std::io::{self, Write, stdout};

#[derive(Copy, Clone, Debug)]
enum Cell {
    Empty,
    Cyan,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
}

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const SCALE: usize = 3;

struct Board {
    grid: [[Cell; WIDTH]; HEIGHT],
}

impl Board {
    fn new() -> Self {
        Self {
            grid: [[Cell::Cyan; WIDTH]; HEIGHT],
        }
    }

    fn draw_cell(i: u16, j: u16, buff: &mut impl Write, style: ContentStyle) -> io::Result<()> {
        buff.queue(cursor::MoveTo(i, j))?;
        buff.queue(style::PrintStyledContent(StyledContent {
            style,
            content: "█".to_string(),
        }));
        Ok(())
    }

    fn draw(&self, buff: &mut impl Write) -> io::Result<()> {
        for i in 0..SCALE * HEIGHT {
            for j in 0..SCALE * WIDTH {
                let l = i / SCALE;
                let c = j / SCALE;
                match self.grid[l][c] {
                    Cell::Empty => {}
                    Cell::Cyan => {
                        buff.queue(cursor::MoveTo(i as u16, j as u16))?;
                        buff.queue(style::Print("█".to_string()))?;
                    }
                    Cell::Blue => {
                        buff.queue(cursor::MoveTo(i as u16, j as u16))?;
                        buff.queue(style::Print("█".to_string()))?;
                    }
                    Cell::Orange => {
                        buff.queue(cursor::MoveTo(i as u16, j as u16))?;
                        buff.queue(style::Print("█".to_string()))?;
                    }
                    Cell::Yellow => {
                        buff.queue(cursor::MoveTo(i as u16, j as u16))?;
                        buff.queue(style::Print("█".to_string()))?;
                    }
                    Cell::Green => {
                        buff.queue(cursor::MoveTo(i as u16, j as u16))?;
                        buff.queue(style::Print("█".to_string()))?;
                    }
                    Cell::Purple => {
                        buff.queue(cursor::MoveTo(i as u16, j as u16))?;
                        buff.queue(style::Print("█".to_string()))?;
                    }
                    Cell::Red => {
                        buff.queue(cursor::MoveTo(i as u16, j as u16))?;
                        buff.queue(style::Print("█".to_string()))?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let board = Board::new();
    let mut stdout = stdout();
    board.draw(&mut stdout).unwrap();
    stdout.flush().unwrap();
}
