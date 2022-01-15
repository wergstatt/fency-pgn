use crate::utils::color::Color;
use crate::utils::coord::Coord;
use crate::utils::piece::Piece;
use regex::Regex;
use std::collections::HashMap;
use test::Bencher;

// A regular expression to decompose a SAN. Note that castling is excluded here.
const SAN_REGEX: &str = "(?P<Piece>[NBRQK])?(?P<RemainderFile>[a-h])?(?P<RemainderRank>[1-8])?(?P<Hit>x)?(?P<Target>[a-h][1-8])=?(?P<PromotesTo>[NBRQK])?(?P<Check>\\+|#)?";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Draw {
    san: String,
    target: Coord,
    piece: Piece,
    is_check: bool,
    is_checkmate: bool,
    is_promo: bool,
    is_hit: bool,
    promoted_piece: Option<Piece>,
    remainder_file: Option<char>,
    remainder_rank: Option<char>,
}

impl From<String> for Draw {
    fn from(san: String) -> Self {
        // Use a regular expression to decompose the SAN (without Castling).
        // ref: https://stackoverflow.com/questions/54259474/convert-regex-captures-into-hashmap-in-rust
        let re_san: Regex = Regex::new(SAN_REGEX).unwrap();
        let captures = re_san.captures(&san[..]).unwrap();
        let capture_map: HashMap<&str, &str> = re_san
            .capture_names()
            .flatten()
            .filter_map(|n| Some((n, captures.name(n)?.as_str())))
            .collect();

        // Sort the matching groups into the according parts.
        Draw {
            san: san.clone(),

            is_check: san.contains('+') | san.contains('#'),
            is_checkmate: san.contains('#'),
            is_promo: san.contains('='),
            is_hit: san.contains('x'),

            target: Coord::from(capture_map.get("Target").unwrap().clone()),
            piece: match capture_map.get("Piece") {
                None => Piece::P,
                Some(&p) => Piece::from(p.chars().next().unwrap()),
            },
            promoted_piece: match capture_map.get("PromotesTo") {
                None => None,
                Some(&c) => Some(Piece::from(c.chars().next().unwrap())),
            },
            remainder_file: match capture_map.get("RemainderFile") {
                None => None,
                Some(&c) => Some(c.chars().next().unwrap()),
            },
            remainder_rank: match capture_map.get("RemainderRank") {
                None => None,
                Some(&c) => Some(c.chars().next().unwrap()),
            },
        }
    }
}

#[test]
fn check_draw_from_san_pt1() {
    let re_san = Regex::new(SAN_REGEX).unwrap();
    let draw = Draw::from("a3".to_owned());

    assert_eq!(draw.target, Coord::from("a3"));
    assert_eq!(draw.piece, Piece::P);
    assert_eq!(draw.is_check, false);
    assert_eq!(draw.is_checkmate, false);
    assert_eq!(draw.is_promo, false);
    assert_eq!(draw.is_hit, false);
    assert_eq!(draw.promoted_piece, None);
    assert_eq!(draw.remainder_file, None);
    assert_eq!(draw.remainder_rank, None);
}

#[test]
fn check_draw_from_san_pt2() {
    let re_san = Regex::new(SAN_REGEX).unwrap();
    let draw = Draw::from("exd1=Q#".to_owned());

    assert_eq!(draw.target, Coord::from("d1"));
    assert_eq!(draw.piece, Piece::P);
    assert_eq!(draw.is_check, true);
    assert_eq!(draw.is_checkmate, true);
    assert_eq!(draw.is_promo, true);
    assert_eq!(draw.is_hit, true);
    assert_eq!(draw.promoted_piece, Some(Piece::Q));
    assert_eq!(draw.remainder_file, Some('e'));
    assert_eq!(draw.remainder_rank, None);
}

#[test]
fn check_draw_from_san_pt3() {
    let re_san = Regex::new(SAN_REGEX).unwrap();
    let draw = Draw::from("Raxc6+".to_owned());

    assert_eq!(draw.target, Coord::from("c6"));
    assert_eq!(draw.piece, Piece::R);
    assert_eq!(draw.is_check, true);
    assert_eq!(draw.is_checkmate, false);
    assert_eq!(draw.is_promo, false);
    assert_eq!(draw.is_hit, true);
    assert_eq!(draw.promoted_piece, None);
    assert_eq!(draw.remainder_file, Some('a'));
    assert_eq!(draw.remainder_rank, None);
}

