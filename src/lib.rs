use actix::prelude::*;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::convert::TryInto;
use std::fmt::Debug;
use rand::seq::SliceRandom; // Trait that provides the shuffle method
use rand::thread_rng;       // Function that provides a random number generator


pub fn vec_to_array<Card: std::fmt::Debug, const N: usize>(mut vec: Vec<Card>) -> Result<[Card; N], Vec<Card>> {
    if vec.len() == N {
        let array: [Card; N] = vec.drain(..).collect::<Vec<_>>().try_into().expect("Length mismatch");
        Ok(array)
    } else {
        Err(vec)
    }
}



#[derive(Debug,EnumIter,Clone,PartialEq)]
pub enum Suit {
    Spade,
    Club,
    Diamond,
    Heart
}


#[derive(Debug,EnumIter,PartialEq)]
pub enum Rank {
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



#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}
impl Message for Card {
    type Result = Result<(), WrongCardErr>;
}

pub struct WrongCardErr;

// pub struct Joker {
//     suit: 
// }

#[derive(Debug)]
pub struct Shuffled;

#[derive(Debug)]
pub struct Unshuffled;

#[derive(Debug)]
pub struct Deck<T> {
    state: T,
    cards: [Card; 52]
}

impl Deck<Unshuffled> {
    pub fn new() -> Deck<Unshuffled> {
        let mut vec_cards: Vec<Card> = vec![];
        for suit in Suit::iter() {
                for rank in Rank::iter() {
                    vec_cards.push(Card { suit: suit.clone(), rank: rank })
                }
            }
            
        Deck {
            state: Unshuffled,
            cards: vec_to_array(vec_cards).unwrap()
        }
    }

    pub fn shuffle_deck(mut self) -> Deck<Shuffled> {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        Deck { state: Shuffled, cards: self.cards }
    }
}
