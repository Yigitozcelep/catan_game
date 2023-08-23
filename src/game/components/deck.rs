use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use super::helper_functions::random_weighted_choice;

#[derive(Clone, Copy)]
pub enum Card {
    Knight,
    VictoryPoint,
    YearOfPlenty,
    RoadBuilding,
    Monopoly,
}

pub struct Deck {
    cards: [Card; 5],
    weights: [isize; 5],
    weight_tot: isize,
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
                COUNT_KNIGHT, COUNT_VICTORY_POINT, COUNT_YEAR_OF_PLENTY, COUNT_ROAD_BUILDING, COUNT_MONOPOLY],
            weight_tot: 25,
            rng: SmallRng::from_entropy(),
        }
    }
    pub fn get_card(&mut self) -> Card {
        let i = random_weighted_choice(&self.weights, self.weight_tot, &self.cards, &mut self.rng);
        self.weights[i] += -1;
        self.weight_tot += -1;
        return self.cards[i];
    }
    #[inline(always)]
    pub fn is_fnished(&self) -> bool {
        self.weight_tot == 0
    }

}