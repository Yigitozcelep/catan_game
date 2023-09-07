use catan_game::game::components::map_creation;
use catan_game::game::components::resources::HexagonTypes;

#[test]
fn just_call_map_creation() {
    for _ in 0..1000 {
        map_creation::random_base_map();
    }
}

#[test]
fn check_6_8_and_same_num() {
    for _ in 0..1000 {
        let res = map_creation::random_base_map();
        for el in &res.map {
            for point in el {
                let mut data: Vec<usize> = Vec::new();
                for hex in &point.hexagons {
                    if data.contains(&hex.num) {panic!();}
                    if hex.num == 8 && data.contains(&6) {panic!();}
                    if hex.num == 6 && data.contains(&8) {panic!();}
                    data.push(hex.num);
                }
            }
        }
    }
}

#[test]
fn check_num_counts() {
    for _ in 0..1000 {
        let mut data = [0,0,6,12,12,12,12,0,12,12,12,12,6];
        let res = map_creation::random_base_map();
        for el in &res.map {
            for point in el {
                for hex in &point.hexagons {
                    data[hex.num] += -1;
                    if data[hex.num] < 0 {panic!();}
                }
            }
        }
    }
}

fn get_index(hex: HexagonTypes) -> usize {
    match hex {
        HexagonTypes::Desert => 0,
        HexagonTypes::Brick => 1,
        HexagonTypes::Ore => 2,
        HexagonTypes::Wool => 3,
        HexagonTypes::Grain => 4,
        HexagonTypes::Lumber => 5,
    }
}

#[test]
fn check_hexagon_counts() {
    for _ in 0..1000 {
        let mut data = [1,3,3,4,4,4];
        let res = map_creation::random_base_map();
        for el in &res.hexagon_list {
            let i = get_index(el.hexagon_type);
            data[i] += -1;
            if data[i] < 0 {panic!();}
        }
        
        for num in data {
            if num != 0 {panic!();}
        }
    }
    
}