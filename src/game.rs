use crate::card::Card;
use crate::player::player;
pub struct Game {
    Players: Vec<player>
}

impl Game {
    fn game_loop() {
        let mut cards: Vec<Card>;

        loop {
            println!("select a card (1-10, j, q, k, a)(s,c,d,h)");
            let mut card = String::new();
            std::io::stdin().read_line(&mut card);

            Self::use_card(Card::new("1".to_string(), "s".to_string()));
        }
    }

    fn use_card(card: Card, cards: Vec<Card>) {
        cards.push(card);
    }

    fn return_52_cards() -> Vec<Card> {
        let mut cards: Vec<Card>;

        for j in 1..4 {
            match j {
                1 => {
                    for i in 1..14 {
                        match i {
                            11 => cards.push(Card::new(i.to_string(), "s".to_string())),
                            12 => cards.push(Card::new(i.to_string(), "s".to_string())),
                            13 => cards.push(Card::new(i.to_string(), "s".to_string())),
                            14 => cards.push(Card::new(i.to_string(), "s".to_string())),
                            _ => cards.push(Card::new(i.to_string(), "s".to_string())),
                        }
                    }
                }

                2 => {
                    for i in 1..14 {
                        match i {
                            11 => cards.push(Card::new(i.to_string(), "c".to_string())),
                            12 => cards.push(Card::new(i.to_string(), "c".to_string())),
                            13 => cards.push(Card::new(i.to_string(), "c".to_string())),
                            14 => cards.push(Card::new(i.to_string(), "c".to_string())),
                            _ => cards.push(Card::new(i.to_string(), "c".to_string())),
                        }
                    }
                }

                3 => {
                    for i in 1..14 {
                        match i {
                            11 => cards.push(Card::new(i.to_string(), "d".to_string())),
                            12 => cards.push(Card::new(i.to_string(), "d".to_string())),
                            13 => cards.push(Card::new(i.to_string(), "d".to_string())),
                            14 => cards.push(Card::new(i.to_string(), "d".to_string())),
                            _ => cards.push(Card::new(i.to_string(), "d".to_string())),
                        }
                    }
                }

                4 => {
                    for i in 1..14 {
                        match i {
                            11 => cards.push(Card::new(i.to_string(), "h".to_string())),
                            12 => cards.push(Card::new(i.to_string(), "h".to_string())),
                            13 => cards.push(Card::new(i.to_string(), "h".to_string())),
                            14 => cards.push(Card::new(i.to_string(), "h".to_string())),
                            _ => cards.push(Card::new(i.to_string(), "h".to_string())),
                        }
                    }
                }
            }
        }
        cards
    }
}
