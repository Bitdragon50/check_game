use actix::prelude::*;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;
use std::io; //any::Any, collections::HashSet, 
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



#[derive(Debug,EnumIter,Clone,PartialEq,Eq, PartialOrd, Ord, Copy,Hash)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
    Red,
    Black
}


#[derive(Debug,EnumIter,PartialEq,Clone,PartialOrd,Eq,Ord,Copy,Hash)]
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
    King,
    Joker
}



#[derive(Debug,Clone,PartialEq,PartialOrd, Eq,Ord, Hash, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
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
    cards: [Card; 54]
}

impl Deck<Unshuffled> {
    pub fn new() -> Deck<Unshuffled> {
        let mut vec_cards: Vec<Card> = vec![];
        for suit in Suit::iter() {
            if (suit != Suit::Red) && (suit != Suit::Black) {
                for rank in Rank::iter() {
                    if rank != Rank::Joker {
                    vec_cards.push(Card { suit: suit.clone(), rank: rank })
                }
                }}
            }
        // Create Jokers
        vec_cards.push(Card {
            rank: Rank::Joker,
            suit: Suit::Black
        });
        vec_cards.push(Card {
            rank: Rank::Joker,
            suit: Suit::Red
        });
            
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

impl Deck<Shuffled> {
    pub fn to_vec(self) -> Vec<Card> {
        let card_vec = self.cards.to_vec();
        card_vec
    }
}

impl FromIterator<Card> for Card {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut cards: Vec<Card> = iter.into_iter().collect();
        cards.sort();
        Card { suit: cards[0].suit, rank: cards[0].rank }
    }
}

impl Card {
    pub fn name(&self) -> String {
        format!("{:#?} of {:#?}", &self.rank , &self.suit)
    }
}

impl Rank {
    pub fn pickup(&self) -> Option<usize> {
        if self == &Rank::Joker {
            return Some(4)
        } else if self == &Rank::Seven {
            return Some(2)
        } else {
            return None;
        }
    }



    pub fn change_to(&self) -> Option<Suit> {
        
            println!("What suit do you want?");
            let mut suit_input = String::new();

            io::stdin().read_line(&mut suit_input)
            .expect("Failed to read from stdin");


            //let mut suit: Suit;
            let spades = "spades".to_owned();
            let hearts = "hearts".to_owned();
            let clubs = "clubs".to_owned();
            let diamonds = "diamonds".to_owned();

            let mut suit_map = HashMap::new();
            suit_map.insert(spades, Suit::Spades);
            suit_map.insert(hearts, Suit::Hearts);
            suit_map.insert(diamonds, Suit::Diamonds);
            suit_map.insert(clubs, Suit::Clubs);
            suit_input = suit_input.trim().to_string();

            suit_map.get_mut(&suit_input).copied()
            
            // if suit_input.to_lowercase() == "spades".to_string() {
            //     return Some(Suit::Spades) 
            // } else if suit_input.to_lowercase() == "clubs".to_string() {
            //     return Some(Suit::Clubs)                
            // } else if suit_input.to_lowercase() == "hearts".to_string() {
            //     return Some(Suit::Hearts)
            // } else if suit_input.to_lowercase() == "diamonds".to_string() {
            //     return Some(Suit::Diamonds)
            // } else {
            //     return None
            // }
    }
    
}