use rust_poker::*;

#[test]
fn rank_hand_royal_flush() {
    let mut hand = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Face {
                face_character: FaceCharacter::Jack,
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
                face_character: FaceCharacter::King,
            },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
    ];

    assert_eq!(HandRank::RoyalFlush, rank_hand(&mut hand));
}

#[test]
fn rank_hand_straight_flush() {
    let mut hand = vec![
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 4 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 5 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 6 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 7 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 8 },
        },
    ];

    assert_eq!(HandRank::StraightFlush, rank_hand(&mut hand));
}

#[test]
fn rank_hand_four_of_a_kind() {
    let mut hand = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Hearts,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
    ];

    assert_eq!(HandRank::FourOfAKind, rank_hand(&mut hand));
}

#[test]
fn rank_hand_full_house() {
    let mut hand = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Hearts,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
    ];

    assert_eq!(HandRank::FullHouse, rank_hand(&mut hand));
}

#[test]
fn rank_hand_flush() {
    let mut hand = vec![
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 2 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 5 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 6 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 8 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 10 },
        },
    ];

    assert_eq!(HandRank::Flush, rank_hand(&mut hand));
}

#[test]
fn rank_hand_straight() {
    let mut hand = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Face {
                face_character: FaceCharacter::Jack,
            },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Face {
                face_character: FaceCharacter::Queen,
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
                face_character: FaceCharacter::Ace,
            },
        },
    ];

    assert_eq!(HandRank::Straight, rank_hand(&mut hand));
}

#[test]
fn rank_hand_three_of_a_kind() {
    let mut hand = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Hearts,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Face {
                face_character: FaceCharacter::Queen,
            },
        },
    ];

    assert_eq!(HandRank::ThreeOfAKind, rank_hand(&mut hand));
}

#[test]
fn rank_hand_two_pair() {
    let mut hand = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 3 },
        },
        Card {
            suit: Suit::Hearts,
            card_type: CardType::Face {
                face_character: FaceCharacter::Queen,
            },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Face {
                face_character: FaceCharacter::Queen,
            },
        },
    ];

    assert_eq!(HandRank::TwoPair, rank_hand(&mut hand));
}

#[test]
fn rank_hand_pair() {
    let mut hand = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 3 },
        },
        Card {
            suit: Suit::Hearts,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Face {
                face_character: FaceCharacter::Queen,
            },
        },
    ];

    assert_eq!(HandRank::Pair, rank_hand(&mut hand));
}

#[test]
fn rank_hand_high_card() {
    let mut hand = vec![
        Card {
            suit: Suit::Spades,
            card_type: CardType::Number { number: 2 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Number { number: 3 },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 5 },
        },
        Card {
            suit: Suit::Hearts,
            card_type: CardType::Number { number: 7 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
    ];

    assert_eq!(HandRank::HighCard, rank_hand(&mut hand));
}
