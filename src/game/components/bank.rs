


// bankada bitersenin cevapları tek ve çok lu oyuncu içi değişiyor
// https://www.reddit.com/r/Catan/comments/4aenhb/ran_out_of_resource_cards/
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bank {
    resources: [usize; 5] // grain, lumber, brick, ore, wool
}

impl Bank {
    pub fn new() -> Bank {
        Bank {resources: [19,19,19,19,19]}
    }
}

