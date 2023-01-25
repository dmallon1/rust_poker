use itertools::{Combinations, Itertools};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::HashMap;
use std::collections::HashSet;
use std::{fmt, io};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

    game.deal_cards();

    // for all rounds
    while game.is_valid() {
        match game.round {
            Round::PreFlop => {
                // do betting and choosing
                println!("ROUND::pre flop");
                println!("---------------");
                game = game.run_game_loop();
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
                println!("ROUND::flop");
                println!("---------------");
                game = game.run_game_loop();
                game.round = Round::Turn
            }
            Round::Turn => {
                game.shared_cards
                    .push(game.cards.pop().expect("somehow ran out of cards"));
                // do betting and choosing
                println!("ROUND::turn");
                println!("---------------");
                game = game.run_game_loop();
                game.round = Round::River
            }
            Round::River => {
                game.shared_cards
                    .push(game.cards.pop().expect("somehow ran out of cards"));
                println!("ROUND::river");
                println!("---------------");
                game = game.run_game_loop();
                break;
            }
        }
    }

    // figure out who won
    println!("player hands {:?}", game.players);
    // go through all players cards and generate the different hand possiblites
    // 7 choose 5 for both players

    let mut player_one_cards: Vec<Card> = game.players[0].cards.clone();
    let mut game_cards = game.shared_cards.clone();
    player_one_cards.append(&mut game_cards);
    println!("player 1 cards {:?}", player_one_cards);

    // this is not correct, this will sometimes return combinations that are just the cards on the table and not any
    // in the players hands. The correct way would have to be
    // * both cards in hand + all combinations of remaining three
    // * one card in hard + all combinations of remaining four
    // * same thing for other card
    let combinations: Combinations<std::slice::Iter<Card>> =
        player_one_cards.iter().combinations(5);
    println!("combinations");

    // sort each of the different combinations
    let foo: Vec<Vec<&Card>> = combinations
        .into_iter()
        .map(|h| {
            let mut h = h.to_vec();
            h.sort();
            h
        })
        .collect();

    for v in foo {
        print_cards(&v);
        print!("rank: {:?}", rank_hand(&v));
        println!();
    }

    // run handrank on them
    // pull the top rank for all players
    // compare them and declare winner

    println!("GAME OVER");
    Ok(())
}

fn print_cards(cards: &Vec<&Card>) {
    for c in cards {
        print!("{} ", c);
    }
}

fn alternate_print_cards(cards: &Vec<Card>) {
    print!("current hand: ");
    for c in cards {
        print!("{} ", c);
    }
    println!();
}

fn print_table_cards(cards: &Vec<Card>) {
    print!("current cards... ");
    for c in cards {
        print!("{} ", c);
    }
    println!();
}

#[derive(Debug)]
struct Game {
    cards: Vec<Card>,
    players: Vec<Player>,
    round: Round,
    shared_cards: Vec<Card>,
    current_dealer: u16,
    current_pot: Vec<Chip>,
    small_blind: u16,
    big_blind: u16,
    folded_player_ids: HashSet<u16>,
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
                .map(|_| Player {
                    cards: vec![],
                    chips: vec![Chip::One, Chip::Five, Chip::TwentyFive, Chip::Fifty],
                })
                .collect(),
            round: Round::PreFlop,
            shared_cards: vec![],
            current_dealer: 0,
            current_pot: vec![],
            small_blind: 1,
            big_blind: 2,
            folded_player_ids: HashSet::new(),
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

            player_hand.cards.push(card1);
            player_hand.cards.push(card2);
        }
    }

    fn is_valid(&self) -> bool {
        self.players.iter().filter(|p| p.cards.len() > 0).count() > 1
    }

    fn print_game(&self) {
        println!("pot... {:?}", self.current_pot);
        // println!("current cards... {:?}", self.shared_cards);
        print_table_cards(&self.shared_cards);
        println!("dealer indicator... {:?}", self.current_dealer);
        println!(
            "big blin / small blind... {:?} / {:?}",
            self.small_blind, self.big_blind
        );
        println!("");
    }

    fn run_game_loop(mut self) -> Game {
        self.print_game();
        // allow all players to bet
        for i in 0..self.players.len() {
            let player = self.players.get(i).expect("should be");
            if self.folded_player_ids.contains(&i.try_into().unwrap()) {
                continue;
            }

            println!("player {}", i + 1);
            alternate_print_cards(&player.cards);
            println!("chips: {:?}", player.chips);
            println!("Choose your move:");
            println!("- c for check");
            println!("- b to bet");
            println!("- f to fold");
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("failed to pull guess");
            println!("input: {}", line);

            // process input
            if line.trim() == "f" {
                println!("player {} folded", i + 1);
                let player_id: u16 = i.clone().try_into().unwrap();
                self.folded_player_ids.insert(player_id);
            }
        }
        self
    }
}

/// Returns the highest hand rank given a 5 card hand
/// # Example
/// 2 Spades, 2 Hearts, Queen Clubs, Queen Hearts, Queen Spades -> FullHouse
pub fn rank_hand(hand: &Vec<&Card>) -> HandRank {
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

fn is_royal_straight(hand: &Vec<&Card>) -> bool {
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

fn is_straight(hand: &Vec<&Card>) -> bool {
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

fn is_flush(hand: &Vec<&Card>) -> bool {
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
        self.card_type.cmp(&other.card_type)
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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {:?}]", self.card_type, self.suit)
    }
}

#[derive(Debug, Eq, Hash, Clone, PartialOrd, PartialEq)]
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

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardType::Face { face_character } => write!(f, "{:?}", face_character),
            CardType::Number { number } => write!(f, "{:?}", number),
        }
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
    cards: Vec<Card>, // this could also be an array of size 5
    chips: Vec<Chip>,
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

#[derive(Debug)]
pub enum Chip {
    One,
    Five,
    TwentyFive,
    Fifty,
}
