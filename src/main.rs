use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(EnumIter,IntoEnumIterator)]
enum Suit {
    Spade,
    Club,
    Diamond,
    Heart
}

enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

struct Card {
    suit: Suit,
    rank: Rank
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}


// struct Joker {
//     suit: 
// }

struct Shuffled;

struct Unshuffled;

struct Deck<T> {
    state: T,
    cards: [Card; 52]
}

impl Deck<Unshuffled> {
    fn new() -> Deck<Unshuffled> {
        Deck {
            state: Unshuffled,
            cards: 
        }
    }
}

fn main() {
    println!("Hello, world!");
}
