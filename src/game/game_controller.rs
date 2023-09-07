use super::components::player::Player;
use super::components::deck::Deck;
use super::components::bank::Bank;
use super::components::map_creation::{State, random_base_map};


pub struct GameController{
    pub players: [Player; 4],
    pub deck: Deck,
    pub bank: Bank,
    pub current_player: usize,
    pub state: State,
}

impl GameController {
    pub fn new(state: State) -> GameController {
        GameController {
            players: [Player::new(1), Player::new(2), Player::new(3), Player::new(4)], 
            deck: Deck::new(), 
            bank: Bank::new(), 
            current_player: 0,
            state,
        }
    }
    pub fn random_map() -> GameController{
        GameController { 
            players: [Player::new(0), Player::new(1), Player::new(2), Player::new(3)], 
            deck: Deck::new(), 
            bank: Bank::new(), 
            current_player: 1,
            state: random_base_map(),
         }
    }

    pub fn get_all_housable_point(&self) -> Vec<(usize, usize)>{
        self.players[self.current_player].get_first_move(&self.state)
    }

}


