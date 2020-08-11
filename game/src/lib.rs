mod player;
mod board;
mod game;

use game::Game;
use board::Board;
use board::BoardState;

pub fn run() {
    // create a new board
    let game = Game::new("matt", "computer");

    // display it
    println!("{:?}", game.display());
}
