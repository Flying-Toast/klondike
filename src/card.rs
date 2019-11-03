pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

pub enum Value {
    Ace, Two, Three, Four, Five,
    Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King
}

pub struct Card {
    suit: Suit,
    value: Value,
    pub face_up: bool,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self {
            suit,
            value,
            face_up: false,
        }
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
