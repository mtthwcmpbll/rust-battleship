mod ship;
mod player;
mod board;
mod game;

use crate::game::Game;

pub fn run() {
    // create a new board
    let game = Game::new("matt", "computer");

    // display it
    println!("{:?}", game.display());
}
