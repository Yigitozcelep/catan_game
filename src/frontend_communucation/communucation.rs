use serde_json;
use crate::game::game_controller::GameController;
use lazy_static::lazy_static;
use std::sync::Mutex;


lazy_static! {
    static ref GAMECONTROLLER: Mutex<GameController> = Mutex::new(GameController::random_map());
}

pub fn get_housable_points() -> String {
    let game = GAMECONTROLLER.lock().unwrap();
    serde_json::to_string(&game.get_all_housable_point()).unwrap()
}

pub fn get_random_map() -> String {
    let game = GAMECONTROLLER.lock().unwrap();
    serde_json::to_string(&game.state.hexagon_list).unwrap()
}


pub fn get_bank() -> String {
    let game = GAMECONTROLLER.lock().unwrap();
    serde_json::to_string(&game.bank).unwrap()
}