# Simple Poker Game in Rust

## To run project
> cargo run -- 2

where 2 represents the number of players in the game

## To run tests
> cargo test

Game Flow (MVP flow) - one round
1. Setup players, can do all humans to start and pass the computer around, could eventually get to ai
2. Start round and deal cards
    * deck is shuffled
    * dealer position set
    * each player gets two cards
    * first round, blinds and inital betting
    * flop: three cards are put face down
    * another round of betting, each player can check, bet (min bet is big blind), or fold
    * the turn: another card is put down
    * another round of betting
    * the river: final card is put down
    * final round of betting
    * show your cards, best five cards win
