use super::resources::Resources;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use super::helper_functions::random_weighted_choice;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Hexagon {
    pub num: usize,
    pub resource: Resources,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub hexagons: Vec<Hexagon>,
    pub is_housable: bool,
    pub port: Option<Resources>
}


pub fn create_random_map() -> Vec<Hexagon>{
    let resource_list = [Resources::Brick, Resources::Grain, Resources::Lumber, Resources::Ore, Resources::Wool];
    let mut resource_weights = [3,4,4,3,4];
    let mut resource_weight_tot = 18;
    
    let num_list = [2, 3,4,5,6, 8,9,10,11, 12];
    let mut num_weights = [1, 2,2,2,2, 2,2,2,2, 1];
    let mut num_weight_tot = 18;

    let mut rng =  SmallRng::from_entropy();
    let mut map: Vec<Hexagon> = Vec::new();
    for _ in 0..18 {
        let i = random_weighted_choice(&resource_weights, resource_weight_tot, &resource_list, &mut rng);
        let j = random_weighted_choice(&num_weights, num_weight_tot, &num_list, &mut rng);
        resource_weight_tot += -1;
        resource_weights[i] += -1;
        num_weight_tot += -1;
        num_weights[j] += -1;
        map.push(Hexagon { num: num_list[j], resource: resource_list[i]});
    }
    let i = rng.gen_range(0..map.len());
    map.insert(i, Hexagon { num: 0, resource: Resources::Desert});
    return map;
}

impl Point {
    pub fn new_points() {
        for val in create_random_map() {
            println!("{:?}", val);
        }
        
    }
}


