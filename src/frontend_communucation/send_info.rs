use crate::game::components::map_creation::{Point, Hexagon, random_base_map, State};
use crate::game::components::resources::{PortTypes, HexagonTypes};
use serde::{Serialize, Deserialize};




pub fn get_random_map() -> String {
    let res = random_base_map();
    serde_json::to_string(&res.hexagon_list).unwrap()
}
