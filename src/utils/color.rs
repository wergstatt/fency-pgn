use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Color {
    W,
    B,
}

impl Color {
    pub fn next(self) -> Self {
        match self {
            Color::W => Color::B,
            Color::B => Color::W,
        }
    }

    // White pawns are moving upwards, black ones downwards.
    pub fn factor(self) -> i8 {
        match self {
            Color::W => 1,
            Color::B => -1,
        }
    }

    pub fn is_white(self) -> bool {
        match self {
            Color::W => true,
            Color::B => false,
        }
    }

    pub fn is_black(self) -> bool {
        match self {
            Color::W => false,
            Color::B => true,
        }
    }
}

impl From<char> for Color {
    fn from(col: char) -> Color {
        // ensure that the incoming char is truly one of the expected colors.
        assert!((col == 'w') | (col == 'b'));

        // Navigate to the correct color instance.
        if col == 'w' {
            Color::W
        } else {
            Color::B
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let outstr = match self {
            Color::W => "w".to_string(),
            Color::B => "b".to_string(),
        };

        write!(f, "{}", outstr)
    }
}

#[test]
fn identity() {
    assert_eq!(Color::W, Color::W);
    assert_eq!(Color::B, Color::B);
}

#[test]
fn iteration() {
    assert_eq!(Color::W.next(), Color::B);
    assert_eq!(Color::B.next(), Color::W);
}

#[test]
fn derivation() {
    assert_eq!(Color::from('b'), Color::B);
    assert_eq!(Color::from('w'), Color::W);
}
