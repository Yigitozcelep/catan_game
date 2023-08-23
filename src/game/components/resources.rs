use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Resources {
   Brick,
   Lumber,
   Wool,
   Grain,
   Ore,
   Desert,
}

