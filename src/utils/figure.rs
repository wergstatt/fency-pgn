use crate::utils::color::Color;
use crate::utils::coord::Coord;
use crate::utils::piece::Piece;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use test::Bencher;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Figure {
    pub color: Color,
    pub coord: Coord,
    pub piece: Piece,
}

impl Figure {
    pub fn to_char(self) -> char {
        self.piece.to_char(self.color)
    }

    pub fn move_to(self, coord: &Coord) -> Self {
        Figure {
            color: self.color,
            coord: coord.clone(),
            piece: self.piece,
        }
    }
}

impl From<&str> for Figure {
    /// Derive a figure from some minimal information about piece and coord, plus the color encoded
    /// in whether the piece is upper- or lowercase, e.g. Nc1 vs ng8.
    fn from(figstr: &str) -> Self {
        let pchar = figstr.clone().chars().next().unwrap();

        return Figure {
            color: match pchar.is_uppercase() {
                true => Color::W,
                false => Color::B,
            },
            coord: Coord::from(&figstr[1..3]),
            piece: Piece::from(pchar),
        };
    }
}

impl Display for Figure {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.piece.to_char(self.color), self.coord)
    }
}

#[test]
fn check_figure_from() {
    assert_eq!(
        Figure::from("Ba3"),
        Figure {
            color: Color::W,
            coord: Coord::from("a3"),
            piece: Piece::B,
        }
    );

    assert_eq!(
        Figure::from("na3"),
        Figure {
            color: Color::B,
            coord: Coord::from("a3"),
            piece: Piece::N,
        }
    );
}

#[bench]
fn bench_figure_creation(b: &mut Bencher) {
    b.iter(|| {
        Vec::from([
            Figure::from("ra8"),
            Figure::from("nb8"),
            Figure::from("bc8"),
            Figure::from("qd8"),
            Figure::from("ke8"),
            Figure::from("bf8"),
            Figure::from("ng8"),
            Figure::from("rh8"),
            Figure::from("pa7"),
            Figure::from("pb7"),
            Figure::from("pc7"),
            Figure::from("pd7"),
            Figure::from("pe7"),
            Figure::from("pf7"),
            Figure::from("pg7"),
            Figure::from("ph7"),
        ])
    });
}
