use crate::game::*;
use crate::card::*;

pub enum Rules {
    PlayerDoesntHaveCard,
    CardTooSmall,
    NoOverSevenToEmpty,
    NoSevenUsed,
}



impl Game {
    pub(super) fn check_move(&self, card: &Card, cards: &Vec<Card>) -> Result<(), Rules> {
        
        if !self.players[0].has_card(card) {
            return Err(Rules::PlayerDoesntHaveCard);
        }
        if cards.is_empty() && (card.num < 7 && card.num > 1){
            return Ok(());
        }
        if cards.is_empty() && (card.num > 7 || card.num == 1) {
            return Err(Rules::NoOverSevenToEmpty);
        }
        println!("{:?}", cards.is_empty());
        let top_card = cards.last().unwrap();
        
        //card too small
        if top_card.num > card.num || top_card.num == 2{
            if card.num == 2 {
                return Ok(());
            }
            if top_card.num > 10 && card.num == 1{
                return Ok(());
            }
            
            return Err(Rules::CardTooSmall);
        }
        
        //cant use picture cards before seven
        if top_card.num < 7 && (card.num > 10 || card.num == 1) {
            return Err(Rules::NoSevenUsed)
        }

        

        
        


        return Ok(());
        
        }
}
