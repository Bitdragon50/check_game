mod lib;
mod actors;
use crate::{actors::{take_card, Board,Table}, lib::Deck};
fn main() {
    let playing_deck = Deck::new();
   // println!("{:#?}", &playing_deck);
    
    //println!("{:#?}", playing_deck.shuffle_deck());
    let players = vec!["Alan".to_owned(), "Mamitha".to_owned()];

    let (mut board, mut table, mut plays) = Board::new(playing_deck.shuffle_deck(), players);
    
    println!("{:#?}", &table);
    println!("player 1 is {:#?}", &plays[0] );
    table.play(&mut plays[0]);
    println!("player 1 is {:#?}", &table  );//plays[0] )
    println!("player 1 is {:#?}", &plays[0] );
    println!("player 2 is {:#?}", &plays[1] );
    take_card (&mut board, &mut plays[1]);
    println!("{:#?} has these cards {:#?}", &plays[1].name , &plays[1].cards );
    // The table starts a turn
    // It requests a player to give it a card during its turn
    // if the player doesn't have a card which matches the suit or rank of the top card
    // Then it will ask the board to give the player a card
    // else if the player plays a power card, 
    // Then it will decode the power card and take the neccessary action like changing suits or asking the board to give cards to the next player
    // Else if the player plays a normal card, it will move on to the next player

}
