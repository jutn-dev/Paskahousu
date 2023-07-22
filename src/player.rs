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


    /// # Remove Card
    ///
    /// This function adds specific card to players cards
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
    /// # Add Card
    ///
    /// this function adds specific card to players cards
    pub fn add_card(&mut self, card: Card){
    self.cards.push(card);

    }

    /// # Print Cards
    ///
    /// print_cards prints all player's cards to stdout
    pub fn print_cards(&self){
        for card in &self.cards {
            print!(" {}{}, ", card.num, card.suit);
        }
        print!("\n");
    }
}
