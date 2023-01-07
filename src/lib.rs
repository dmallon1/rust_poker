// use core::num;
// use std::sync::Arc;

// use compare::Compare;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::HashMap;
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
    // go through all players cards and generate the different hand possiblites
    // 7 choose 5 for both players

    let mut player_one_cards: Vec<Card> = game.players[0].hand.clone();
    let mut game_cards = game.shared_cards.clone();
    player_one_cards.append(&mut game_cards);
    println!("player 1 cards {:?}", player_one_cards);
    let combinations = player_one_cards.iter().combinations(5);
    println!("combinations");
    for v in combinations {
        println!("{:?}", v);
    }

    // sort each of the different combinations
    // run handrank on them
    // pull the top rank for all players
    // compare them and declare winner

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
pub fn rank_hand(hand: &[Card]) -> HandRank {
    // basically go through top to bottom and try to match each one,
    // should definitely be able to do this purely functional style, I'm just
    // using a mutable borrow.

    // TODO: sort elsewhere, not here, assume it's in ascending order

    let royal_straight = is_royal_straight(hand);
    let is_flush = is_flush(hand);

    if royal_straight && is_flush {
        return HandRank::RoyalFlush;
    }

    let is_straight = is_straight(hand);

    if is_straight && is_flush {
        return HandRank::StraightFlush;
    }

    // count the cards into a heap
    let mut hash_map: HashMap<&CardType, i32> = HashMap::new();
    for card in hand {
        let mut val = 1;
        if hash_map.contains_key(&card.card_type) {
            val = *hash_map.get_mut(&card.card_type).unwrap();
            val += 1;
        }
        hash_map.insert(&card.card_type, val);
    }

    // iterate (sorted by values)
    let mut three_of_a_kind = false;
    let mut num_pairs = 0;
    for (_, v) in hash_map.iter().sorted_by_key(|x| x.1).rev() {
        if *v == 4 {
            return HandRank::FourOfAKind;
        }
        if *v == 3 {
            three_of_a_kind = true;
        }
        if *v == 2 {
            num_pairs += 1;
        }
    }

    if three_of_a_kind && num_pairs > 0 {
        return HandRank::FullHouse;
    }

    if is_flush {
        return HandRank::Flush;
    }

    if is_straight {
        return HandRank::Straight;
    }

    if three_of_a_kind {
        return HandRank::ThreeOfAKind;
    }

    if num_pairs == 2 {
        return HandRank::TwoPair;
    }

    if num_pairs == 1 {
        return HandRank::Pair;
    }

    HandRank::HighCard
}

fn is_royal_straight(hand: &[Card]) -> bool {
    if hand[0].card_type != (CardType::Number { number: 10 }) {
        return false;
    }
    if hand[1].card_type
        != (CardType::Face {
            face_character: FaceCharacter::Jack,
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
            face_character: FaceCharacter::King,
        })
    {
        return false;
    }
    if hand[4].card_type
        != (CardType::Face {
            face_character: FaceCharacter::Ace,
        })
    {
        return false;
    }

    true
}

fn is_straight(hand: &[Card]) -> bool {
    for i in 0..4 {
        let current_card = &hand[i];
        let next_card = &hand[i + 1];

        let expected_next_card_type = get_next_card_type(&current_card.card_type);

        let next_card_type;
        match expected_next_card_type {
            None => return false,
            Some(card_type) => next_card_type = card_type,
        }

        if next_card.card_type != next_card_type {
            return false;
        }
    }

    true
}

fn get_next_card_type(card_type: &CardType) -> Option<CardType> {
    match card_type {
        CardType::Face { face_character } => match get_next_face_character(face_character) {
            Some(next_face_character) => {
                return Some(CardType::Face {
                    face_character: next_face_character,
                })
            }
            None => return None,
        },
        CardType::Number { number } => {
            if *number == 10 {
                return Some(CardType::Face {
                    face_character: FaceCharacter::Jack,
                });
            }
            return Some(CardType::Number { number: number + 1 });
        }
    }
}

/// Will return the next valid face character or None if the parameter is an Ace
fn get_next_face_character(face: &FaceCharacter) -> Option<FaceCharacter> {
    match face {
        FaceCharacter::Ace => None,
        FaceCharacter::King => Some(FaceCharacter::Ace),
        FaceCharacter::Queen => Some(FaceCharacter::King),
        FaceCharacter::Jack => Some(FaceCharacter::Queen),
    }
}

fn is_flush(hand: &[Card]) -> bool {
    let first_suit = &hand[0].suit;
    for i in 1..4 {
        if first_suit != &hand[i].suit {
            return false;
        }
    }
    true
}

#[derive(Debug, Eq, Clone)]
pub struct Card {
    pub suit: Suit,
    pub card_type: CardType,
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

#[derive(Debug, Eq, Hash, Clone)]
pub enum CardType {
    Face { face_character: FaceCharacter },
    Number { number: u8 },
}

impl Ord for CardType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            CardType::Face {
                face_character: self_face_character,
            } => match other {
                CardType::Face { face_character } => self_face_character.cmp(face_character),
                CardType::Number { number: _ } => Ordering::Greater,
            },
            CardType::Number {
                number: self_number,
            } => match other {
                CardType::Face { face_character: _ } => Ordering::Less,
                CardType::Number { number } => self_number.cmp(number),
            },
        }
    }
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CardType {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

#[derive(Debug, EnumIter, Clone, Copy, Eq, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum FaceCharacter {
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

// TODO: lowest to highest
#[derive(PartialEq, Debug)]
pub enum HandRank {
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
