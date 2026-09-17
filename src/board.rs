use crossterm::{
    QueueableCommand, cursor,
    style::{self, Color, ContentStyle, StyledContent},
};
use std::io::{self, Write, stdout};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const X_SCALE: usize = 3;
const Y_SCALE: usize = 2;

pub struct Board {
    grid: [[Option<Color>; HEIGHT]; WIDTH],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[None; HEIGHT]; WIDTH],
        }
    }

    fn draw_cell(x: u16, y: u16, buff: &mut impl Write, style: ContentStyle) -> io::Result<()> {
        buff.queue(cursor::MoveTo(x, y))?;
        buff.queue(style::PrintStyledContent(StyledContent::new(
            style,
            "█".to_string(),
        )))?;
        Ok(())
    }

    pub fn draw(&self, buff: &mut impl Write) -> io::Result<()> {
        for x in 0..X_SCALE * WIDTH {
            for y in 0..Y_SCALE * HEIGHT {
                let i = x / X_SCALE;
                let j = y / Y_SCALE;
                Board::draw_cell(
                    x as u16,
                    y as u16,
                    buff,
                    match self.grid[i][j] {
                        None => ContentStyle {
                            foreground_color: Some(if (x + y) % 2 == 0 {
                                Color::Grey
                            } else {
                                Color::Black
                            }),
                            ..Default::default()
                        },
                        color => ContentStyle {
                            foreground_color: color,
                            ..Default::default()
                        },
                    },
                )?;
            }
        }

        Ok(())
    }
}
