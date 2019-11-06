use crate::card::*;

pub trait Pile {
    fn cards(&self) -> &[Card];
    fn can_push(&self, card: &Card) -> bool;
    //cards are pushed to the back of the `cards` vector, which is the top of the pile
    fn push(&mut self, card: Card);
}

pub struct Stock {
    cards: Vec<Card>,
}

impl Stock {
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

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
        self.cards.push(card);
    }
}

pub struct Foundation {
    suit: Suit,
    cards: Vec<Card>,
    top_value: Option<Value>,
}

impl Foundation {
    pub fn new(suit: Suit) -> Self {
        Self {
            suit,
            cards: vec![],
            top_value: None,
        }
    }
}

impl Pile for Foundation {
    fn cards(&self) -> &[Card] {
        &self.cards
    }

    fn can_push(&self, card: &Card) -> bool {
        if &self.suit != card.suit() {
            return false
        }

        if let Some(v) = self.top_value {
            return card.value().pred() == v;
        } else {
            return card.value() == &Value::Ace;
        }
    }

    fn push(&mut self, card: Card) {
        self.top_value = Some(*card.value());
        self.cards.push(card);
    }
}

pub struct Tableau {
    cards: Vec<Card>,
}

impl Tableau {
    pub fn new(initial_cards: Vec<Card>) -> Self {
        Self {
            cards: initial_cards,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.len() == 0
    }

    pub fn top_card(&self) -> &Card {
        &self.cards.get(self.cards.len() - 1).unwrap()
    }
}

impl Pile for Tableau {
    fn cards(&self) -> &[Card] {
        &self.cards
    }

    fn can_push(&self, card: &Card) -> bool {
        if self.is_empty() {
            return card.value() == &Value::King;
        } else if !self.top_card().face_up || self.top_card().value() == &Value::Ace {
            return false;
        } else {
            return self.top_card().suit().color() != card.suit().color() &&
                &self.top_card().value().pred() == card.value();
        }
    }

    fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
}
