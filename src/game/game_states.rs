use super::components::player::Player;
use super::components::deck::Deck;
use super::components::bank::Bank;
use super::components::map_creation::{Point, random_base_map, Hexagon};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
type CatanMap = [[Point; 21]; 23];

#[derive(Serialize, Deserialize, Debug)]
pub struct GameStates {
    pub players: [Player; 4],
    pub deck: Deck,
    pub bank: Bank,
    pub current_player: usize,
    pub round: usize,
    pub map: CatanMap,
    pub hexagon_list: Vec<Hexagon>,
}


impl GameStates {
    pub fn random_map() -> GameStates {
        let state = random_base_map();
        GameStates {
            players: [Player::new(0), Player::new(1), Player::new(2), Player::new(3)], 
            deck: Deck::new(), 
            bank: Bank::new(), 
            current_player: 0,
            round: 0,
            map: state.map,
            hexagon_list: state.hexagon_list,
        }
    }
}

pub struct FirstPlacement;
pub struct SecondPlacement;
pub struct AfterPlacement;


const PLACES: [(usize, usize); 54] = [(10, 0), (8, 2), (6, 2), (4, 4), (6, 6), (8, 6), (10, 8), (8, 10), (10, 12), (8, 14), (10, 16), (8, 18), (6, 18), (4, 16), (2, 16), (0, 14), (2, 12), (0, 10), (2, 8), (0, 6), (4, 12), (10, 20), (12, 20), (14, 18), (16, 18), (18, 16), (16, 14), (14, 14), (18, 12), (16, 10), (14, 10), (18, 8), (16, 6), (14, 6), (12, 4), (14, 2), (16, 2), (18, 4), (20, 4), (22, 6), (20, 8), (22, 10), (20, 12), (22, 14), (20, 16), (12, 16), (6, 14), (12, 12), (6, 10), (12, 8), (4, 8), (2, 4), (10, 4), (12, 0)];
pub trait MoveControllers {
    fn get_houses(game_state: &GameStates) -> Vec<(usize, usize)> {
        PLACES.into_iter().filter(|&(x,y)| game_state.map[x][y].is_movable()).collect()
    }
    
    fn pay_house(game_state: &mut GameStates) {
        
    }

    fn build_house(game_state: &mut GameStates, x: usize, y: usize) {
        game_state.map[x][y].player_num = game_state.current_player;
        for &(nx, ny) in &game_state.map[x][y].neighbour_houses {
            game_state.map[nx][ny].player_num = game_state.current_player;
        }
        Self::pay_house(game_state);
    }

    fn get_roads(game_state: &GameStates) -> Vec<(usize, usize)>{
        game_state.players[game_state.current_player].road_coors.iter()
                        .flat_map(|&(x,y)| game_state.map[x][y].neighbour_roads.clone())
                        .filter(|&(x,y)| game_state.map[x][y].is_movable())
                        .collect()
    }
    fn pay_road(game_state: &mut GameStates) {}
    fn build_road(game_state: &mut GameStates) {
        Self::pay_road(game_state);
    }
    fn get_devs(game_state: &GameStates) {}
    fn use_dev(game_state: &GameStates) {}
    fn buy_dev(game_state: &GameStates) {}
    fn end_turn(game_state: &mut GameStates);
    fn next_turn(game_state: &mut GameStates) {
        game_state.round += 1;
        Self::end_turn(game_state);
    }
}

impl MoveControllers for FirstPlacement {
    fn end_turn(game_state: &mut GameStates) {
        if game_state.current_player < 3 {game_state.current_player += 1}
    }
}
impl MoveControllers for SecondPlacement {
    fn end_turn(game_state: &mut GameStates) {
        if game_state.current_player > 0 {game_state.current_player -= 1}
    }
}
impl MoveControllers for AfterPlacement {
    fn end_turn(game_state: &mut GameStates) {
        game_state.current_player = (game_state.current_player + 1) % 4;
    }
}

