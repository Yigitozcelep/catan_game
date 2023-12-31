use crate::game::components::helper_functions::{random_weighted_choice, default_rng};
use crate::game::components::resources::{HexagonTypes, Resources, PortTypes};
use rand::SeedableRng;
use rand::rngs::SmallRng;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Hexagon {
    pub num: usize,
    pub resource: Resources,
    pub hexagon_type: HexagonTypes,
    pub port: PortTypes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub player_num: usize,
    pub neighbour_roads: Vec<(usize, usize)>,
    pub neighbour_houses: Vec<(usize, usize)>,
    pub port: PortTypes,
    pub hexagons: Vec<Hexagon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapInfos {
    pub map: [[Point;21]; 23],
    pub hexagon_list: Vec<Hexagon>,
    port_types: [PortTypes; 6],
    port_type_weights: [isize; 6],
    port_type_weight_tot: isize,
    hexagon_types: [HexagonTypes; 6],
    hexagon_type_weights: [isize; 6],
    hexagon_type_weight_tot: isize,
    num_list: [isize; 11],
    num_weights: [isize; 11] ,
    num_weight_tot: isize,
    #[serde(skip, default="default_rng")]
    rng: SmallRng,
}

impl MapInfos {
    fn new() -> MapInfos {
        MapInfos {
            map: [[(); 21]; 23].map(|data| data.map(|_| Point::new())),
            hexagon_list: Vec::new(),
            port_types: [PortTypes::Brick, PortTypes::Grain, PortTypes::Lumber, PortTypes::Ore, PortTypes::Wool, PortTypes::QuestionMark],
            port_type_weights: [1,1,1,1,1,4],
            port_type_weight_tot: 9,
            hexagon_types: [HexagonTypes::Brick, HexagonTypes::Grain, HexagonTypes::Lumber, HexagonTypes::Wool, HexagonTypes::Ore, HexagonTypes::Desert], 
            hexagon_type_weights: [3,4,4,4,3,1], 
            hexagon_type_weight_tot: 19, 
            num_list: [2, 3,4,5,6,7, 8,9,10,11, 12], 
            num_weights: [1, 2,2,2,2,0, 2,2,2,2, 1], 
            num_weight_tot: 18, 
            rng: SmallRng::from_entropy(), 
        }
    }
}

impl Point {
    fn new() -> Point {
        Point { 
            player_num: std::usize::MAX, 
            neighbour_roads: Vec::new(), 
            neighbour_houses: Vec::new(),
            port: PortTypes::None, 
            hexagons: Vec::new(), 
        }
    }
    #[inline(always)]
    pub fn is_movable(&self) -> bool {self.player_num == std::usize::MAX}
}

fn get_resource_of_hexagon_type(hexagon_type: HexagonTypes) -> Resources {
    match hexagon_type {
        HexagonTypes::Brick => Resources::Brick,
        HexagonTypes::Grain => Resources::Grain,
        HexagonTypes::Lumber => Resources::Lumber,
        HexagonTypes::Ore => Resources::Ore,
        HexagonTypes::Wool => Resources::Wool,
        HexagonTypes::Desert => unreachable!(),
    }
}

fn get_random_hexagon(state: &mut MapInfos, x: isize, y: isize) -> Option<Hexagon>{
    let mut excludes = neighbour_hexagons(state, x, y);
    let hexagon_index = random_weighted_choice(&mut state.hexagon_type_weights, &mut state.hexagon_type_weight_tot, &mut state.rng);
    let hexagon_type = state.hexagon_types[hexagon_index];
    if hexagon_type == HexagonTypes::Desert {
        let hexagon = Hexagon {num: 0, resource: Resources::Wool, hexagon_type, port: PortTypes::None };
        state.hexagon_list.push(hexagon);
        return Some(hexagon);
    }
    save_weights(&mut excludes, state);
    if state.num_weight_tot == 0 {return None;}
    let num_index = random_weighted_choice(&mut state.num_weights, &mut state.num_weight_tot, &mut state.rng);
    merge_weights(&excludes, state);
    let resource = get_resource_of_hexagon_type(hexagon_type);
    let hexagon = Hexagon { num: state.num_list[num_index] as usize, resource, hexagon_type, port: PortTypes::None};
    state.hexagon_list.push(hexagon);
    Some(hexagon)
}

fn neighbour_hexagons (state: &MapInfos, mut x: isize, mut y: isize) -> Vec<(usize, usize)>{
    let mut excludes: Vec<(usize, usize)> = Vec::new();
    for mov in [(2,0), (2,2), (-2,2), (-2,0), (-2,-2), (2,-2)] {
        x += mov.0;
        y += mov.1;
        for hex in &state.map[x as usize][y as usize].hexagons {
            if hex.num == 6 {push_if_not_exist(&mut excludes, (6, state.num_weights[6] as usize));}
            if hex.num == 8 {push_if_not_exist(&mut excludes, (4, state.num_weights[4] as usize));}
            if hex.num != 0 && state.num_weights[hex.num  - 2] != 0 {
                push_if_not_exist(&mut excludes, (hex.num -2, state.num_weights[hex.num -2] as usize));
            }
        }
    }
    excludes
}

fn save_weights(excludes: &Vec<(usize, usize)>, state: &mut MapInfos) {
    state.num_weight_tot -= excludes.iter().map(|data| data.1 as isize).sum::<isize>();
    excludes.iter().for_each(|data| state.num_weights[data.0] = 0);
}

fn merge_weights(excludes: &Vec<(usize, usize)>, state: &mut MapInfos) {
    state.num_weight_tot += excludes.iter().map(|data| data.1 as isize).sum::<isize>();
    excludes.iter().for_each(|data| state.num_weights[data.0] = data.1 as isize)
}

fn get_random_port(state: &mut MapInfos) -> PortTypes {
    let i = random_weighted_choice(&mut state.port_type_weights, &mut state.port_type_weight_tot, &mut state.rng);
    state.port_types[i]
}

fn add_ports(state: &mut MapInfos) {
    let data: [[usize; 4]; 9] = [
        [14,2, 16,2], [20,4, 22,6], [22,10, 20,12],
        [18,16, 16,18], [12,20, 10,20], [6,18, 4,16],
        [2,12, 0,10], [0,6, 2,4], [6,2, 8,2],
    ];
    let hexagon_indexes = [1,2,3,5,6,7,9,10,11];

    for (i,[x1,y1, x2, y2]) in data.into_iter().enumerate(){
        let port = get_random_port(state);
        state.map[x1][y1].port = port;
        state.map[x2][y2].port = port;
        state.hexagon_list[hexagon_indexes[i]].port = port;
    }
}

fn push_if_not_exist<T: PartialEq>(my_vec: &mut Vec<T>, mov: T) {
    if !my_vec.contains(&mov) { my_vec.push(mov);}
}

fn create_hexagon (state: &mut MapInfos, x: &mut isize, y: &mut isize, count: isize, plus_x: isize, plus_y: isize) -> bool{ 
    for _ in 0..count {
        *x += plus_x;
        *y += plus_y;
        let hexagon = match get_random_hexagon(state, *x, *y){
            Some(el) => el,
            None => return false,
        };
        for mov in [(2,0), (2,2), (-2,2), (-2,0), (-2,-2), (2,-2)] {
            push_if_not_exist(&mut state.map[*x as usize][*y as usize].neighbour_roads, ((*x + mov.0 / 2) as usize, (*y + mov.1 / 2) as usize));
            push_if_not_exist(&mut state.map[*x as usize][*y as usize].neighbour_houses, ((*x + mov.0) as usize, (*y + mov.1) as usize));
            *x += mov.0;
            *y += mov.1;
            let current_hexagon = &mut state.map[*x as usize][*y as usize];

            if hexagon.hexagon_type != HexagonTypes::Desert {
                current_hexagon.hexagons.push(hexagon);
            }
            push_if_not_exist(&mut current_hexagon.neighbour_roads, ((*x -mov.0 / 2) as usize, (*y -mov.1 / 2) as usize));
            push_if_not_exist(&mut current_hexagon.neighbour_houses, ((*x -mov.0) as usize, (*y -mov.1) as usize));
        }
    }
    true
}

fn right_up (state: &mut MapInfos, x: &mut isize, y: &mut isize, count: isize) -> bool {create_hexagon(state, x, y, count, -4,2)}
fn right(state: &mut MapInfos, x: &mut isize, y: &mut isize, count: isize) -> bool {create_hexagon(state, x, y, count, 0, 4)}
fn right_down(state: &mut MapInfos, x: &mut isize, y: &mut isize, count: isize) -> bool {create_hexagon(state, x, y, count, 4, 2)}   
fn left_down(state: &mut MapInfos, x: &mut isize, y: &mut isize, count: isize) -> bool {create_hexagon(state, x, y, count, 4, -2)}
fn left(state: &mut MapInfos, x: &mut isize, y: &mut isize, count: isize) -> bool {create_hexagon(state, x, y, count, 0, -4)}
fn left_up(state: &mut MapInfos, x: &mut isize, y: &mut isize, count: isize) -> bool {create_hexagon(state, x, y, count, -4, -2)}

pub fn random_base_map() -> MapInfos {
    'start: loop {
        let mut state = MapInfos::new();
        let mut x = 6;
        let mut y = -2;
        for i in 0..2 {
            if !right_down(&mut state, &mut x, &mut y, 3 - i) {continue 'start;}
            if !right(&mut state, &mut x, &mut y, 2 - i){continue 'start;}
            if !right_up(&mut state, &mut x, &mut y, 2 - i){continue 'start;}
            if !left_up(&mut state, &mut x, &mut y, 2 - i){continue 'start;}
            if !left(&mut state, &mut x, &mut y, 2 - i){continue 'start;}
            if !left_down(&mut state, &mut x, &mut y, 1 - i){continue 'start;}
        }
        if !right_down(&mut state, &mut x, &mut y, 1){continue 'start;}
        add_ports(&mut state);
        return state;
    }
}
