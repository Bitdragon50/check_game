use crate::lib::{Card, Deck, Shuffled};

#[derive(Debug)]
pub struct Board {
    cards: Vec<Card>
}

#[derive(Debug)]
pub struct Table {
    cards: Vec<Card>
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

pub fn play(player: &mut Player,table: &mut Table, card_position: usize) {
    table.cards.push(player.cards.remove(card_position));
}

pub fn take_card(board: &mut Board, player: &mut Player) {
    player.cards.push(board.cards.pop().unwrap())
}

pub fn give_card(amount: usize, player: &mut Player, board: &mut Board) {
    player.cards.push(board.cards.drain(..amount).collect())
}