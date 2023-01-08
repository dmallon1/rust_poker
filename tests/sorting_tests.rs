// use itertools::Itertools;
use std::cmp::Ordering;

use rust_poker::*;

#[test]
fn compare_card() {
    let card1 = Card {
        suit: Suit::Clubs,
        card_type: CardType::Face {
            face_character: FaceCharacter::Jack,
        },
    };
    let card2 = Card {
        suit: Suit::Diamonds,
        card_type: CardType::Face {
            face_character: FaceCharacter::Queen,
        },
    };

    assert_eq!(card1.cmp(&card2), Ordering::Less);
}

#[test]
fn sort_five_cards() {
    let mut five_cards = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 1 },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Number { number: 2 },
        },
        Card {
            suit: Suit::Hearts,
            card_type: CardType::Face {
                face_character: FaceCharacter::Queen,
            },
        },
    ];

    let manual_sorted_five_cards = vec![
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 1 },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Number { number: 2 },
        },
        Card {
            suit: Suit::Clubs,
            card_type: CardType::Number { number: 10 },
        },
        Card {
            suit: Suit::Hearts,
            card_type: CardType::Face {
                face_character: FaceCharacter::Queen,
            },
        },
        Card {
            suit: Suit::Diamonds,
            card_type: CardType::Face {
                face_character: FaceCharacter::Ace,
            },
        },
    ];

    five_cards.sort();

    assert_eq!(five_cards, manual_sorted_five_cards);
}

#[test]
fn compare_face_card_type() {
    let jack = CardType::Face {
        face_character: FaceCharacter::Jack,
    };
    let queen = CardType::Face {
        face_character: FaceCharacter::Queen,
    };
    let king = CardType::Face {
        face_character: FaceCharacter::King,
    };
    let ace = CardType::Face {
        face_character: FaceCharacter::Ace,
    };

    assert_eq!(ace.cmp(&jack), Ordering::Greater);
    assert_eq!(jack.cmp(&king), Ordering::Less);
    assert_eq!(queen.cmp(&king), Ordering::Less);
    assert_eq!(king.cmp(&queen), Ordering::Greater);
    assert_eq!(ace.cmp(&ace), Ordering::Equal);
}

#[test]
fn compare_number_card_type() {
    let five = CardType::Number { number: 5 };
    let seven = CardType::Number { number: 7 };
    let nine = CardType::Number { number: 9 };
    let two = CardType::Number { number: 2 };

    assert_eq!(seven.cmp(&five), Ordering::Greater);
    assert_eq!(two.cmp(&seven), Ordering::Less);
    assert_eq!(seven.cmp(&nine), Ordering::Less);
    assert_eq!(five.cmp(&two), Ordering::Greater);
    assert_eq!(two.cmp(&two), Ordering::Equal);
}

#[test]
fn compare_number_and_face_card_type() {
    let five = CardType::Number { number: 5 };
    let two = CardType::Number { number: 2 };
    let ace = CardType::Face {
        face_character: FaceCharacter::Ace,
    };
    let king = CardType::Face {
        face_character: FaceCharacter::King,
    };

    assert_eq!(ace.cmp(&five), Ordering::Greater);
    assert_eq!(two.cmp(&ace), Ordering::Less);
    assert_eq!(five.cmp(&king), Ordering::Less);
    assert_eq!(ace.cmp(&two), Ordering::Greater);
    assert_eq!(two.cmp(&two), Ordering::Equal);
}

#[test]
fn sort_inner_vector() {
    let foo = vec![vec![23, 7, 5], vec![44, 6, 22], vec![123, 2, 1]];
    let expected = vec![vec![5, 7, 23], vec![6, 22, 44], vec![1, 2, 123]];

    let bar: Vec<Vec<i32>> = foo
        .iter()
        .map(|v| {
            let mut v = v.to_vec();
            v.sort();
            v
        })
        .collect();

    assert_eq!(bar, expected)
}

#[test]
fn sort_inner_vector_in_place() {
    let mut foo = vec![vec![23, 7, 5], vec![44, 6, 22], vec![123, 2, 1]];
    let expected = vec![vec![5, 7, 23], vec![6, 22, 44], vec![1, 2, 123]];

    foo.iter_mut().for_each(|v| v.sort());

    assert_eq!(foo, expected)
}
