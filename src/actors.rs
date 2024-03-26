use std::{collections::{HashMap, HashSet}, io};

use crate::lib::{Card, Deck, Rank, Shuffled};

#[derive(Debug)]
pub struct Board {
    pub cards: Vec<Card>
}

#[derive(Debug)]
pub struct Table {
    pub cards: Vec<Card>
}

#[derive(Debug,Clone,PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>
}


// impl Iterator for PlayerIterator {
//     type Item = Player;

//     fn next(&mut self) -> Option<Self::Item> {
//         if 
//     }
// }

// impl IntoIterator for Player {
//     type Item = Player;
//     type IntoIter = PlayerIterator;

//     fn into_iter(self) -> self::IntoIter{
//         PlayerIterator {}
//     }
// }

impl Board {
    pub fn new(deck: Deck<Shuffled>, players: Vec<String>) -> (Board, Table, HashMap<Player,(Player,Player,Player)>) {
        let mut deck_vec = deck.to_vec();
        let mut player_vec = vec![];

        for player in players {
            let players_card = deck_vec.drain(..4).collect();
            player_vec.push(Player{name: player , cards: players_card})
        }
        let mut map = HashMap::new();
        let mut next_player: Player;
        let mut player_after: Player;
        for player in &player_vec {
                next_player = player.clone().next_players(&player_vec);
                player_after = next_player.next_players(&player_vec);
                map.insert(player.clone(), (player.clone(), next_player, player_after));
            }

        let first_card = vec![deck_vec.pop().unwrap()];
        return (Board{cards: deck_vec} ,Table{cards: first_card } ,map);
        }
    }


impl Table {
        pub fn play(&mut self, player: &mut Player, board: &mut Board, card: &mut Card, next: &mut Player) {
            println!("Player {:#?} plays {:#?}", &player.name, card.name());
         
            if self.playable(card){ 
                println!("Player {:#?} plays {:#?}",&player, card);
                match card.rank {
                    Rank::Ace => { 
                        self.cards.push(player.cards.remove(player.cards.iter().position(|iter_card| iter_card == card).unwrap()));
                        self.play(player, board, card, next)
                    } ,
                    Rank::Two => {
                        self.cards.push(player.cards.remove(player.cards.iter().position(|iter_card| iter_card == card).unwrap()));
                        give_card(2, next, board);                        
                        self.play(player, board, card, next)
                    },
                    Rank::Four => {
                        self.cards.push(player.cards.remove(player.cards.iter().position(|iter_card| iter_card == card).unwrap()));
                        give_card(4, next, board);                        
                        self.play(player, board, card, next)
                    },
                    Rank::Jack => {
                        self.cards.push(player.cards.remove(player.cards.iter().position(|iter_card| iter_card == card).unwrap()));                        
                    },
                    _ => {
                        self.cards.push(player.cards.remove(player.cards.iter().position(|iter_card| iter_card == card).unwrap()));                        
                    }
                }
                // self.cards.push(player.cards.remove(player.cards.iter().position(|iter_card| iter_card == card).unwrap())); 
            } else {
                println!("{:#?} you can't play {:#?} on {:#?}", player.name, card.name(), self.cards.get(self.cards.len()-1).unwrap().name());
                println!("{:#?} is getting {:#?} from the board", player.name, board.cards.get(board.cards.len()-1).unwrap().name() );
                give_card(1, player, board)
            }
        }

        fn playable(&mut self, card: &Card) -> bool {
            self.cards[self.cards.len()-1].rank == card.rank || self.cards[self.cards.len()-1].suit == card.suit 
        }
}

pub fn take_card(board: &mut Board, player: &mut Player) {
            player.cards.push(board.cards.pop().unwrap())
        }

pub fn give_card(amount: usize, player: &mut Player, board: &mut Board) {
    player.cards.push(board.cards.drain(..amount).collect())
}

