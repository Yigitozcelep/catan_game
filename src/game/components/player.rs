use serde::{Serialize, Deserialize};
use super::map_creation::State;


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
    pub  dev_any_2: usize,
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

    pub fn get_first_move(&self, state: &State) -> Vec<(usize, usize)>{
        let mut moved = [[false; 21];23];
        let mut res: Vec<(usize, usize)> = Vec::new();
        let mut q: Vec<(usize, usize)> = vec![(10,0)];
        while !q.is_empty() {
            let (x,y) = q.pop().unwrap();
            res.push((x,y));
            for (nx, ny) in &state.map[x][y].neighbour_houses {
                if moved[*nx][*ny] || !state.map[*nx][*ny].is_housable() {continue;}
                moved[*nx][*ny] = true;
                q.push((*nx,*ny));
            }
        }
        res
    }

    pub fn make_house(&mut self, state: &mut State, x: usize, y: usize) {
        state.map[x][y].player_num = self.player_num;
        self.house_coors.push((x,y));
        for (nx,ny) in &state.map[x][y].neighbour_houses {
            state.map[*nx][*ny].player_num = self.player_num;
        }
    }
}
