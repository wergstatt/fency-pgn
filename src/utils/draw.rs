use crate::utils::coord::Coord;
use crate::utils::piece::Piece;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

// A regular expression to decompose a SAN. Note that castling is excluded here.
const SAN_REGEX: &str = "(?P<Piece>[NBRQK])?(?P<RemainderFile>[a-h])?(?P<RemainderRank>[1-8])?(?P<Hit>x)?(?P<Target>[a-h][1-8])=?(?P<PromotesTo>[NBRQK])?(?P<Check>\\+|#)?";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Draw {
    san: String,
    pub target: Coord,
    pub piece: Piece,
    is_check: bool,
    is_checkmate: bool,
    pub is_promo: bool,
    pub is_hit: bool,
    pub promoted_piece: Option<Piece>,
    pub remainder_file: Option<char>,
    pub remainder_rank: Option<char>,
}
impl FromStr for Draw {
    fn from_str(san: &str) -> Result<Self, Self::Err> {
        // Use a regular expression to decompose the SAN (without Castling).
        // ref: https://stackoverflow.com/questions/54259474/convert-regex-captures-into-hashmap-in-rust
        let re_san: Regex = Regex::new(SAN_REGEX).unwrap();
        let captures = re_san.captures(san).unwrap();
        let capture_map: HashMap<&str, &str> = re_san
            .capture_names()
            .flatten()
            .filter_map(|n| Some((n, captures.name(n)?.as_str())))
            .collect();

        // Sort the matching groups into the according parts.
        Ok(Draw {
            san: san.to_string(),

            is_check: san.contains('+') | san.contains('#'),
            is_checkmate: san.contains('#'),
            is_promo: san.contains('='),
            is_hit: san.contains('x'),

            target: Coord::from(*capture_map.get("Target").unwrap()),
            piece: match capture_map.get("Piece") {
                None => Piece::P,
                Some(&p) => Piece::from(p.chars().next().unwrap()),
            },
            promoted_piece: capture_map.get("PromotesTo").map(|&c| Piece::from(c.chars().next().unwrap())),
            remainder_file: capture_map.get("RemainderFile").map(|&c| c.chars().next().unwrap()),
            remainder_rank: capture_map.get("RemainderRank").map(|&c| c.chars().next().unwrap()),
        })
    }

    type Err = String;
}

#[test]
fn check_draw_from_san_pt1() {
    let draw = Draw::from_str("a3").unwrap();

    assert_eq!(draw.target, Coord::from("a3"));
    assert_eq!(draw.piece, Piece::P);
    assert!(!draw.is_check);
    assert!(!draw.is_checkmate);
    assert!(!draw.is_promo);
    assert!(!draw.is_hit);
    assert_eq!(draw.promoted_piece, None);
    assert_eq!(draw.remainder_file, None);
    assert_eq!(draw.remainder_rank, None);
}

#[test]
fn check_draw_from_san_pt2() {
    let draw = Draw::from_str("exd1=Q#").unwrap();

    assert_eq!(draw.target, Coord::from("d1"));
    assert_eq!(draw.piece, Piece::P);
    assert!(draw.is_check);
    assert!(draw.is_checkmate);
    assert!(draw.is_promo);
    assert!(draw.is_hit);
    assert_eq!(draw.promoted_piece, Some(Piece::Q));
    assert_eq!(draw.remainder_file, Some('e'));
    assert_eq!(draw.remainder_rank, None);
}

#[test]
fn check_draw_from_san_pt3() {
    let draw = Draw::from_str("Raxc6+").unwrap();

    assert_eq!(draw.target, Coord::from("c6"));
    assert_eq!(draw.piece, Piece::R);
    assert!(draw.is_check);
    assert!(!draw.is_checkmate);
    assert!(!draw.is_promo);
    assert!(draw.is_hit);
    assert_eq!(draw.promoted_piece, None);
    assert_eq!(draw.remainder_file, Some('a'));
    assert_eq!(draw.remainder_rank, None);
}

#[test]
fn check_draw_from_san_pt4() {
    let draw = Draw::from_str("N1c3").unwrap();

    assert_eq!(draw.target, Coord::from("c3"));
    assert_eq!(draw.piece, Piece::N);
    assert!(!draw.is_check);
    assert!(!draw.is_checkmate);
    assert!(!draw.is_promo);
    assert!(!draw.is_hit);
    assert_eq!(draw.promoted_piece, None);
    assert_eq!(draw.remainder_file, None);
    assert_eq!(draw.remainder_rank, Some('1'));
}
