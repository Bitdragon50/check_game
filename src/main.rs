mod lib;
mod actors;
use crate::{actors::{give_card, take_card, Board, Player, Table}, lib::Deck};
use std::{collections::HashMap, io, vec};
use std::collections::HashSet;



fn main() {
    let playing_deck = Deck::new();
   // println!("{:#?}", &playing_deck);
    
    //println!("{:#?}", playing_deck.shuffle_deck());
    let player_names = vec!["Alan".to_owned(), "Mamitha".to_owned()];

    let (mut board, mut table,mut players) = Board::new(playing_deck.shuffle_deck(), player_names);
    
    println!("{:#?}", &table);
    //table.play(&mut plays[0]);
   // println!("player 1 is {:#?}", &table  );//plays[0] )
    //println!("player 1 is {:#?}", &plays[0] );
   // println!("player 2 is {:#?}", &plays[1] );
   // take_card (&mut board, &mut plays[1]);
   // println!("{:#?} has these cards {:#?}", &plays[1].name , &plays[1].cards );
   

    loop { /*!players.iter().any(|player| !game(players[player.0].0)) &players[0].cards.len() != &(0 as usize) && &players[1].cards.len() != &(0 as usize)  No player has emptied their hands */ 
        //let players = players;
        
        
        for mut player in &players {

            // Player Maths
            let everyone: Vec<Player> = players.keys().cloned().collect();
            //let relationships = player.maths(&mut everyone);

            println!("It is {:#?}'s turn ", &player.0.name);
            println!("You have {:#?} cards in your hands.", &player.0.cards.len());
            println!("Your cards are {:#?}", &player.0.cards);
            println!("There is {:#?} on the table. {:#?}", table.cards[table.cards.len()-1].name(), table.cards[table.cards.len()-1]);
            println!("What is the position of the card you wish to play {:#?}, write 0 if you have no card", player.0.name);

            let mut card_position_input = String::new();
            io::stdin().read_line(&mut card_position_input)
            .expect("Failed to read from stdin");    
            let card_position: usize = card_position_input.trim().parse()
            .expect("Please type a number!");
       
            let mut next_player: &mut Player;

            
            // let position = players.iter().position(|current| current.1.0 == player).unwrap();       
            // if position == players.len()-1 {
                next_player = &mut players.get_mut(&player.0).unwrap().1;
                //player_after = players.get(1).unwrap();
            // } else if position == players.len()-2 {
            //     next_player = players.get_mut(position+1).unwrap();
            //     //player_after = players.get(0).unwrap()
            // } else {
            //     next_player = players.get_mut(position+1).unwrap();
            //     //player_after = players.get(position+2).unwrap()
            // }
            match card_position {
                        0 => {
                            println!("{:#?} I'm giving you a card", player.0.name);
                            println!("{:#?} is getting {:#?} from the board", player.0.name, board.cards.get(board.cards.len()-1).unwrap().name() );
                            give_card(1, &mut player.0, &mut board); },
                        1.. => {
                                let mut card = player.0.cards[card_position-1];
                                table.play(&mut player.0,&mut board, &mut card, &mut next_player)   }    }
                    }
                    
    }
}

pub fn game(checkdown: Player) -> bool {
    if checkdown.cards.len() == 0 {
        return true
    }
    false
    }

pub fn whos_next<'a>(current: &'a mut Player, players: &'a mut Vec<Player>) -> &'a mut Player {
    // find position of current
    let position = players.iter().position(|player| current == player).unwrap();
    let next: &mut Player;
    //let mut player_after: &Player;
    // check if position is at either ends
    if position == players.len()-1 {
        next = players.get_mut(0).unwrap();
        //player_after = players.get(1).unwrap();
    } else if position == players.len()-2 {
        next = players.get_mut(position+1).unwrap();
        //player_after = players.get(0).unwrap()
    } else {
        next = players.get_mut(position+1).unwrap();
        //player_after = players.get(position+2).unwrap()
    }
    next
    //(next,player_after)
    // what if the position is in the middle?
    // find next position
    // fetch player on that index
}



// struct Sect {
//     item: Vec<Player>
// }

//     pub fn not_self_list(not_self: &mut Vec<Player>) -> Sect {
//         Sect { item: not_self.to_vec() }
//     }

// impl Player {
//     fn self_list( self: &mut Player) -> Sect {
//         Sect { item: self.iter() }
//     }

//     pub fn maths (self: &mut Player , other_players: &mut Vec<Player>) -> HashMap<String, HashSet<Player>>{
//         let me: HashSet<Player> = self.self_list().item.iter().cloned().collect();
//         let you: HashSet<Player> = not_self_list(other_players).item.iter().cloned().collect();

//         // // Union
//         let union: HashSet<Player> = me.union(&you).cloned().collect();
//         println!("Union: {:?}", union);

//         // // Intersection
//         let intersection: HashSet<_> = me.intersection(&you).cloned().collect();
//         println!("Intersection: {:?}", intersection);

//         // // Difference
//         let difference: HashSet<_> = me.difference(&you).cloned().collect();
//         println!("Difference: {:?}", difference);

//         // // Symmetric Difference
//         let sym_diff: HashSet<_> = me.symmetric_difference(&you).cloned().collect();
//         println!("Symmetric Difference: {:?}", sym_diff);

//         let mut relationship = HashMap::new();
//         relationship.insert("union".to_owned(), union);
//         relationship.insert("intersection".to_owned(), intersection);
//         relationship.insert("difference".to_owned(), difference);
//         relationship.insert("symdif".to_owned(), sym_diff);
//         relationship


//     }
// }







// // fn maths (player_a: Player, player_b: Player) {
//     // let me: HashSet<Player> = Sect {item : vec![player_a]}.item.iter().cloned().collect();
//     // let you: HashSet<Player> = Sect {item : vec![player_b]}.item.iter().cloned().collect();

//     // // Union
//     // let union: HashSet<_> = me.union(&you).cloned().collect();
//     // println!("Union: {:?}", union);

//     // // Intersection
//     // let intersection: HashSet<_> = me.intersection(&you).cloned().collect();
//     // println!("Intersection: {:?}", intersection);

//     // // Difference
//     // let difference: HashSet<_> = me.difference(&you).cloned().collect();
//     // println!("Difference: {:?}", difference);

//     // // Symmetric Difference
//     // let sym_diff: HashSet<_> = me.symmetric_difference(&you).cloned().collect();
//     // println!("Symmetric Difference: {:?}", sym_diff);
// //}














// // Players play in turns
// // Players can play cards to the table
// // if a player has no valid card to play, they must pick up a card from the board

// // The board divvies cards to the players and one to the table at the start of the game
// // If a power card like 2 and 4 are played, then the board will give the next player the associated number of cards

// // The table validates a play and announces each players turn