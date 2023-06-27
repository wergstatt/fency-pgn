use crate::utils::color::Color;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Piece {
    P,
    R,
    N,
    B,
    Q,
    K,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<char> for Piece {
    fn from(c: char) -> Self {
        match c {
            'r' | 'R' => Piece::R,
            'n' | 'N' => Piece::N,
            'b' | 'B' => Piece::B,
            'q' | 'Q' => Piece::Q,
            'k' | 'K' => Piece::K,
            _ => Piece::P, // ToDo: Note that _ assumes correct values.
        }
    }
}

impl Piece {
    pub fn to_char(self, color: Color) -> char {
        if color == Color::W {
            match self {
                Piece::R => 'R',
                Piece::N => 'N',
                Piece::B => 'B',
                Piece::Q => 'Q',
                Piece::K => 'K',
                Piece::P => 'P',
            }
        } else {
            match self {
                Piece::R => 'r',
                Piece::N => 'n',
                Piece::B => 'b',
                Piece::Q => 'q',
                Piece::K => 'k',
                Piece::P => 'p',
            }
        }
    }
}
