use crossterm::style::Color;

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
    square: Box<[bool]>,
    x: usize,
    y: usize,
    color: Color,
}

impl Piece {
    pub fn new(shape: Shape) -> Self {
        match shape {
            Shape::O => Self {
                size: 2,
                square: Box::new([true, true, true, true]),
                x: 4,
                y: 0,
                color: Color::Yellow,
            },
            Shape::S => Self {
                size: 3,
                square: Box::new([false, true, true, true, true, false, false, false, false]),
                x: 3,
                y: 0,
                color: Color::Green,
            },
            Shape::Z => Self {
                size: 3,
                square: Box::new([true, true, false, false, true, true, false, false, false]),
                x: 3,
                y: 0,
                color: Color::Red,
            },
            Shape::T => Self {
                size: 3,
                square: Box::new([false, true, false, true, true, true, false, false, false]),
                x: 3,
                y: 0,
                color: Color::Magenta,
            },
            Shape::L => Self {
                size: 3,
                square: Box::new([false, false, true, true, true, true, false, false, false]),
                x: 3,
                y: 0,
                color: Color::DarkYellow,
            },
            Shape::J => Self {
                size: 3,
                square: Box::new([true, false, false, true, true, true, false, false, false]),
                x: 3,
                y: 0,
                color: Color::Blue,
            },
            Shape::I => Self {
                size: 4,
                square: Box::new([
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    true, true, true, true,
                ]),
                x: 3,
                y: 0,
                color: Color::Cyan,
            },
        }
    }

    pub fn get(&self, l: usize, c: usize) -> bool {
        self.square[self.size * l + c]
    }

    fn set(&mut self, l: usize, c: usize, set: bool) {
        self.square[self.size * l + c] = set;
    }

    pub fn rotate(&mut self) {
        let aux = self.clone();

        for i in 0..self.size {
            for j in 0..self.size {
                self.set(i, j, aux.get(j, i));
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..self.size {
            for j in 0..self.size {
                s.push(if self.get(i, j) { '1' } else { '0' });
            }
            s.push('\n');
        }
        s
    }
}
