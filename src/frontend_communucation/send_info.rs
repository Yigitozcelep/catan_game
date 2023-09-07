use crate::game::game_controller::GameController;


pub fn get_random_map() -> String {
    let game = GameController::random_map();
    serde_json::to_string(&game).unwrap()
}