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
    pub fn remove_card(&mut self, card: Card){
        let mut index = 0;
        for players_card in self.cards.iter() {
            if players_card == &card {
                self.cards.remove(index);
                return;
            }
            index += 1;
        }
    }

    pub fn add_card(&mut self, card: Card){
    self.cards.push(card);

    }
}
