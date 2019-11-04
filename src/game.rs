use std::collections::HashMap;
use crate::card::*;
use crate::components::*;

pub struct Game {
    stock: Stock,
    waste: Waste,
    foundations: HashMap<Suit, Foundation>,
    tableaus: Vec<Tableau>,
}

impl Game {
    pub fn new() -> Self {
        let mut foundations = HashMap::new();
        for &i in Suit::all().iter() {
            foundations.insert(i, Foundation::new(i));
        }

        let mut stock = Stock::new_shuffled();
        let mut tableaus = Vec::new();

        let mut num_cards = 1;
        for _ in 0..7 {
            let mut cards = Vec::new();
            for i in 0..num_cards {
                let mut card = stock.draw().unwrap();
                if i == num_cards - 1 {
                    card.face_up = true;
                }
                cards.push(card);
            }
            num_cards += 1;
            tableaus.push(Tableau::new(cards));
        }

        Self {
            stock,
            waste: Waste::new(),
            foundations,
            tableaus,
        }
    }
}
