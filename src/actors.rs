use std::collections::HashSet;
use crate::lib::{Card, Deck, Rank, Shuffled, Suit};

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
        let first_card = vec![deck_vec.pop().unwrap()];
        (
            Board { cards: deck_vec },
            Table { cards: first_card },
            player_vec,
        )
    }

    pub fn shuffle_board(&mut self, table: &mut Table) {
        //let top_card = table.cards.pop().unwrap();
        // drain card
        let to_shuffle: HashSet<Card> = table.cards.drain(..table.cards.len()).collect();

        for card in to_shuffle {
            self.cards.push(card)
        }
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
                        .unwrap(),
                ),
            );
            if card.rank == Rank::Jack {
                self.cards.push(Card {
                    rank: card.rank,
                    suit: card.rank.change_to().unwrap(),
                })
            }
        } else {
            println!(
                "{:#?} you can't play {:#?} on {:#?}",
                player.name,
                card.name(),
                self.cards.get(self.cards.len() - 1).unwrap().name()
            );
            println!(
                "{:#?} is getting {:#?} from the board",
                player.name,
                board.cards.get(board.cards.len() - 1).unwrap().name()
            );
            give_card(1, player, board)
        }
    }

    fn playable(&mut self, card: Option<(&Rank, &Suit)>) -> bool {
        let incoming_suit = card.unwrap().1;
        let incoming_rank = card.unwrap().0;
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
                return true;
            } else if black_set.contains(incoming_suit) && black_set.contains(&card_on_deck.suit) {
                return true;
            } else {
                return false;
            }
        }
    }
}

pub fn give_card(amount: usize, player: &mut Player, board: &mut Board) {
    player.cards.push(board.cards.drain(..amount).collect())
}
