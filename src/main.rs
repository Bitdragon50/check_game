mod lib;
mod actors;
use crate::{actors::{play, take_card, Board, Player}, lib::Deck};
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

    while &plays[0].cards.len() != &(0 as usize) || &plays[1].cards.len() != &(0 as usize) /* No player has emptied their hands */ {
        playing_deck.play()        
    }
}

// Players play in turns
// Players can play cards to the table
// if a player has no valid card to play, they must pick up a card from the board

// The board divvies cards to the players and one to the table at the start of the game
// If a power card like 2 and 4 are played, then the board will give the next player the associated number of cards

// The table validates a play and announces each players turn