#[test]
fn check_draw_from_san_pt4() {
    let re_san = Regex::new(SAN_REGEX).unwrap();
    let draw = Draw::from("N1c3".to_owned());

    assert_eq!(draw.target, Coord::from("c3"));
    assert_eq!(draw.piece, Piece::N);
    assert_eq!(draw.is_check, false);
    assert_eq!(draw.is_checkmate, false);
    assert_eq!(draw.is_promo, false);
    assert_eq!(draw.is_hit, false);
    assert_eq!(draw.promoted_piece, None);
    assert_eq!(draw.remainder_file, None);
    assert_eq!(draw.remainder_rank, Some('1'));
}

#[bench]
fn bench_san_to_draw_conversion(b: &mut Bencher) {
    /// https://lichess.org/ML6mBOpY
    b.iter(|| {
        Vec::from([
            "d4", "d5", "e3", "c5", "c3", "Nc6", "h3", "Nf6", "Nf3", "e6", "Be2", "c4", "a3",
            "Bd6", "b4", "Bd7", "Bb2", "a6", "O-O", "O-O", "Nbd2", "h6", "Qe1", "Nh7", "Nh2", "f5",
            "f4", "b5", "Nhf3", "Qf6", "Kh2", "Kh8", "Rg1", "g5", "Ne5", "Bxe5", "dxe5", "Qg6",
            "g3", "Kg7", "Nf3", "Kf7", "Nd4", "Ke7", "Nxc6+", "Bxc6", "Bf3", "h5", "h4", "g4",
            "Bg2", "Rf7", "Rf1", "Nf8", "Rf2", "Nd7", "Rd2", "Nb6", "Qd1", "Rd8", "Rd4", "Kf8",
            "Bf1", "Rfd7", "Be2", "Na4", "Qc2", "Kg7", "Bc1", "Qf7", "Bd1", "Nb6", "Qd2", "Qe7",
            "Bb2", "Qf7", "Bc2", "Qe7", "Qd1", "Kg6", "Kg2", "Qf7", "Kf2", "Qe7", "Bc1", "Qf7",
            "Bd2", "Qe7", "Be1", "Qf7", "Qc1", "Qe7", "Qb2", "Qf7", "Kg2", "Qe7", "Bf2", "Qf7",
            "Qc1", "Qe7", "Ra2", "Qf7", "Qe1", "Qe7", "Kg1", "Qf7", "Rd1", "Qe7", "Rda1", "Qf7",
            "Qd2", "Qe7", "Qd4", "Rb7", "Kh2", "Qc7", "Bd1", "Ra8", "Bc2", "a5", "Be1", "a4",
            "Bf2", "Nd7", "Re1", "Nb6", "Raa1", "Nd7", "Rad1", "Nb6", "Re2", "Nd7", "Rde1", "Nb6",
            "Kg1", "Nd7", "Rd2", "Nb6", "Rdd1", "Nd7", "Qd2", "Nb6", "Qe2", "Nd7", "Rd4", "Nb6",
            "Red1", "Rd8", "e4", "fxe4", "Bxe4+", "Kh6", "Bc2", "Qe7", "Qe3", "Rbd7", "f5+", "Kh7",
            "f6+", "Kg8", "Qg5+", "Kf8", "fxe7+", "Rxe7", "Rf1", "Rg7", "Qh6", "Rd7", "Be3+",
            "Kg8", "Qxh5", "Rdf7", "Rxf7", "Rxf7", "Rf4", "Rxf4", "Bxf4", "Be8", "Qxg4+", "Kf7",
            "Qh5+", "Ke7", "Qh7+", "Bf7", "Bg5+", "Kf8", "Bh6+", "Ke7", "Bg6", "Kd7", "Qxf7+",
            "Kc6", "Qxe6+", "Kb7", "Qe7+", "Ka6", "Qc7", "d4", "e6", "d3", "e7", "d2", "e8=Q",
            "d1=Q+", "Kh2", "Qd2+", "Kh3", "Qxh6", "Qef7", "Qxg6",
        ])
        .into_iter()
        .map(|san| san.to_owned())
        .map(|san| Draw::from(san))
    });
}
