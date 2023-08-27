use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PlayerNum {
    Player1,
    Player2,
    Player3,
    Player4,
    None,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Player {
    pub player_num: PlayerNum,
    pub tot: usize,
    pub lumber: usize,
    pub grain: usize,
    pub ore: usize,
    pub brick: usize,
    pub wool: usize,
    pub max_road: usize,
    pub max_army: usize,
    pub dev_army: usize,
    pub dev_road: usize,
    pub dev_monopoly: usize,
    pub dev_2_resources: usize,
    pub dev_victory: usize,
}

impl Player {
    pub fn new(player_num: PlayerNum) -> Player {
        Player {
            player_num, tot: 0,lumber: 0,
            ore: 0,grain: 0,
            brick: 0,wool: 0,
            max_road: 0,max_army: 0,
            dev_army: 0,dev_road: 0,
            dev_monopoly: 0,dev_2_resources: 0,
            dev_victory: 0
        }
    }
}
