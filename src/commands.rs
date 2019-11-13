use std::io::{stdin, stdout, Write};
use crate::game::*;
use crate::components::*;
use crate::card::*;

pub fn execute_command(command: &str, game: &mut Game) -> Result<(), String> {
    match command {
        "d" => cmd_draw(game),
        "m" => cmd_move(game),
        "h" => cmd_help(),

        _ => Err(String::from("Invalid command. Type 'h' for help."))
    }
}

fn get_pile_mut<'a>(pile_ident: &str, game: &'a mut Game) -> Option<&'a mut dyn Pile> {
    match pile_ident {
        "s" => return Some(&mut game.stock),
        "w" => return Some(&mut game.waste),
        _ => {}
    }

    if pile_ident.len() == 2 {
        match pile_ident.as_bytes()[0] as char {
            'f' => {
                match pile_ident.as_bytes()[1] as char {
                    'h' => return Some(game.foundations.get_mut(&Suit::Hearts).unwrap()),
                    's' => return Some(game.foundations.get_mut(&Suit::Spades).unwrap()),
                    'd' => return Some(game.foundations.get_mut(&Suit::Diamonds).unwrap()),
                    'c' => return Some(game.foundations.get_mut(&Suit::Clubs).unwrap()),

                    _ => return None
                }
            },
            't' => {
                if let Ok(num) = (pile_ident.as_bytes()[1] as char).to_string().parse::<usize>() {
                    if num < 7 {
                        return Some(game.tableaus.get_mut(num).unwrap());
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            },

            _ => {}
        }
    }

    None
}

//TODO: How to do this without entirely rewriting get_pile_mut?
fn get_pile<'a>(pile_ident: &str, game: &'a Game) -> Option<&'a dyn Pile> {
    match pile_ident {
        "s" => return Some(&game.stock),
        "w" => return Some(&game.waste),
        _ => {}
    }

    if pile_ident.len() == 2 {
        match pile_ident.as_bytes()[0] as char {
            'f' => {
                match pile_ident.as_bytes()[1] as char {
                    'h' => return Some(game.foundations.get(&Suit::Hearts).unwrap()),
                    's' => return Some(game.foundations.get(&Suit::Spades).unwrap()),
                    'd' => return Some(game.foundations.get(&Suit::Diamonds).unwrap()),
                    'c' => return Some(game.foundations.get(&Suit::Clubs).unwrap()),

                    _ => return None
                }
            },
            't' => {
                if let Ok(num) = (pile_ident.as_bytes()[1] as char).to_string().parse::<usize>() {
                    if num < 7 {
                        return Some(game.tableaus.get(num).unwrap());
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            },

            _ => {}
        }
    }

    None
}

fn cmd_draw(game: &mut Game) -> Result<(), String> {
    if let Some(card) = game.stock.draw() {
        game.waste.push(card);
    } else {
        return Err(String::from("Stock is empty."));
    }

    Ok(())
}

fn cmd_move(game: &mut Game) -> Result<(), String> {
    let from_pile;
    let to_pile;

    let mut from = String::new();
    print!("\tFrom: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut from).unwrap();
    if let Some(pile) = get_pile(from.trim(), game) {
        from_pile = pile;
    } else {
        return Err(String::from("No such pile."));
    }

    let mut to = String::new();
    print!("\tTo: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut to).unwrap();
    if let Some(pile) = get_pile(to.trim(), game) {
        to_pile = pile;
    } else {
        return Err(String::from("No such pile."));
    }


    if from_pile.empty() {
        return Err(String::from("From pile is empty."));
    }
    if to_pile.can_push(from_pile.top().unwrap()) {
        let card = get_pile_mut(from.trim(), game).unwrap().draw().unwrap();
        get_pile_mut(to.trim(), game).unwrap().push(card);
    } else {
        return Err(String::from("Illegal move."));
    }

    Ok(())
}

fn cmd_help() -> Result<(), String> {
    println!("Commands:");
    println!("\td - Draw a card from the stock to the waste.");
    println!("\tm - Move the specified card to the specified location.");
    println!("\t\tTo specify a card/location:");
    println!("\t\t\ts - stock");
    println!("\t\t\tw - waste");
    println!("\t\t\tfs - spades foundation");
    println!("\t\t\tfh - hearts foundation");
    println!("\t\t\tfd - diamonds foundation");
    println!("\t\t\tfc - clubs foundation");
    println!("\t\t\tt[0-6] - tableau pile x");

    Err(String::new())
}
