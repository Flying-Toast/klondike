use crate::card::*;

pub trait Pile {
    fn cards(&self) -> &[Card];
    fn can_push(&self, card: &Card) -> bool;
    fn push(&mut self, card: Card);
}

pub struct Stock {
    cards: Vec<Card>,
}

impl Stock {
    pub fn new() -> Self {
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

    pub fn new_shuffled() -> Self {
        use rand::seq::SliceRandom;

        let mut rng = rand::thread_rng();
        let mut deck = Self::new();
        deck.cards.shuffle(&mut rng);

        deck
    }
}

impl Pile for Stock {
    fn cards(&self) -> &[Card] {
        &self.cards
    }

    fn can_push(&self, _card: &Card) -> bool {
        //can't ever push onto the stock
        false
    }

    fn push(&mut self, _card: Card) {
        panic!("Can't push onto stock.");
    }
}

pub struct Waste {
    cards: Vec<Card>,
}

impl Waste {
    pub fn new() -> Self {
        Self {
            cards: vec![]
        }
    }
}

impl Pile for Waste {
    fn cards(&self) -> &[Card] {
        &self.cards
    }

    fn can_push(&self, _card: &Card) -> bool {
        //can always push onto waste
        true
    }

    fn push(&mut self, card: Card) {
        self.cards.insert(0, card);
    }
}
