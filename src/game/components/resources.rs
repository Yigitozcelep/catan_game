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
pub enum HexagonTypes {
   Brick,
   Lumber,
   Wool,
   Grain,
   Ore,
   Desert,
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PortTypes {
   Brick,
   Lumber,
   Wool,
   Grain,
   Ore,
   QuestionMark,
}
