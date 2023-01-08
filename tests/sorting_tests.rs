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
