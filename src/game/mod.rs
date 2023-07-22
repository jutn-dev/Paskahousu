use crate::card::Card;
use crate::player::Player;
use rand::Rng;

mod rules;

pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game { players: vec![] }
    }

    pub fn game_loop(&mut self) {


        let mut cards: Vec<Card>;
        let mut played_cards: Vec<Card> = vec![];
        cards = Self::return_all_cards();
        self.players.push(Player::new()); 

        //TODO move to init function when you make it
        Self::new_cards(&mut self.players[0], &mut cards);

        loop {

            self.players[0].print_cards();
            println!("{:?}", played_cards);
            
            //get user card
            println!("select a card (1-10, j, q, k, a),(s,c,d,h)");
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let buf_vec: Vec<&str> = buf.trim().split(",").collect();
            let card = Card::new(buf_vec[0].to_string(), buf_vec[1].to_string());

            if self.check_move(&card, &played_cards) {
                Self::use_card(card, &mut played_cards, &mut self.players[0]);
                Self::new_cards(&mut self.players[0], &mut cards);
            }
            else {
                println!("you can't do that!")
            }
           
        }
    }

    fn new_cards(player: &mut Player, cards: &mut Vec<Card>) {
        loop {
            //if player has more than 5 card dont take a card
            if player.cards.len() >= 5 {
                return;
            }
            let mut rng = rand::thread_rng();
            let random_card = rng.gen_range(0..(cards.len() - 1));
            //adds random card to player
            player.add_card(cards[random_card].clone());
            //removes the random card from cards
            cards.remove(random_card);
        }
    }

    fn use_card(card: Card, cards: &mut Vec<Card>, player: &mut Player) {
        cards.push(card.clone());
        player.remove_card(card.clone())
    }

    ///# checks if move is valid
    ///
    ///returns true if move is valid


    /// # returns all cards in order
    fn return_all_cards() -> Vec<Card> {
        let mut cards: Vec<Card> = vec![];
        for j in 1..4 {
            match j {
                1 => {
                    for i in 1..14 {
                        match i {
                            11 => cards.push(Card::new("j".to_string(), "s".to_string())),
                            12 => cards.push(Card::new("q".to_string(), "s".to_string())),
                            13 => cards.push(Card::new("k".to_string(), "s".to_string())),
                            14 => cards.push(Card::new("a".to_string(), "s".to_string())),
                            _ => cards.push(Card::new(i.to_string(), "s".to_string())),
                        }
                    }
                }

                2 => {
                    for i in 1..14 {
                        match i {
                            11 => cards.push(Card::new("j".to_string(), "c".to_string())),
                            12 => cards.push(Card::new("q".to_string(), "c".to_string())),
                            13 => cards.push(Card::new("k".to_string(), "c".to_string())),
                            14 => cards.push(Card::new("a".to_string(), "c".to_string())),
                            _ => cards.push(Card::new(i.to_string(), "c".to_string())),
                        }
                    }
                }

                3 => {
                    for i in 1..14 {
                        match i {
                            11 => cards.push(Card::new("j".to_string(), "d".to_string())),
                            12 => cards.push(Card::new("q".to_string(), "d".to_string())),
                            13 => cards.push(Card::new("k".to_string(), "d".to_string())),
                            14 => cards.push(Card::new("a".to_string(), "d".to_string())),
                            _ => cards.push(Card::new(i.to_string(), "d".to_string())),
                        }
                    }
                }

                4 => {
                    for i in 1..14 {
                        match i {
                            11 => cards.push(Card::new("j".to_string(), "h".to_string())),
                            12 => cards.push(Card::new("q".to_string(), "h".to_string())),
                            13 => cards.push(Card::new("k".to_string(), "h".to_string())),
                            14 => cards.push(Card::new("a".to_string(), "h".to_string())),
                            _ => cards.push(Card::new(i.to_string(), "h".to_string())),
                        }
                    }
                }
                _ => (),
            }
        }
        cards
    }
}