struct Sect {
    item: HashSet<Vec<Player>>
}

    pub fn not_self_list(not_self: &mut Vec<Player>) -> Sect {
        Sect { item: not_self.to_vec() }
    }

impl Player {

    pub fn next_players(&mut self, players: &Vec<Player>) -> Player {
        // find position of current
        let position = players.iter().position(|player| self == player).unwrap();
        let next: &Player;
        //let mut player_after: &Player;
        // check if position is at either ends
        if position == players.len()-1 {
            next = players.get(0).unwrap();
            //player_after = players.get(1).unwrap();
        } else if position == players.len()-2 {
            next = players.get(position+1).unwrap();
            //player_after = players.get(0).unwrap()
        } else {
            next = players.get(position+1).unwrap();
            //player_after = players.get(position+2).unwrap()
        }
        next.clone()
        //(next,player_after)
        // what if the position is in the middle?
        // find next position
        // fetch player on that index
    }

    //pub fn maths (self: &mut Player , other_players: &mut Vec<Player>) -> HashMap<String, HashSet<Player>>{
    //     let me: HashSet<Player> = self.self_list().iter().cloned().collect();
    //     let you: HashSet<Player> = not_self_list(other_players).item.iter().cloned().collect();

    //     // // Union
    //     let union: HashSet<Player> = me.union(&you).cloned().collect();
    //     println!("Union: {:?}", union);

    //     // // Intersection
    //     let intersection: HashSet<_> = me.intersection(&you).cloned().collect();
    //     println!("Intersection: {:?}", intersection);

    //     // // Difference
    //     let difference: HashSet<_> = me.difference(&you).cloned().collect();
    //     println!("Difference: {:?}", difference);

    //     // // Symmetric Difference
    //     let sym_diff: HashSet<_> = me.symmetric_difference(&you).cloned().collect();
    //     println!("Symmetric Difference: {:?}", sym_diff);

    //     let mut relationship = HashMap::new();
    //     relationship.insert("union".to_owned(), union);
    //     relationship.insert("intersection".to_owned(), intersection);
    //     relationship.insert("difference".to_owned(), difference);
    //     relationship.insert("symdif".to_owned(), sym_diff);
    //     relationship


    // }
}






// fn maths (player_a: Player, player_b: Player) {
    // let me: HashSet<Player> = Sect {item : vec![player_a]}.item.iter().cloned().collect();
    // let you: HashSet<Player> = Sect {item : vec![player_b]}.item.iter().cloned().collect();

    // // Union
    // let union: HashSet<_> = me.union(&you).cloned().collect();
    // println!("Union: {:?}", union);

    // // Intersection
    // let intersection: HashSet<_> = me.intersection(&you).cloned().collect();
    // println!("Intersection: {:?}", intersection);

    // // Difference
    // let difference: HashSet<_> = me.difference(&you).cloned().collect();
    // println!("Difference: {:?}", difference);

    // // Symmetric Difference
    // let sym_diff: HashSet<_> = me.symmetric_difference(&you).cloned().collect();
    // println!("Symmetric Difference: {:?}", sym_diff);
//}














// Players play in turns
// Players can play cards to the table
// if a player has no valid card to play, they must pick up a card from the board

// The board divvies cards to the players and one to the table at the start of the game
// If a power card like 2 and 4 are played, then the board will give the next player the associated number of cards

// The table validates a play and announces each players turn

// Select the next player

// impl FromIterator<Player> for Player {
//     fn from_iter<T: IntoIterator<Item = Player>>(iter: T) -> Self {
//         let mut player: Vec<Player> = iter.into_iter().collect();
//         player.sort();
//         Player { name: player[0].name, cards: player[0].cards }
//     }
// }

// impl FromIterator<&Player> for &Player  {
//     fn from_iter<T: IntoIterator<Item = &Player>>(iter: T) -> Self {
//         let mut player: Vec<Player> = iter.into_iter().collect();
//         player.sort();
//         &Player { name: player[0].name, cards: player[0].cards }
//     }
// }