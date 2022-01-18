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

    pub fn castle(self, color: Color) -> Self {
        match color {
            Color::W => Castling {
                white_kingside: false,
                white_queenside: false,
                black_kingside: self.black_kingside,
                black_queenside: self.black_queenside,
            },
            Color::B => Castling {
                white_kingside: self.white_kingside,
                white_queenside: self.white_queenside,
                black_kingside: false,
                black_queenside: false,
            },
        }
    }

    pub fn update(self, figure: Figure) -> Self {
        let mut castling = self.clone();

        if figure.piece == Piece::R {
            if figure.color == Color::W {
                if figure.coord.idx == 56 {
                    castling.white_queenside = false;
                } else if figure.coord.idx == 63 {
                    castling.white_kingside = false;
                }
            } else {
                if figure.coord.idx == 0 {
                    castling.black_queenside = false;
                } else if figure.coord.idx == 7 {
                    castling.black_kingside = false;
                }
            }
        } else if figure.piece == Piece::K {
            if figure.color == Color::W {
                castling.white_queenside = false;
                castling.white_kingside = false;
            } else {
                castling.black_queenside = false;
                castling.black_kingside = false;
            }
        }

        castling
    }
}

impl From<&str> for Castling {
    fn from(fen: &str) -> Self {
        return Castling {
            white_kingside: fen.contains('K'),
            white_queenside: fen.contains('Q'),
            black_kingside: fen.contains('k'),
            black_queenside: fen.contains('q'),
        };
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
        let ca = (&ca[..]).to_string();

        write!(f, "{}", if ca.len() == 0 { dash } else { ca })
    }
}
