use rust_poker::Config;
use std::env;
use std::process;

fn main() {
    println!("--------------------------------------------------------------------------------");
    println!("Welcome to poker!");
    println!("--------------------------------------------------------------------------------");
    // take some command line arguments
    // number of players?
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|_| {
        eprintln!("problem getting args, expected something like \"cargo run -- 4\"");
        process::exit(1);
    });

    println!("You've selected {} players.", config.number_of_players);

    // start game
    let result = rust_poker::play_game(config.number_of_players);
    match result {
        Ok(_) => println!("thanks for playing"),
        Err(msg) => println!("{}", msg),
    }
}
