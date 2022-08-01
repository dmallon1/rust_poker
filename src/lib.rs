// use core::num;
// use std::sync::Arc;

// use compare::Compare;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{Eq, Ordering, PartialEq};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

pub struct Config {
    pub number_of_players: u8,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough command line args");
        }

        let number_of_players = args[1]
            .clone()
            .parse()
            .expect("couldn't parse number of players");

        Ok(Config { number_of_players })
    }
}

pub fn play_game(num_players: u8) -> Result<(), &'static str> {
    let mut game = Game::new(&num_players)?;
    game.shuffle_cards();

    println!("initializing cards, {:?}", game);

    println!("card count {}", game.cards.len());

    game.deal_cards();
    println!("initializing cards, {:?}", game);

    // for all rounds
    while game.is_valid() {
        match game.round {
            Round::PreFlop => {
                // do betting and choosing
                println!("pre flop");
                game.round = Round::Flop
            }
            Round::Flop => {
                // show three cards
                game.shared_cards
                    .push(game.cards.pop().expect("somehow ran out of cards"));
                game.shared_cards
                    .push(game.cards.pop().expect("somehow ran out of cards"));
                game.shared_cards
                    .push(game.cards.pop().expect("somehow ran out of cards"));
                println!("flop: {:?}", game.shared_cards);
                // do betting and choosing
                game.round = Round::Turn
            }
            Round::Turn => {
                game.shared_cards
                    .push(game.cards.pop().expect("somehow ran out of cards"));
                // do betting and choosing
                println!("turn: {:?}", game.shared_cards);
                game.round = Round::River
            }
            Round::River => {
                game.shared_cards
                    .push(game.cards.pop().expect("somehow ran out of cards"));
                // do betting and choosing
                println!("river: {:?}", game.shared_cards);
                break;
            }
        }
    }

    // figure out who won
    println!("player hands {:?}", game.players);

    println!("GAME OVER");
    Ok(())
}

#[derive(Debug)]
struct Game {
    cards: Vec<Card>,
    players: Vec<Player>,
    dealer_position: u8,
    round: Round,
    shared_cards: Vec<Card>,
}

impl Game {
    pub fn new(num_players: &u8) -> Result<Game, &'static str> {
        let mut cards = Vec::new();

        // initialize all number cards
        for suit in Suit::iter() {
            for n in 2..11 {
                cards.push(Card {
                    suit: suit,
                    card_type: CardType::Number { number: n },
                })
            }
        }

        // initialize all face chards
        for suit in Suit::iter() {
            for face in FaceCharacter::iter() {
                cards.push(Card {
                    suit: suit,
                    card_type: CardType::Face {
                        face_character: face,
                    },
                })
            }
        }

        // initialize players
        if num_players < &2 {
            return Err("not enough players");
        }

        Ok(Game {
            cards,
            players: (1..num_players + 1)
                .map(|_| Player { hand: vec![] })
                .collect(),
            dealer_position: 0,
            round: Round::PreFlop,
            shared_cards: vec![],
        })
    }

    fn shuffle_cards(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn deal_cards(&mut self) {
        for i in 0..self.players.len() {
            let player_hand = &mut self.players[i];

            let card1 = self.cards.pop().expect("somehow ran out of cards");
            let card2 = self.cards.pop().expect("somehow ran out of cards");

            player_hand.hand.push(card1);
            player_hand.hand.push(card2);
        }
    }

    fn is_valid(&self) -> bool {
        self.players.iter().filter(|p| p.hand.len() > 0).count() > 1
    }
}

/// Returns the highest hand rank given a 5 card hand
/// # Example
/// 2 Spades, 2 Hearts, Queen Clubs, Queen Hearts, Queen Spades -> FullHouse
fn rank_hand(hand: &mut [Card]) -> HandRank {
    // basically go through top to bottom and try to match each one,
    // should definitely be able to do this purely functional style, I'm just
    // using an immutable borrow.

    // first sort
    hand.sort();

    // royal flush
    let first_suit = hand[0].suit;
    // has to be an ace high straight with all the same suit
    // explict look for royal flush
    let is_highest_straight = false;

    if is_royal_flush(hand) {
        return HandRank::RoyalFlush;
    }

    for (i, card) in hand.iter().enumerate() {
        let res = match &card.card_type {
            CardType::Face { face_character } => match face_character {
                FaceCharacter::Ace => {
                    if i != 4 {
                        break;
                    }
                }
                FaceCharacter::King => {
                    if i != 3 {
                        break;
                    }
                }
                FaceCharacter::Jack => {
                    if i != 2 {
                        break;
                    }
                }
                FaceCharacter::Queen => {
                    if i != 1 {
                        break;
                    }
                }
            },
            CardType::Number { number } => {
                if *number != 10 {
                    // why?
                    break;
                }
            }
        };
        // if !res {
        //     break;
        // }

        // card[i] ==
    }

    let is_flush = hand.iter().all(|c| matches!(c.suit, first_suit));

    HandRank::HighCard
}

