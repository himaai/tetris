mod board;
mod piece;

use crossterm::{
    QueueableCommand,
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{Rng, seq::SliceRandom};
use std::{
    io::{self, Cursor, Result, Write, stdout},
    thread,
    time::{Duration, Instant},
};

use board::Board;
use piece::{Piece, Shape};

use crate::board::WIDTH;

const FPS: u64 = 30;
const FRAME_TIME: Duration = Duration::from_millis(1000 / FPS);

pub type TermPos = (u16, u16);

struct Bag {
    list: [&'static Shape; 7],
    read: usize,
}

impl Bag {
    pub fn genereate(rng: &mut impl rand::Rng) -> Self {
        let mut list = [
            &piece::O_SHAPE,
            &piece::S_SHAPE,
            &piece::Z_SHAPE,
            &piece::T_SHAPE,
            &piece::L_SHAPE,
            &piece::J_SHAPE,
            &piece::I_SHAPE,
        ];
        list.shuffle(rng);
        Self { list, read: 0 }
    }

    pub fn next(&mut self, rng: &mut impl rand::Rng) -> &'static Shape {
        if self.read == 7 {
            *self = Self::genereate(rng);
        }
        self.read += 1;
        self.list[self.read - 1]
    }
}

struct Game {
    board: Board,
    active_piece: Piece,
    bag: Bag,
    speed: Duration,
    timer: Duration,
    pub is_over: bool,
    level: u64,
    pub score: u64,
    cleared_cnt: u64,
}

fn calculate_speed(level: u64) -> Duration {
    Duration::from_secs_f64((0.8 - ((level as f64 - 1.0) * 0.007)).powi(level as i32 - 1))
}

impl Game {
    fn init(rng: &mut impl rand::Rng, term_size: TermPos) -> Self {
        let mut bag = Bag::genereate(rng);
        let active_piece = Piece::new(bag.next(rng));

        Self {
            board: Board::new(term_size),
            timer: Duration::ZERO,
            speed: calculate_speed(0),
            is_over: false,
            active_piece,
            bag,
            level: 0,
            score: 0,
            cleared_cnt: 0,
        }
    }

    fn draw(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.board.draw(stdout)?;
        self.active_piece.draw(stdout, &self.board)?;
        let (x, y) = (self.board.pos.0 + WIDTH as u16 * 2 + 3, self.board.pos.1);
        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(Print(format!("Score: {}", self.score)))?
            .queue(cursor::MoveTo(x, y + 1))?
            .queue(Print(format!("Level: {}", self.level)))?
            .queue(cursor::MoveTo(x, y + 2))?
            .queue(Print(format!("Lines: {}", self.cleared_cnt)))?
            .queue(cursor::MoveTo(x, y + 4))?
            .queue(Print("q - quit, p - pause"))?
            .queue(cursor::MoveTo(x, y + 5))?
            .queue(Print("move: h - left, l - rigth"))?
            .queue(cursor::MoveTo(x, y + 6))?
            .queue(Print("rotate: k - right, K - left"))?
            .queue(cursor::MoveTo(x, y + 7))?
            .queue(Print("drop: j - soft, J - hard"))?;
        Ok(())
    }

    fn handle_input(&mut self, event: &Event, stdout: &mut impl Write) -> Result<()> {
        match event {
            Event::Resize(x, y) => {
                stdout.queue(Clear(ClearType::All))?;
                self.board.pos = Board::calculate_pos((*x, *y));
            }
            Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: _,
                state: _,
            }) => match code {
                KeyCode::Char('q') => {
                    self.is_over = true;
                }
                KeyCode::Char('h') => {
                    self.active_piece.try_move(Piece::left, &mut self.board);
                }
                KeyCode::Char('l') => {
                    self.active_piece.try_move(Piece::right, &mut self.board);
                }
                KeyCode::Char('j') => {
                    if self.active_piece.try_move(Piece::down, &mut self.board) {
                        self.timer = Duration::ZERO;
                    }
                }
                KeyCode::Char('k') => {
                    self.active_piece
                        .try_move(Piece::rotate_right, &mut self.board);
                }
                KeyCode::Char('K') => {
                    self.active_piece
                        .try_move(Piece::rotate_left, &mut self.board);
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, rng: &mut impl Rng) {
        self.timer += FRAME_TIME;
        if self.timer < self.speed {
            return;
        }
        self.timer -= self.speed;

        if self.active_piece.try_move(Piece::down, &mut self.board) {
            return;
        };
        self.board.add_piece(&self.active_piece);

        let cleared = self.board.prune();
        match cleared {
            1 => self.score += 40 * (self.level + 1),
            2 => self.score += 100 * (self.level + 1),
            3 => self.score += 300 * (self.level + 1),
            4 => self.score += 1200 * (self.level + 1),
            _ => {}
        };
        self.cleared_cnt += cleared;
        if self.cleared_cnt >= 10 {
            self.cleared_cnt -= 10;
            self.level += 1;
            self.speed = calculate_speed(self.level);
        }

        self.active_piece = Piece::new(self.bag.next(rng));
        if !self.board.check_piece(&self.active_piece) {
            self.is_over = true;
        }
    }
}

fn run(stdout: &mut impl Write) -> io::Result<u64> {
    let mut rng = rand::rng();
    let mut game = Game::init(&mut rng, terminal::size()?);
    loop {
        let timer = Instant::now();

        game.draw(stdout)?;
        stdout.flush()?;

        while event::poll(Duration::ZERO)? {
            game.handle_input(&event::read()?, stdout)?;
        }
        game.update(&mut rng);
        if game.is_over {
            break Ok(game.score);
        }

        if timer.elapsed() < FRAME_TIME {
            thread::sleep(FRAME_TIME - timer.elapsed());
        }
    }
}

fn setup_terminal(stdout: &mut impl Write) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    stdout.queue(EnterAlternateScreen)?;
    stdout.queue(Clear(terminal::ClearType::All))?;
    stdout.queue(Hide)?;
    stdout.flush()?;
    Ok(())
}

fn restore_terminal(stdout: &mut impl Write) -> io::Result<()> {
    stdout.queue(Show)?;
    stdout.queue(Clear(terminal::ClearType::All))?;
    stdout.queue(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    stdout.flush()?;
    Ok(())
}

fn main() {
    let mut stdout = stdout();
    setup_terminal(&mut stdout).unwrap();
    let result = run(&mut stdout);
    restore_terminal(&mut stdout).unwrap();
    match result {
        Ok(score) => println!("You earned {} points", score),
        Err(err) => println!("The game crashed with err: {err}"),
    }
}
