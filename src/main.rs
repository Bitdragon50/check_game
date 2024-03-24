mod lib;
mod actors;
use crate::{actors::{give_card, take_card, Board, Table}, lib::Deck};
use std::io;
fn main() {
    let playing_deck = Deck::new();
   // println!("{:#?}", &playing_deck);
    
    //println!("{:#?}", playing_deck.shuffle_deck());
    let players = vec!["Alan".to_owned(), "Mamitha".to_owned()];

    let (mut board, mut table, mut plays) = Board::new(playing_deck.shuffle_deck(), players);
    
    println!("{:#?}", &table);
    println!("player 1 is {:#?}", &plays[0] );
    //table.play(&mut plays[0]);
   // println!("player 1 is {:#?}", &table  );//plays[0] )
    //println!("player 1 is {:#?}", &plays[0] );
   // println!("player 2 is {:#?}", &plays[1] );
   // take_card (&mut board, &mut plays[1]);
   // println!("{:#?} has these cards {:#?}", &plays[1].name , &plays[1].cards );

    while &plays[0].cards.len() != &(0 as usize) && &plays[1].cards.len() != &(0 as usize) /* No player has emptied their hands */ {
        for player in &mut plays{
            println!("It is {:#?}'s turn ", player.name);
            println!("You have {:#?} cards in your hands.", &player.cards.len());
            println!("Your cards are {:#?}", player.cards);
            println!("There is {:#?} on the table. {:#?}", table.cards[table.cards.len()-1].name(), table.cards[table.cards.len()-1]);
            println!("What is the position of the card you wish to play {:#?}, write 0 if you have no card", player.name);

            let mut card_position_input = String::new();
            io::stdin().read_line(&mut card_position_input)
            .expect("Failed to read from stdin");    
            let card_position: usize = card_position_input.trim().parse()
            .expect("Please type a number!");

            match card_position {
                0 => {
                    println!("{:#?} I'm giving you a card", player.name);
                    println!("{:#?} is getting {:#?} from the board", player.name, board.cards.get(board.cards.len()-1).unwrap().name() );
                    give_card(1, player, &mut board); },
                1.. => {
                        let mut card = player.cards[card_position-1];
                        table.play(player,&mut board, &mut card)   }    }
            }
    }
}

// Players play in turns
// Players can play cards to the table
// if a player has no valid card to play, they must pick up a card from the board

// The board divvies cards to the players and one to the table at the start of the game
// If a power card like 2 and 4 are played, then the board will give the next player the associated number of cards

// The table validates a play and announces each players turn