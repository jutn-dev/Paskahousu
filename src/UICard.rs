use bevy::prelude::*;
use crate::card::Card;



#[derive(Component, Debug, Clone)]
pub struct UICard{
    pub card: Card,
}

