use crate::game::components::point;
use serde::{Serialize, Deserialize};


pub fn get_random_map() -> String {
    let res = point::create_random_map();
    serde_json::to_string(&res).unwrap()
}
