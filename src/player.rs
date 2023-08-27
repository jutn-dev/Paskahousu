use crate::{UICard::UICard,card::Card};
use bevy::prelude::*;

//MainPlayer as main for the player
#[derive(Component)]
pub struct MainPlayer;

#[derive(Component, Debug, PartialEq, Clone)]
pub struct Player {
    pub cards: Vec<Card>,
}

pub fn create_player_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    query: Query<&Player, With<MainPlayer>>,
) {
    let main_player = query.single();

    let cards_texture = asset_server.load("cards.png");

    let mut cards_ui = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    });

    //TODO add the for loop here
    for card in main_player.cards.iter() {
        let card_suit;
        match card.suit.as_str() {
            "c" => card_suit = 0,
            "s" => card_suit = 1,
            "d" => card_suit = 2,
            "h" => card_suit = 3,
            _ => card_suit = 0,
        }

        cards_ui.with_children(|parent| {
            parent.spawn((AtlasImageBundle {
                texture_atlas: texture_atlas.add(TextureAtlas::from_grid(
                    cards_texture.clone(),
                    Vec2::new(106.0, 156.0),
                    card.num as usize,
                    card_suit,
                    None,
                    None,
                )),

                style: Style {
                    width: Val::Px(106.0),
                    height: Val::Px(156.0),
                    align_self: AlignSelf::End,
                    bottom: Val::Percent(10.0),
                    ..default()
                },
                ..default()
            },
            UICard { card: card.clone()}
            ));
        });
    }
}


//PLayer implemation
impl Player {
    pub fn new() -> Player {
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
    pub fn remove_card(&mut self, card: Card) {
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
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// # Print Cards
    ///
    /// print_cards prints all player's cards to stdout
    pub fn print_cards(&self) {
        for card in &self.cards {
            print!(" {}{}, ", card.num, card.suit);
        }
        print!("\n");
    }
}
