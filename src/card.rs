
///# Card
///
/// Card struct hold playing card's number and suit
///
/// `num` is 1-10 or jack(j), queen(q) or king(k).
/// `Suit` is spade(s), club(c), heart(h) or diamond(d).
#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    /// `num` is 1-13 (jack(11), queen(12) or king(13)).
    pub num: u8,
    /// `Suit` is spade(s), club(c), heart(h) or diamond(d).
    pub suit: String,
}

impl Card {
    pub fn new(num: u8, suit: String) -> Card {
        Card {
            num: num,
            suit: suit,
        }
    }
}
