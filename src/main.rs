mod lib;
mod actors;
use crate::{actors::{play, take_card, Board}, lib::Deck};
fn main() {
    let playing_deck = Deck::new();
   // println!("{:#?}", &playing_deck);
    
    //println!("{:#?}", playing_deck.shuffle_deck());
    let players = vec!["Alan".to_owned(), "Mamitha".to_owned()];

    let (mut board, mut table, mut plays) = Board::new(playing_deck.shuffle_deck(), players);
    
    println!("{:#?}", &table);
    println!("player 1 is {:#?}", &plays[0] );
    play(&mut plays[0], &mut table, 2);
    println!("player 1 is {:#?}", &table  );//plays[0] )
    println!("player 1 is {:#?}", &plays[0] );
    println!("player 2 is {:#?}", &plays[1] );
    take_card (&mut board, &mut plays[1]);
    println!("{:#?} has these cards {:#?}", &plays[1].name , &plays[1].cards );
}
