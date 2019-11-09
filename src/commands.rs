use std::io::{stdin, stdout, Write};
use crate::game::*;
use crate::components::*;

pub fn execute_command(command: &str, game: &mut Game) -> Result<(), String> {
    match command {
        "d" => cmd_draw(game),
        "m" => cmd_move(game),

        _ => Err(String::from("Invalid command."))
    }
}

fn get_pile<'a>(pile_ident: &str, game: &'a mut Game) -> Option<&'a mut dyn Pile> {
    //TODO
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
    let mut from_pile;
    let mut to_pile;

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
        to_pile.push(from_pile.draw().unwrap());
    } else {
        return Err(String::from("Illegal move."));
    }

    Ok(())
}