fn is_royal_flush(hand: &mut [Card]) -> bool {
    vec![
        FaceCharacter::Ace,
        FaceCharacter::King,
        FaceCharacter::Queen,
        FaceCharacter::Jack,
    ];
    if hand[0].card_type
        != (CardType::Face {
            face_character: FaceCharacter::Ace,
        })
    {
        return false;
    }
    if hand[1].card_type
        != (CardType::Face {
            face_character: FaceCharacter::King,
        })
    {
        return false;
    }
    if hand[2].card_type
        != (CardType::Face {
            face_character: FaceCharacter::Queen,
        })
    {
        return false;
    }
    if hand[3].card_type
        != (CardType::Face {
            face_character: FaceCharacter::Jack,
        })
    {
        return false;
    }
    if hand[4].card_type != (CardType::Number { number: 10 }) {
        return false;
    }
    true
}

#[derive(Debug, Eq)]
struct Card {
    suit: Suit,
    card_type: CardType,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
        // self.height.cmp(&other.height)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.card_type == other.card_type
    }
}

#[derive(Debug, Eq, PartialEq)]
enum CardType {
    Face { face_character: FaceCharacter },
    Number { number: u8 },
}

#[derive(Debug, EnumIter, Clone, Copy, Eq, PartialEq)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
enum FaceCharacter {
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug)]
struct Player {
    // could potentially make this an enum
    hand: Vec<Card>, // this could also be an array of size 5
}

#[derive(Debug)]
enum Round {
    PreFlop,
    Flop,
    Turn,
    River,
}

#[derive(PartialEq, Debug)]
enum HandRank {
    RoyalFlush = 10,
    StraightFlush = 9,
    FourOfAKind = 8,
    FullHouse = 7,
    Flush = 6,
    Straight = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    Pair = 2,
    HighCard = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_hand_royal_flush() {
        let mut hand = vec![
            Card {
                suit: Suit::Clubs,
                card_type: CardType::Face {
                    face_character: FaceCharacter::Ace,
                },
            },
            Card {
                suit: Suit::Clubs,
                card_type: CardType::Face {
                    face_character: FaceCharacter::King,
                },
            },
            Card {
                suit: Suit::Clubs,
                card_type: CardType::Face {
                    face_character: FaceCharacter::Queen,
                },
            },
            Card {
                suit: Suit::Clubs,
                card_type: CardType::Face {
                    face_character: FaceCharacter::Jack,
                },
            },
            Card {
                suit: Suit::Clubs,
                card_type: CardType::Number { number: 10 },
            },
        ];

        assert_eq!(HandRank::RoyalFlush, rank_hand(&mut hand));
    }

    #[test]
    fn rank_hand_straight_flush() {
        let mut hand = vec![
            Card {
                suit: Suit::Spades,
                card_type: CardType::Number { number: 8 },
            },
            Card {
                suit: Suit::Spades,
                card_type: CardType::Number { number: 7 },
            },
            Card {
                suit: Suit::Spades,
                card_type: CardType::Number { number: 6 },
            },
            Card {
                suit: Suit::Spades,
                card_type: CardType::Number { number: 5 },
            },
            Card {
                suit: Suit::Spades,
                card_type: CardType::Number { number: 4 },
            },
        ];

        assert_eq!(HandRank::StraightFlush, rank_hand(&mut hand));
    }

    #[test]
    fn rank_hand_high_card() {
        let mut hand = vec![
            Card {
                suit: Suit::Diamonds,
                card_type: CardType::Face {
                    face_character: FaceCharacter::Ace,
                },
            },
            Card {
                suit: Suit::Hearts,
                card_type: CardType::Number { number: 7 },
            },
            Card {
                suit: Suit::Clubs,
                card_type: CardType::Number { number: 5 },
            },
            Card {
                suit: Suit::Diamonds,
                card_type: CardType::Number { number: 3 },
            },
            Card {
                suit: Suit::Spades,
                card_type: CardType::Number { number: 2 },
            },
        ];

        assert_eq!(HandRank::HighCard, rank_hand(&mut hand));
    }

    //     #[test]
    //     fn case_insensitive() {
    //         let query = "rUsT";
    //         let contents = "\
    // Rust:
    // safe, fast, productive.
    // Pick three.
    // Trust me.";

    //         assert_eq!(
    //             vec!["Rust:", "Trust me."],
    //             search_case_insensitive(query, contents)
    //         );
    //     }
}
