use std::io;

use crate::lib::{Card, Deck, Rank, Shuffled, Suit};

#[derive(Debug)]
pub struct Board {
    pub cards: Vec<Card>
}

#[derive(Debug)]
pub struct Table {
    pub cards: Vec<Card>
}

#[derive(Debug,Clone)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>
}



impl Board {
    pub fn new(deck: Deck<Shuffled>, players: Vec<String>) -> (Board, Table, Vec<Player>) {
        let mut deck_vec = deck.to_vec();
        let mut player_vec = vec![];

        for player in players {
            let players_card = deck_vec.drain(..4).collect();
            player_vec.push(Player{name: player , cards: players_card})
        }
        let first_card = vec![deck_vec.pop().unwrap()];
        (Board{cards: deck_vec} ,Table{cards: first_card } ,player_vec)
    }
}

impl Table {
        pub fn play(&mut self, player: &mut Player, board: &mut Board, card: &mut Card) {
            println!("Player {:#?} plays {:#?}",&player.name, card.name());
         
            if self.playable(Some((&card.rank,&card.suit))){ 
                println!("Player {:#?} plays {:#?}",&player, card);
                self.cards.push(player.cards.remove(player.cards.iter().position(|iter_card| iter_card == card).unwrap()));
                if card.rank == Rank::Jack { } 
            } else {
                println!("{:#?} you can't play {:#?} on {:#?}", player.name, card.name(), self.cards.get(self.cards.len()-1).unwrap().name());
                println!("{:#?} is getting {:#?} from the board", player.name, board.cards.get(board.cards.len()-1).unwrap().name() );
                give_card(1, player, board)
            }
        }

        fn playable(&mut self, card: Option<(&Rank,&Suit)>) -> bool {
            // if card.rank != Rank::Jack {
                &self.cards[self.cards.len()-1].rank == card.unwrap().0 || &self.cards[self.cards.len()-1].suit == card.unwrap().1
            // } else {
            //     card.rank.change_to()
            // }
        }
}

pub fn take_card(board: &mut Board, player: &mut Player) {
            player.cards.push(board.cards.pop().unwrap())
        }

pub fn give_card(amount: usize, player: &mut Player, board: &mut Board) {
    player.cards.push(board.cards.drain(..amount).collect())
}