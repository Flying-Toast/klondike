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
                cards.insert(0, card);
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

    //DELETE
    pub fn debug_print(&self) {
        fn print_cards(cards: &[Card]) {
            for i in cards.iter() {
                print!("  ");
                if i.face_up {
                    print!("{}{}", i.value(), i.suit());
                } else {
                    print!("**");
                }
            }
            println!();
        }

        print!("Stock:");
        print_cards(&self.stock.cards());

        print!("Waste:");
        print_cards(&self.waste.cards());

        for (suit, f) in self.foundations.iter() {
            print!("{} Foundation:", suit);
            print_cards(f.cards());
        }

        for (index, t) in self.tableaus.iter().enumerate() {
            print!("Tableau {}:", index + 1);
            print_cards(t.cards());
        }
    }

    //DELETE
    pub fn debug_loop(&self) {
        use std::io::stdin;

        loop {
            self.debug_print();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
        }
    }
}
