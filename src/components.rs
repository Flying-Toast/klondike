use crate::card::*;

pub struct Stack {
    pub cards: Vec<Card>,
}

impl Stack {
    pub fn new_deck() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in &Suit::all() {
            for value in &Value::all() {
                cards.push(Card::new(*suit, *value));
            }
        }

        Self {
            cards
        }
    }

    pub fn new_shuffled_deck() -> Self {
        use rand::seq::SliceRandom;

        let mut rng = rand::thread_rng();
        let mut deck = Self::new_deck();
        deck.cards.shuffle(&mut rng);

        deck
    }
}
