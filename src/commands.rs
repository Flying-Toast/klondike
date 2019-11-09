use crate::game::*;
use crate::components::*;

pub fn execute_command(command: &str, game: &mut Game) -> Result<(), String> {
    match command {
        "d" => cmd_draw(game),

        _ => Err(String::from("Invalid command."))
    }
}

fn cmd_draw(game: &mut Game) -> Result<(), String> {
    if let Some(card) = game.stock.draw() {
        game.waste.push(card);
    } else {
        return Err(String::from("Stock is empty."));
    }

    Ok(())
}
