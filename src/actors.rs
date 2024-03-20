use crate::lib::{Card, Deck, Shuffled};

#[derive(Debug)]
pub struct Board {
    cards: Vec<Card>
}

#[derive(Debug)]
pub struct Table {
    cards: Vec<Card>
}

#[derive(Debug)]
pub struct Player {
    name: String,
    cards: Vec<Card>
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