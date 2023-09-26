use serde_json;
use crate::game::game_states::{GameStates, FirstPlacement, SecondPlacement, AfterPlacement, MoveControllers};
use lazy_static::lazy_static;
use std::sync::Mutex;


lazy_static! {
    static ref GAME_STATES: Mutex<GameStates> = Mutex::new(GameStates::random_map());
}

pub fn get_housable_points() -> String {
    let game = GAME_STATES.lock().unwrap();
    if game.round < 4 {serde_json::to_string(&FirstPlacement::get_houses(&game)).unwrap()}
    else if game.round < 8 {serde_json::to_string(&SecondPlacement::get_houses(&game)).unwrap()}
    else {serde_json::to_string(&AfterPlacement::get_houses(&game)).unwrap()}
}

pub fn get_random_map() -> String {
    let game = GAME_STATES.lock().unwrap();
    serde_json::to_string(&game.hexagon_list).unwrap()
}


pub fn get_bank() -> String {
    let game = GAME_STATES.lock().unwrap();
    serde_json::to_string(&game.bank).unwrap()
}

pub fn get_players() -> String {
    let game = GAME_STATES.lock().unwrap();
    serde_json::to_string(&game.players).unwrap()
}

pub fn get_deck() -> String {
    let game = GAME_STATES.lock().unwrap();
    serde_json::to_string(&game.deck).unwrap()
}


pub fn get_current_player() -> String {
    let game = GAME_STATES.lock().unwrap();
    serde_json::to_string(&game.current_player).unwrap()
}

pub fn get_roads() -> String {
    let game = GAME_STATES.lock().unwrap();
    unimplemented!()
}

pub fn make_house(x: usize, y: usize) -> String {
    let mut game = GAME_STATES.lock().unwrap();
    if game.round < 4 {serde_json::to_string(&FirstPlacement::build_house(&mut game, x, y)).unwrap()}
    else if game.round < 8 {serde_json::to_string(&SecondPlacement::build_house(&mut game, x, y)).unwrap()}
    else {serde_json::to_string(&AfterPlacement::build_house(&mut game, x, y)).unwrap()}
}

