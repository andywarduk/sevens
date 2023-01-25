use crate::{
    cards::{Card, CardCollection, Deck},
    game::SEVEN_HEARTS,
};

#[derive(Debug, Clone)]
pub struct State {
    pub board: CardCollection,
    pub plays: CardCollection,
    pub players: Vec<CardCollection>,
    cur_player: usize,
    pub depth: usize,
}

impl State {
    pub fn new(player_count: u8, mut deck: Deck) -> Self {
        // Create the players
        let mut players = (0..player_count)
            .map(|_| CardCollection::new())
            .collect::<Vec<_>>();

        // Deal the cards
        let mut deal_player = 0;
        let mut first_player = None;

        while let Some(card) = deck.pop() {
            if card == *SEVEN_HEARTS {
                first_player = Some(deal_player);
            }

            players[deal_player as usize].add(card);

            deal_player = (deal_player + 1) % player_count;
        }

        // Create state
        Self {
            board: CardCollection::new(),
            plays: CardCollection::new_single(SEVEN_HEARTS.clone()),
            players,
            cur_player: first_player.expect("Player with 7♥ not found") as usize,
            depth: 0,
        }
    }

    #[inline]
    pub fn cur_player(&self) -> usize {
        self.cur_player
    }

    pub fn next_player(&mut self) {
        // Move to next player
        self.cur_player = (self.cur_player + 1) % self.players.len();
    }

    pub fn play_card(&mut self, card: Card) {
        if card == *SEVEN_HEARTS {
            self.plays.add(Card::new(&'♣', "7").unwrap());
            self.plays.add(Card::new(&'♦', "7").unwrap());
            self.plays.add(Card::new(&'♠', "7").unwrap());
        }

        self.plays.add(card.lower());
        self.plays.add(card.higher());

        self.players[self.cur_player].remove(card.clone());

        self.board.add(card);
    }
}
