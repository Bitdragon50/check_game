mod lib;
mod actors;
use crate::lib::Deck;
fn main() {
    let playing_deck = Deck::new();
    println!("{:#?}", &playing_deck);
    
    println!("{:#?}", playing_deck.shuffle_deck());
}
