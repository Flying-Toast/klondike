use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

#[derive(PartialEq)]
pub enum SuitColor {
    Red,
    Black
}

impl Suit {
    pub fn all() -> [Self; 4] {
        [Self::Hearts, Self::Diamonds, Self::Clubs, Self::Spades]
    }

    pub fn color(&self) -> SuitColor {
        match self {
            Self::Hearts | Self::Diamonds => SuitColor::Red,
            _ => SuitColor::Black
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Clubs => "♣️",
            Self::Diamonds => "♦️",
            Self::Hearts => "♥️",
            Self::Spades => "♠️"
        };

        write!(f, "{}", string)
    }
}

#[derive(PartialEq, Copy, Clone)]
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

    pub fn pred(&self) -> Self {
        if self == &Self::Ace {
            panic!("Can't get predecessor of an Ace.");
        }

        let all = Self::all();
        let index = all.iter()
            .position(|i| i == self).unwrap() - 1;

        all[index]
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Value::*;
        let string = match self {
            Ace => "A",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "J",
            Queen => "Q",
            King => "K"
        };

        write!(f, "{}", string)
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

    pub fn identifier(&self) -> String {
        if self.face_up {
            return format!("{}{}", self.value, self.suit);
        } else {
            return String::from("***");
        }
    }
}
