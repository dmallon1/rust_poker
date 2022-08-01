use rust_poker::Config;
use std::env;
use std::process;

fn main() {
    println!("Welcome to poker!");
    // take some command line arguments
    // number of players?
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem getting args: {}", err);
        process::exit(1);
    });

    println!("You've selected {} players.", config.number_of_players);

    // start game
    rust_poker::play_game(config.number_of_players);
}
