use crate::utils::color::Color;
use crate::utils::figure::Figure;
use crate::utils::piece::Piece;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Castling {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Castling {
    pub fn new() -> Self {
        Castling {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }

    pub fn castle(&mut self, color: Color) {
        match color {
            Color::W => {
                self.white_kingside = false;
                self.white_queenside = false;
            },
            Color::B => {
                self.black_kingside = false;
                self.black_queenside = false;
            },
        }
    }

    pub fn update(&mut self, figure: Figure) {
        if figure.piece == Piece::R {
            if figure.color == Color::W {
                if figure.coord.idx == 56 {
                    self.white_queenside = false;
                } else if figure.coord.idx == 63 {
                    self.white_kingside = false;
                }
            } else if figure.coord.idx == 0 {
                self.black_queenside = false;
            } else if figure.coord.idx == 7 {
                self.black_kingside = false;
            }
        } else if figure.piece == Piece::K {
            if figure.color == Color::W {
                self.white_queenside = false;
                self.white_kingside = false;
            } else {
                self.black_queenside = false;
                self.black_kingside = false;
            }
        }
    }
}

impl Default for Castling {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for Castling {
    fn from(fen: &str) -> Self {
        Castling {
            white_kingside: fen.contains('K'),
            white_queenside: fen.contains('Q'),
            black_kingside: fen.contains('k'),
            black_queenside: fen.contains('q'),
        }
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // note that order matters.
        let mut ca = "".to_owned();
        if self.white_kingside {
            ca.push('K')
        };
        if self.white_queenside {
            ca.push('Q')
        };
        if self.black_kingside {
            ca.push('k')
        };
        if self.black_queenside {
            ca.push('q')
        };

        // Make all results &str.
        let dash = "-".to_string();
        let ca = ca[..].to_string();

        write!(f, "{}", if ca.is_empty() { dash } else { ca })
    }
}
