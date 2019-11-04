use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone)]
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

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Suit::Clubs => "♣️",
            Suit::Diamonds => "♦️",
            Suit::Hearts => "♥️",
            Suit::Spades => "♠️"
        };

        write!(f, "{}", string)
    }
}

#[derive(Copy, Clone)]
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

    ///The row of the rendered card that displays its identity (value and suit)
    pub fn ident_row(&self) -> String {
        let ident = format!("{}{}", self.value, self.suit);

        format!("│{:7}│", ident)
    }

    ///The top row of a rendered card
    pub fn top_row() -> String {
        String::from("┌──────┐")
    }

    pub fn empty_row() -> String {
        String::from("│      │")
    }

    pub fn bottom_row() -> String {
        String::from("└──────┘")
    }
}
