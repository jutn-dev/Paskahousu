pub struct Card {
    num: String,
    suit: String,
}

impl Card {
    pub fn new(num: String, suit: String) -> Card {
        Card {
            num: num,
            suit: suit,
        }
    }

}
