use std::fmt;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coord {
    pub file: char,
    pub rank: char,

    // use signed integers to enable comparisons to other coords via subtraction.
    pub x: i8,
    pub y: i8,
    pub idx: i8,

    pub anti_diagonal: i8,
    pub main_diagonal: i8,
}

// Traits
pub trait FromIndex {
    fn from_idx(idx: i8) -> Self;
}

// Implementations
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl From<&str> for Coord {
    fn from(field: &str) -> Self {
        // use early assertions to safeguard against broken inputs.
        assert_eq!(field.len(), 2);

        // if there are at least two characters, assign those as file and rank.
        let file = field.chars().next().unwrap();
        let rank = field.chars().nth(1).unwrap();

        // derive coordinate system and vector representation of coordinate.
        let x: i8 = file as i8 - 'a' as i8;
        let y: i8 = rank as i8 - '1' as i8;
        let idx: i8 = x + 8 * (7 - y); // indented to match FEN.

        // derive diagonals of coordinate.
        let anti_diagonal: i8 = x + y;
        let main_diagonal: i8 = 7 + y - x;

        // check validity of char inputs directly by their integer bounds.
        //  Note that this indirectly also check the validity of idx and diagonals.
        //  Additionally: u8 guarantees the integer to be positive ( > 0).
        assert!((x < 8) & (y < 8));

        Coord {
            file,
            rank,
            x,
            y,
            idx,
            anti_diagonal,
            main_diagonal,
        }
    }
}

impl FromIndex for Coord {
    fn from_idx(idx: i8) -> Self {
        // check upper bound.
        assert!(idx < 64);

        // derive x and y coordinates from index.
        let x: i8 = idx % 8;
        let y: i8 = 7 - idx / 8;

        // convert into rank and file characters.
        let file = char::from_u32((x + 97) as u32).unwrap();
        let rank = char::from_u32((y + 49) as u32).unwrap();

        // concat and convert coordinate
        let mut coo = "".to_owned();
        coo.push(file);
        coo.push(rank);
        let coo: &str = &coo[..]; // StackOverflow > "How to convert a String into a &'static str"

        Coord::from(coo)
    }
}

//-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-

#[test]
fn check_x() {
    assert_eq!(0, Coord::from("a1").x);
    assert_eq!(4, Coord::from("e1").x);
    assert_eq!(4, Coord::from("e4").x);
    assert_eq!(7, Coord::from("h7").x);
}

#[test]
fn check_y() {
    assert_eq!(0, Coord::from("a1").y);
    assert_eq!(0, Coord::from("e1").y);
    assert_eq!(3, Coord::from("e4").y);
    assert_eq!(6, Coord::from("h7").y);
}

#[test]
fn check_idx() {
    assert_eq!(0, Coord::from("a8").idx);
    assert_eq!(4, Coord::from("e8").idx);
    assert_eq!(28, Coord::from("e5").idx);
    assert_eq!(63, Coord::from("h1").idx);
}

#[test]
fn check_anti_diagonal() {
    assert_eq!(0, Coord::from("a1").anti_diagonal);
    assert_eq!(4, Coord::from("e1").anti_diagonal);
    assert_eq!(7, Coord::from("e4").anti_diagonal);
    assert_eq!(13, Coord::from("h7").anti_diagonal);
}

#[test]
fn check_main_diagonal() {
    assert_eq!(7, Coord::from("a1").main_diagonal);
    assert_eq!(3, Coord::from("e1").main_diagonal);
    assert_eq!(6, Coord::from("e4").main_diagonal);
    assert_eq!(6, Coord::from("h7").main_diagonal);
}

#[test]
fn check_identity() {
    assert_eq!(Coord::from("a1"), Coord::from("a1"));
    assert_eq!(Coord::from("b3"), Coord::from("b3"));
    assert_eq!(Coord::from("h8"), Coord::from("h8"));
}

#[test]
#[should_panic]
fn check_illegal_coords_pt1() {
    let _ = Coord::from("a9");
}

#[test]
#[should_panic]
fn check_illegal_coords_pt2() {
    let _ = Coord::from("i1");
}

#[test]
#[should_panic]
fn check_illegal_coords_pt3() {
    let _ = Coord::from("1a");
}

#[test]
fn check_idx_conversion() {
    assert_eq!(Coord::from("a1"), Coord::from_idx(Coord::from("a1").idx));
    assert_eq!(Coord::from("h8"), Coord::from_idx(Coord::from("h8").idx));
    assert_eq!(Coord::from("e3"), Coord::from_idx(Coord::from("e3").idx));
}
