use crate::game::*;
use crate::card::*;

pub enum Ruules {
    PlayerDoesntHaveCard,
    CardTooSmall,
    NoSevenUsed,
}



impl Game {
    pub(super) fn check_move(&self, card: &Card, cards: &Vec<Card>) -> Result<(), Ruules> {
        
        if !self.players[0].has_card(card) {
            return Err(Ruules::PlayerDoesntHaveCard);
        }
        if cards.is_empty() /*&& card.num >! 10*/{
            return Ok(());
        }
        println!("{:?}", cards.is_empty());
        let top_card = cards.last().unwrap();
        
        //card too small
        if top_card.num > card.num && (card.num != 2 || card.num != 1) {
            return Err(Ruules::CardTooSmall);
        }
        
        //cant use picture cards before seven
        if top_card.num < 7 && (card.num > 10 || card.num == 1) {
            return Err(Ruules::NoSevenUsed)
        }

        

        
        


        return Ok(());
        
        }
}
