use crate::game::*;
use crate::card::*;

impl Game {
    pub(super) fn check_move(&self, card: &Card, cards: &Vec<Card>) -> bool {
        if !self.players[0].has_card(card) {
            return false;
        }

        //TODO
        //if cards.last().unwrap().num == "" {

        //}
        


        true
    }
}
