use std::collections::HashMap;
use crate::card::*;
use crate::components::*;
use crate::commands::*;

pub struct Game {
    pub stock: Stock,
    pub waste: Waste,
    pub foundations: HashMap<Suit, Foundation>,
    pub tableaus: Vec<Tableau>,
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

    fn display_top_or_empty(pile: &impl Pile) -> String {
        if pile.empty() {
            String::from("|   |")
        } else {
            format!("{}", pile.top().unwrap())
        }
    }

    fn render(&self) {
        let stock_rend = Self::display_top_or_empty(&self.stock);
        print!("{} {}      ", stock_rend, Self::display_top_or_empty(&self.waste));
        for suit in Suit::all().iter() {
            let fdn = self.foundations.get(suit).unwrap();
            if fdn.empty() {
                print!(" | {} |", suit);
            } else {
                print!("{}", fdn.top().unwrap());
            }
        }
        println!();
        println!();

        let max_len = self.tableaus.iter().map(|i| i.cards().len()).max().unwrap();
        for i in 1..max_len+1 {
            for t in self.tableaus.iter() {
                if t.cards().len() < i {
                    print!("     ");
                } else {
                    print!("{}", t.cards()[i-1]);
                }
            }
            println!();
        }
        println!();
    }

    pub fn play(&mut self) {
        use std::io::{stdin, stdout, Write};

        let mut valid = true;

        loop {
            if valid {
                self.render();
            }

            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            let command = line.trim();
            if let Err(result) = execute_command(command, self) {
                valid = false;
                print!("\x1B[1;31m");
                println!("{}", result);
                print!("\x1B[0m");
                stdout().flush().unwrap();
            } else {
                valid = true;
            }
        }
    }
}
