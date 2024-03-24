use std::io;

use crate::lib::{Card, Deck, Shuffled};

#[derive(Debug)]
pub struct Board {
    cards: Vec<Card>
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
        pub fn play(&mut self, player: &mut Player) {
            let mut card_position_input = String::new();
            io::stdin().read_line(&mut card_position_input)
            .expect("Failed to read from stdin");
    
            let card_position: u8 = card_position_input.trim().parse()
            .expect("Please type a number!");

            self.cards.push(player.cards.remove(card_position as usize));
        }
}

pub fn take_card(board: &mut Board, player: &mut Player) {
    player.cards.push(board.cards.pop().unwrap())
}

pub fn give_card(amount: usize, player: &mut Player, board: &mut Board) {
    player.cards.push(board.cards.drain(..amount).collect())
}