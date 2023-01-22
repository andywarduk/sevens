use crate::{
    cards::{Card, CardCollection, Deck},
    play::SEVEN_HEARTS,
};

use super::player::Player;

#[derive(Debug, Clone)]
pub struct State {
    pub board: CardCollection,
    pub players: Vec<Player>,
    cur_player: usize,
    pub depth: usize,
}

impl State {
    pub fn new(player_count: u8, mut deck: Deck) -> Self {
        // Create the players
        let mut players = (0..player_count).map(|_| Player::new()).collect::<Vec<_>>();

        // Deal the cards
        let mut deal_player = 0;
        let mut first_player = None;

        while let Some(card) = deck.pop() {
            if card == *SEVEN_HEARTS {
                first_player = Some(deal_player);
            }

            players[deal_player as usize].cards.add(card);

            deal_player = (deal_player + 1) % player_count;
        }

        // Create state
        let state = State {
            board: CardCollection::new(),
            players,
            cur_player: first_player.expect("Player with 7♥ not found") as usize,
            depth: 0,
        };

        state
    }

    #[inline]
    pub fn cur_player(&self) -> usize {
        self.cur_player
    }

    pub fn next_player(&mut self) {
        // Move to next player
        self.cur_player = (self.cur_player + 1) % self.players.len();
    }

    pub fn card_plays(&self, card: Card) -> bool {
        let rank = card.rank_elem();

        if rank == 6 {
            // '7'
            if card.suit() == '♥' {
                true
            } else {
                // Can only play a 7 if 7♥ played
                if !self.board.contains(SEVEN_HEARTS.clone()) {
                    false
                } else {
                    true
                }
            }
        } else if rank < 6 {
            if self.board.contains(card.higher()) {
                true
            } else {
                false
            }
        } else {
            if self.board.contains(card.lower()) {
                true
            } else {
                false
            }
        }
    }

    pub fn play_card(&mut self, card: Card) {
        self.players[self.cur_player].cards.remove(card.clone());
        self.board.add(card);
    }
}
