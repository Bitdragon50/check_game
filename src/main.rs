mod cards;
mod deck;
pub mod web;
//use cards::Player;
use deck::Unshuffled;
use crate::{
    cards::{give_card,  Board},
    deck::{Deck, Rank},
};
use std::{collections::HashSet, io, num::ParseIntError};
use axum::{self, response::{Html, IntoResponse}, routing::{get, post}, Form, Json, Router};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    console_error_panic_hook::set_once();
    //get players
    //let game = axum::Router::new().route("/", post(add_players(request)));
    // build our application with a single route
    let app = Router::new()
                                .route("/", get(serve_html_file))
                                .route("/submit", post(serve_html_file));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    let playing_deck: Deck<Unshuffled> = Deck::new();
    let players: Vec<String> = vec!["Alan".to_owned(), "Mamitha".to_owned()];
    let (mut board , mut table, mut plays) = Board::new(playing_deck.shuffle_deck(), players);
    
    let mut pickup: usize = 0;
    let mut skipped: bool = false;
    let mut gameover: bool = false;

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

async fn add_players(request: Json<String>) -> Vec<String> {
    let name_string = request.0;
    let name_vec = name_string.to_owned().split(",").map(|str| str.to_owned()).collect();
    name_vec
}

async fn greeting() -> String {
    "Hello Alan".to_owned()
}

use std::fs;

async fn serve_html_file() -> impl IntoResponse {
    let html_content = fs::read_to_string("src/template/index.html")
        .expect("Failed to read HTML file");
    Html(html_content)
}

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(serve_html_file));
//     // ... rest of your server setup
// }
