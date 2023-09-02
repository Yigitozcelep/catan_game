use serde::{Serialize, Deserialize};
use crate::game::components::map_creation::{Point, Hexagon, base_map_frame};

#[derive(Serialize, Deserialize)]
struct Info {
    map: [[Point;21]; 23],
    hexagon_list: Vec<Hexagon>,
}

pub fn get_random_map() -> String {
    let res = base_map_frame();
    serde_json::to_string(&Info{map: res.map, hexagon_list: res.hexagon_list}).unwrap()
}