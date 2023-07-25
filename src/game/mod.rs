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
            println!("{:?}", played_cards.is_empty());
            let card = Self::get_user_card();

            if let Some(card) = card {
                match self.check_move(&card, &played_cards) {
                    Err(rules::Rules::NoSevenUsed) => println!("no seven used yet"),
                    Err(rules::Rules::NoOverSevenToEmpty) => println!("no card larger than seven can be used"),
                    Err(rules::Rules::CardTooSmall) => println!("Card too small"),
                    Err(rules::Rules::PlayerDoesntHaveCard) => println!("you don't have that card"),
                    Ok(_) => {
                        Self::use_card(card, &mut played_cards, &mut self.players[0]);
                        Self::new_cards(&mut self.players[0], &mut cards);
                    }
                }
            }
        }
    }

    fn get_user_card() -> Option<Card> {
        println!("select a card (1-10, j, q, k, a),(s,c,d,h)");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let buf_vec: Vec<&str> = buf.trim().split(",").collect();

        if buf_vec.len() < 2 {
            println!("syntax error");
            return None;
        }
        Some(Card::new(
            buf_vec[0].parse().unwrap(),
            buf_vec[1].to_string(),
        ))
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
                    for i in 1..13 {
                        cards.push(Card::new(i, "s".to_string()));
                    }
                }
                2 => {
                    for i in 1..13 {
                        cards.push(Card::new(i, "c".to_string()));
                    }
                }
                3 => {
                    for i in 1..13 {
                        cards.push(Card::new(i, "d".to_string()));
                    }
                }
                4 => {
                    for i in 1..13 {
                        cards.push(Card::new(i, "h".to_string()));
                    }
                }
                _ => (),
            }
        }
        cards
    }
}
