use serde::{Serialize, Deserialize};
use crate::game::components::map_creation;

pub fn get_random_map() -> String {
    let res = map_creation::base_map_frame();
    serde_json::to_string(&res.hexagon_list).unwrap()
}