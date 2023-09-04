use super::components::player::{Player,PlayerNum};
use super::components::deck::Deck;
use super::components::bank::Bank;
use super::components::map_creation::State;


pub struct GameController{
    players: [Player; 4],
    deck: Deck,
    bank: Bank,
    state: State,
}

impl GameController {
    pub fn new() -> GameController{
        unimplemented!();
    }
}


