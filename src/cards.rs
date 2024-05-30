use crate::deck::{Card, Deck, Rank, Shuffled, Suit};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Board {
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub struct Table {
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
}

impl Board {
    pub fn new(deck: Deck<Shuffled>, players: Vec<String>) -> (Board, Table, Vec<Player>) {
        let mut deck_vec = deck.to_vec();
        let mut player_vec = vec![];

        for player in players {
            let players_card = deck_vec.drain(..4).collect();
            player_vec.push(Player {
                name: player,
                cards: players_card,
            })
        }
        let first_card = vec![deck_vec.pop().expect("The deck is empty")];
        (
            Board { cards: deck_vec },
            Table { cards: first_card },
            player_vec,
        )
    }

    pub fn shuffle_board(&mut self, table: &mut Table) {
        let to_shuffle: HashSet<Card> = table.cards.drain(..table.cards.len()).collect();
        to_shuffle.into_iter().for_each(|card| self.cards.push(card))
    }
}

impl Table {
    pub fn play(&mut self, player: &mut Player, board: &mut Board, card: &mut Card) {
        println!("Player {:#?} plays {:#?}", &player.name, card.name());

        if self.playable(Some((&card.rank, &card.suit))) {
            println!("Player {:#?} plays {:#?}", &player, card);
            self.cards.push(
                player.cards.remove(
                    player
                        .cards
                        .iter()
                        .position(|iter_card| iter_card == card)
                        .expect("The card the player is trying to play doesn't exist in their hand"),
                ),
            );
            if card.rank == Rank::Jack {
                self.cards.push(Card {
                    rank: card.rank,
                    suit: {
                        match card.rank.change_to() {
                            Ok(suit) => suit,
                            Err(err) => {
                                println!("We got an error, did you mispell? {}", err);
                                card.suit
                            }
                        }
                    },
                })
            }
        } else {
            println!(
                "{:#?} you can't play {:#?} on {:#?}",
                player.name,
                card.name(),
                self.cards.last().expect("The table has no top card at the moment").name()
            );
            println!(
                "{:#?} is getting {:#?} from the board",
                player.name,
                board.cards.last().expect("The board is out of cards").name()
            );
            give_card(1, player, board)
        }
    }

    fn playable(&mut self, card: Option<(&Rank, &Suit)>) -> bool {
        let incoming_suit = card.expect("No card received").1;
        let incoming_rank = card.expect("No card received").0;
        let card_on_deck = self.cards[self.cards.len() - 1];

        if incoming_rank != &Rank::Joker && card_on_deck.rank != Rank::Joker {
            //if incoming card isn't a joker
            &card_on_deck.rank == incoming_rank || &card_on_deck.suit == incoming_suit
        } else {
            let mut red_set = HashSet::new();
            let mut black_set = HashSet::new();
            red_set.insert(Suit::Diamonds);
            red_set.insert(Suit::Red);
            red_set.insert(Suit::Hearts);
            black_set.insert(Suit::Spades);
            black_set.insert(Suit::Black);
            black_set.insert(Suit::Clubs);
            // red jokers play on diamonds and heart
            if red_set.contains(incoming_suit) && red_set.contains(&card_on_deck.suit) {
                true
            } else {
                black_set.contains(incoming_suit) && black_set.contains(&card_on_deck.suit)
            }
        }
    }
}

pub fn give_card(amount: usize, player: &mut Player, board: &mut Board) {
    println!("{:#?} I'm giving you a card", player.name);
    println!(
        "{:#?} is getting {:#?} from the board",
        player.name,
        board.cards.last().expect("The board is out of cards").name()
    );
    player.cards.push(board.cards.drain(..amount).collect())
}
