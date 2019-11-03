pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

impl Suit {
    pub fn all() -> [Self; 4] {
        [Self::Hearts, Self::Diamonds, Self::Clubs, Self::Spades]
    }
}

pub enum Value {
    Ace, Two, Three, Four, Five,
    Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King
}

impl Value {
    pub fn all() -> [Self; 13] {
        [
            Self::Ace, Self::Two, Self::Three, Self::Four, Self::Five,
            Self::Six, Self::Seven, Self::Eight, Self::Nine, Self::Ten,
            Self::Jack, Self::Queen, Self::King
        ]
    }
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
