
///# Card
///
/// Card struct hold playing card's number and suit
///
/// `num` is 1-10 or jack(j), queen(q) or king(k).
/// `Suit` is spade(s), club(c), heart(h) or diamond(d).
#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    /// `num` is 1-10 or jack(j), queen(q) or king(k).
    pub num: String,
    /// `Suit` is spade(s), club(c), heart(h) or diamond(d).
    pub suit: String,
}

impl Card {
    pub fn new(num: String, suit: String) -> Card {
        Card {
            num: num,
            suit: suit,
        }
    }
}
