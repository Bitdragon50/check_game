mod cards;
mod deck;
use std::{collections::HashMap, io::Error, time::{self, Duration, SystemTime}};
use cards::{Player, Table};
use redis::{Commands, Connection};
use deck::{Card, Unshuffled};
use crate::{cards::{give_card,  Board},deck::{Deck, Rank},};
use std::{collections::HashSet, error, io, num::ParseIntError, process::{Child, Command}};
use axum::{
    handler, http::StatusCode, response::Html, routing::{get, post}, serve, Json, Router
};
use serde::{Deserialize, Serialize};



#[tokio::main]
async fn main(){
    
    
    let playing_deck: Deck<Unshuffled> = Deck::new();
    let players: Vec<String> = vec!["Alan".to_owned(), "Mamitha".to_owned()];
    let (mut board , mut table, mut plays) = Board::new(playing_deck.shuffle_deck(), players);
    //let redis_table = con.set("table");
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


fn handler(table: &Table) ->  Html<String> {
    //let now = SystemTime::now();
    Html(format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Check Up Your Cards</title>
        <script>
            function refresh() {{
                window.location.reload();
            }}
            // Refresh the page every 30 seconds
            setInterval(refresh, 30000);
        </script>
    </head>
    <body>
        <h1>The board is {:#?}.</h1>
    </body>
    </html>
"#,table.cards[0].name()))
}















fn start_db() -> Result<(Child , Connection), Error> {
    let mut redis_server = Command::new("redis-server")
        .spawn()
        .expect("failed to start redis server");
    // Connect to the Redis server
    let client = redis::Client::open("redis://127.0.0.1/").expect("Can't start client");
    let con = client.get_connection().expect("Can't connect to db");
    Ok((redis_server,con))
}

fn main_v2() -> redis::RedisResult<()> {
    let (mut redis_server,mut con) = start_db()?;  
    
    let playing_deck: Deck<Unshuffled> = Deck::new();
    let players: Vec<String> = vec!["Alan".to_owned(), "Mamitha".to_owned()];
    let (mut board , mut table, mut plays) = Board::new(playing_deck.shuffle_deck(), players);


    
    let mut map = HashMap::new();
    map.insert("Table".to_string(), table.cards[0].name());
    
    let redis_table : () = con.set("table", map.get(&"Table".to_string()))?;
    let the_table: String = con.get("table")?;
    println!("{:#?}",the_table);

    // Set a key-value pair in Redis
    // let _: () = con.set("my_key", 42)?;

    // // Get the value back from Redis
    // let my_value: usize = con.get("my_key")?;

    // println!("Got value from Redis: {}", my_value);
    redis_server.kill().expect("failed to kill redis server");

    Ok(())
}

fn main_2() {
    
    let playing_deck: Deck<Unshuffled> = Deck::new();
    let players: Vec<String> = vec!["Alan".to_owned(), "Mamitha".to_owned()];
    let (mut board , mut table, mut plays) = Board::new(playing_deck.shuffle_deck(), players);
    //let redis_table = con.set("table");
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
