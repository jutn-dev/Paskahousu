use crate::card::Card;

#[derive(PartialEq)]
pub struct player {
    pub cards: Vec<Card>,
}





impl player {
    fn has_card(&self, card: &Card) -> bool {
        for players_card in self.cards.iter() {
            if players_card == card {
                return true;
            }
        }
        false
    }
}
