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
                println!(
                    "You have been skipped with {:#?}.",
                    &table.cards[table.cards.len() - 1].name()
                );
                skipped = false;
                if pickup != 0 {
                    println!("You are picking up {}", &pickup);
                    give_card(pickup, player, &mut board);
                }
                continue;
            } else {
                println!("You have {:#?} cards in your hands.", &player.cards.len());
                println!("Your cards are {:#?}", player.cards);
                println!(
                    "There is {:#?} on the table. {:#?}",
                    table.cards[table.cards.len() - 1].name(),
                    table.cards[table.cards.len() - 1]
                );
                println!("What is the position of the card you wish to play {:#?}, write 0 if you have no card", player.name);

//============================================================================================================================
                
                let card_position = card_position_fn();

                

                match card_position { Ok(number) => match number {
                        0 => {
                                println!("{:#?} I'm giving you a card", player.name);
                                println!(
                                    "{:#?} is getting {:#?} from the board",
                                    player.name,
                                    board.cards.get(board.cards.len() - 1).unwrap().name()
                                );
                                give_card(1, player, &mut board);
                            }

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
                Err(err) => println!("You didn't type a number {err}")
                }

//==================================================================================================================

                if board.cards.len() < 6 { board.shuffle_board(&mut table) }
                if player.cards.len() == 0 {
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
