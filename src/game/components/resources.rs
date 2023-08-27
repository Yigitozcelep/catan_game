use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Resources {
   Brick,
   Lumber,
   Wool,
   Grain,
   Ore,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum HexagonType {
   Brick,
   Lumber,
   Wool,
   Grain,
   Ore,
   Desert,
}

