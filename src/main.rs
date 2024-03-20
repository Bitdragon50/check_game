mod lib;
mod actors;
use crate::{actors::Board, lib::Deck};
fn main() {
    let playing_deck = Deck::new();
   // println!("{:#?}", &playing_deck);
    
    //println!("{:#?}", playing_deck.shuffle_deck());
    let players = vec!["Alan".to_owned(), "Mamitha".to_owned()];

    let new_game = Board::new(playing_deck.shuffle_deck(), players);
    println!("{:#?}", &new_game);
}
