use std::collections::VecDeque;

use catan_game::game::components::map_creation;

#[test]
#[ignore]
fn test_base_map() {
    let res = map_creation::base_map_frame();
    let json = serde_json::to_string(&res.hexagon_list).unwrap();

    println!("{:?}", json);
    let mut moved: Vec<(isize,isize)> = Vec::new();
    let mut dq: VecDeque<(isize, isize)> = VecDeque::new();
    dq.push_back((10,0));
    while !dq.is_empty() {
        let (x,y) = dq.pop_back().unwrap();
        if moved.contains(&(x,y)) {continue;}
        let current = &res.map[x as usize][y as usize];
        println!("\n**************************************\n({},{})\n, {:?}\n", x,y, current);
        moved.push((x,y));
        for mov in &current.valid_moves {
            dq.push_back((x + mov.0 * 2, y + mov.1 * 2));
        }
    } 
}

#[test]
fn just_call_map_creation() {
    map_creation::base_map_frame();
}

