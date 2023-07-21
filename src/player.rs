use crate::card::Card;

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub cards: Vec<Card>,
}





impl Player {
    
    pub fn new() -> Player{
        Player { cards: vec![] }
    }

    //check if player has a specific card
    pub fn has_card(&self, card: &Card) -> bool {
        for players_card in self.cards.iter() {
            if players_card == card {
                return true;
            }
        }
        false
    }
}
