mod cards;
mod deck;
use crate::{
    cards::{give_card,  Board},
    deck::{Deck, Rank},
};
use std::{collections::HashSet, io, num::ParseIntError};

fn main() {

    
    let playing_deck = Deck::new();

    let players = vec!["Alan".to_owned(), "Mamitha".to_owned()];

    let (mut board, mut table, mut plays) = Board::new(playing_deck.shuffle_deck(), players);

    println!("{:#?}", &table);
    println!("player 1 is {:#?}", &plays[0]);

    let mut pickup: usize = 0;
    let mut skipped: bool = false;
    let mut gameover = false;

    while !gameover {
        for player in &mut plays {
            println!("It is {:#?}'s turn ", player.name);
            if skipped {
                if let Some(top_card) = table.cards.last()  { 
                    println!( "You have been skipped with {:#?}.", top_card.name());
                    skipped = false; 
                }

                if pickup != 0 { 
                    println!("You are picking up {}", &pickup); 
                    give_card(pickup, player, &mut board); 
                }
                continue; 
            }

            else {

//============================================================================================================================
                println!("You have {:#?} cards in your hands.", &player.cards.len());
                println!("Your cards are {:#?}", player.cards);
                
                if let Some(top_card) = table.cards.last()  {
                    println!( "There is {:#?} on the table. {:#?}", top_card.name(), top_card );
                
                } else {
                    println!("There was no last card")
                }


                //println!( "There is {:#?} on the table. {:#?}", table.cards.last().unwrap().name(), table.cards.last().unwrap() );
                println!("What is the position of the card you wish to play {:#?}, write 0 if you have no card", player.name);

                match card_position_fn() { Ok(number) => match number {
                        0 => {
                                give_card(1, player, &mut board);
                            },

                        1.. => {
                                let mut card = player.cards[number - 1];
                                let mut power_cards = HashSet::new();
                                power_cards.insert(&Rank::Joker);
                                power_cards.insert(&Rank::Seven);
                                if power_cards.contains(&card.rank) {
                                    pickup = card.rank.pickup().unwrap();
                                    skipped = true
                                }

                                if card.rank == Rank::Ace {
                                    skipped = true
                                }
                                table.play(player, &mut board, &mut card)
                            }
                },
                Err(err) => { 
                    println!("You didn't type a number {err}");
                    give_card(1, player, &mut board);
                }
            }

//==================================================================================================================

                if board.cards.len() < 6 { board.shuffle_board(&mut table) }

                if player.cards.len() == 0 {
                    println!("{:#?} has won the game.", player.name);
                    gameover = true;
                }
            }
        }
    }
}




fn card_position_fn() -> Result<usize, ParseIntError> {
    let mut card_position_input = String::new();
    let _input = io::stdin().read_line(&mut card_position_input);
    let card_position = card_position_input
        .trim()
        .parse();
    match card_position {
        Ok(number) => Ok(number),
        Err(parse_int_error) => Err(parse_int_error)
    }

    }
