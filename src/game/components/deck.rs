use rand::SeedableRng;
use rand::rngs::SmallRng;
use super::helper_functions::{random_weighted_choice, default_rng};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Card {
    Knight,
    VictoryPoint,
    YearOfPlenty,
    Monopoly,
    RoadBuilding,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    cards: [Card; 5],
    weights: [isize; 5],
    weight_tot: isize,
    #[serde(skip, default="default_rng")]
    rng: SmallRng,
}

impl Deck {
    pub fn new() -> Deck {
        const COUNT_KNIGHT: isize = 14;
        const COUNT_VICTORY_POINT: isize = 5;
        const COUNT_YEAR_OF_PLENTY: isize = 2;
        const COUNT_ROAD_BUILDING: isize = 2;
        const COUNT_MONOPOLY: isize = 2;
        Deck { 
            cards: [
                Card::Knight, Card::VictoryPoint, Card::YearOfPlenty, Card::RoadBuilding, Card::Monopoly], 
            weights: [
                COUNT_KNIGHT, COUNT_VICTORY_POINT, COUNT_YEAR_OF_PLENTY, COUNT_MONOPOLY, COUNT_ROAD_BUILDING],
            weight_tot: 25,
            rng: SmallRng::from_entropy(),
        }
    }
    pub fn get_card(&mut self) -> Card {
        let i = random_weighted_choice(&mut self.weights, &mut self.weight_tot, &mut self.rng);
        return self.cards[i];
    }
    #[inline(always)]
    pub fn is_fnished(&self) -> bool {
        self.weight_tot == 0
    }
}