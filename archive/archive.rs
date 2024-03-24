use actix::prelude::*;
use actix::Handler;
use check_game::Rank;
use crate::lib::Card;
use actix::Context;

pub struct Player {
    name: String,
    cards: Vec<Card>
}

pub struct Board {
    cards: Vec<Card>
}

pub struct Table {
    cards: Vec<Card>,
    turn: Player
}

pub enum Game {
    Player,
    Board,
    Table
}

impl Actor for Game {
    type Context = Context<Self>;
    // fn started (&mut self, _ctx: &mut Self::Context){        
    // }
}

impl Actor for Table {
    type Context = Context<Self>;
    // fn started (&mut self, _ctx: &mut Self::Context){        
    // }
}

impl Actor for Player {
    type Context = Context<Self>;
    // fn started (&mut self, _ctx: &mut Self::Context){        
    // }
}

impl Actor for Board {
    type Context = Context<Self>;
    // fn started (&mut self, _ctx: &mut Self::Context){        
    // }
}

impl Handler<Card> for Table{
    type Result = ();
    fn handle(&mut self, msg: Card, ctx: &mut Self::Context) -> Self::Result {
        // The incoming card is the of the same rank or suit as the top card, accept it
        // if the card is a power card, do something special
        if msg.suit == self.cards[self.cards.len()-1].suit || msg.rank == self.cards[self.cards.len()-1].rank {
            self.cards.push(msg);
            
            match msg.rank {
                _ => "Some",
                Four => println!("Pick four cards");
            }
        }
    }
}

impl Handler<Card> for Board{
    type Result = ();
    
}

impl Handler<Card> for Player{
    type Result = ();

    fn handle(&mut self, msg: Card, ctx: &mut Self::Context) -> Self::Result {
        self.cards = self.cards.push(msg)
    }
    
}
