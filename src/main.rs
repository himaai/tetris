mod board;
mod piece;

use crossterm::{
    QueueableCommand,
    cursor::{Hide, Show},
    event::{self, Event},
    // execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{Write, stdout},
    thread,
    time::{Duration, Instant},
};

use board::Board;
use piece::{Piece, Shape};

const FPS: u64 = 30;
const FRAME_TIME: Duration = Duration::from_millis(1000 / FPS);

fn run(stdout: &mut impl Write) -> Result<(), Box<dyn Error>> {
    // Setup State
    let mut board = Board::new();
    let mut active_piece = Piece::new(Shape::L);
    board.add_piece(&active_piece);

    let mut counter = Duration::ZERO;
    loop {
        let start = Instant::now();

        // Draw
        stdout.queue(Clear(ClearType::All))?;
        board.draw(stdout)?;
        stdout.flush()?;

        // Handle Input
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(_) => break Ok(()),
                _ => {}
            }
        }

        // Update State
        if counter >= Duration::from_secs(1) {
            board.remove_piece(&active_piece);
            active_piece.rotate();
            board.add_piece(&active_piece);
            counter = Duration::ZERO;
        }
        counter += FRAME_TIME;

        let end = Instant::now();
        thread::sleep(FRAME_TIME - (end - start));
    }
}

fn setup_terminal(stdout: &mut impl Write) -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    stdout.queue(EnterAlternateScreen)?;
    stdout.queue(Hide)?;
    stdout.flush()?;
    Ok(())
}

fn restore_terminal(stdout: &mut impl Write) -> Result<(), Box<dyn Error>> {
    terminal::disable_raw_mode()?;
    stdout.queue(LeaveAlternateScreen)?;
    stdout.queue(Show)?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    setup_terminal(&mut stdout)?;
    let result = run(&mut stdout);
    restore_terminal(&mut stdout)?;
    result
}
