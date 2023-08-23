use crate::game::components::point::Hexagon;
use crate::game::components::point::Point;
use serde::{Serialize, Deserialize};


pub fn get_map(hexaongs: Vec<Hexagon>) -> String {
    return serde_json::to_string(&hexaongs).unwrap();
}
