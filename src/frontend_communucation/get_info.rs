use serde_json;
use crate::game::game_controller::GameController;

pub fn get_moves(data: String) -> Vec<(usize, usize)> {
    let res: GameController = serde_json::from_str(&data).unwrap();
    res.get_all_housable_point()
}
