mod board;
mod piece;

use crossterm::{
    QueueableCommand,
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{Rng, seq::SliceRandom};
use std::{
    error::Error,
    io::{self, Write, stdout},
    thread,
    time::{Duration, Instant},
};

use board::Board;
use piece::{Piece, Shape};

const FPS: u64 = 30;
const FRAME_TIME: Duration = Duration::from_millis(1000 / FPS);

struct Bag {
    list: [Shape; 7],
    read: usize,
}

impl Bag {
    pub fn genereate(rng: &mut impl rand::Rng) -> Self {
        use Shape as Sh;
        let mut list = [Sh::O, Sh::S, Sh::Z, Sh::T, Sh::L, Sh::J, Sh::I];
        list.shuffle(rng);
        Self { list, read: 0 }
    }

    pub fn next(&mut self, rng: &mut impl rand::Rng) -> Shape {
        if self.read == 7 {
            *self = Self::genereate(rng);
        }
        self.read += 1;
        self.list[self.read - 1]
    }
}

const TICK: Duration = Duration::from_secs(1);

struct Game {
    board: Board,
    active_piece: Piece,
    bag: Bag,
    timer: Duration,
    pub is_over: bool,
    level: u64,
    pub score: u64,
}

impl Game {
    fn init(rng: &mut impl rand::Rng) -> Self {
        let mut bag = Bag::genereate(rng);
        let active_piece = Piece::new(bag.next(rng));
        let mut board = Board::new();
        board.add_piece(&active_piece);
        Self {
            board,
            timer: Duration::ZERO,
            is_over: false,
            active_piece,
            bag,
            level: 0,
            score: 0,
        }
    }

    fn draw(&self, stdout: &mut impl Write) -> io::Result<()> {
        self.board.draw(stdout)
    }

    fn handle_input(&mut self, event: &Event) {
        let Event::Key(KeyEvent {
            code,
            modifiers: _,
            kind: _,
            state: _,
        }) = event
        else {
            return;
        };
        match code {
            KeyCode::Char('q') => {
                self.is_over = true;
            }
            KeyCode::Char('h') => {
                self.active_piece.try_left(&mut self.board);
            }
            KeyCode::Char('j') => {
                if self.active_piece.try_down(&mut self.board) {
                    self.timer = Duration::ZERO;
                }
            }
            KeyCode::Char('k') => {
                self.active_piece.try_rotate(&mut self.board);
            }
            KeyCode::Char('l') => {
                self.active_piece.try_right(&mut self.board);
            }
            _ => {}
        }
    }

    fn update(&mut self, rng: &mut impl Rng) {
        self.timer += FRAME_TIME;
        if self.timer < TICK {
            return;
        }
        self.timer -= TICK;

        if self.active_piece.try_down(&mut self.board) == false {
            match self.board.prune() {
                1 => self.score += 40 * (self.level + 1),
                2 => self.score += 100 * (self.level + 1),
                3 => self.score += 300 * (self.level + 1),
                4 => self.score += 1200 * (self.level + 1),
                _ => {}
            };

            self.active_piece = Piece::new(self.bag.next(rng));
            if !self.board.check_piece(&self.active_piece) {
                self.is_over = true;
            }
            self.board.add_piece(&self.active_piece);
        };
    }
}

fn run(stdout: &mut impl Write) -> Result<u64, Box<dyn Error>> {
    let mut rng = rand::rng();
    let mut game = Game::init(&mut rng);
    loop {
        let start = Instant::now();

        game.draw(stdout)?;
        stdout.flush()?;

        while event::poll(Duration::ZERO)? {
            game.handle_input(&event::read()?);
        }

        game.update(&mut rng);

        if game.is_over {
            break Ok(game.score);
        }

        if start.elapsed() < FRAME_TIME {
            thread::sleep(FRAME_TIME - start.elapsed());
        }
    }
}

fn setup_terminal(stdout: &mut impl Write) -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    stdout.queue(EnterAlternateScreen)?;
    stdout.queue(Clear(terminal::ClearType::All))?;
    stdout.queue(Hide)?;
    stdout.flush()?;
    Ok(())
}

fn restore_terminal(stdout: &mut impl Write) -> Result<(), Box<dyn Error>> {
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
