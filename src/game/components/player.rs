use serde::{Serialize, Deserialize};
use super::map_creation::MapInfos;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub player_num: usize,
    pub road_coors: Vec<(usize, usize)>,
    pub house_coors: Vec<(usize, usize)>,
    pub tot: usize,
    pub lumber: usize,
    pub grain: usize,
    pub ore: usize,
    pub brick: usize,
    pub wool: usize,
    pub max_road: usize,
    pub max_army: usize,
    pub dev_knight: usize,
    pub dev_road: usize,
    pub dev_monopoly: usize,
    pub dev_any_2: usize,
    pub dev_victory: usize,
}

impl Player {
    pub fn new(player_num: usize) -> Player {
        Player {
            player_num, tot: 0,lumber: 0,
            road_coors: Vec::new(), house_coors: Vec::new(),
            ore: 0,grain: 0,
            brick: 0,wool: 0,
            max_road: 0,max_army: 0,
            dev_knight: 0,dev_road: 0,
            dev_monopoly: 0, dev_any_2: 0,
            dev_victory: 0
        }
    }


    pub fn get_roads(&self, map_infos: &MapInfos) -> Vec<(usize, usize)> {
        self.house_coors.iter().flat_map(|(x,y)| map_infos.map[*x][*y].neighbour_roads.clone()).collect()
    }
    
    pub fn make_road(&mut self) {
        unimplemented!()
    }

    pub fn make_house(&mut self, map_infos: &mut MapInfos, x: usize, y: usize) {
        map_infos.map[x][y].player_num = self.player_num;
        self.house_coors.push((x,y));
        for &(nx,ny) in &map_infos.map[x][y].neighbour_houses {
            map_infos.map[nx][ny].player_num = self.player_num;
        }
    }
}
