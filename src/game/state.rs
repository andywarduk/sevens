use std::cmp::Ordering;

use crate::{
    cards::{Card, CardCollection, Deck},
    game::SEVEN_HEARTS,
};

#[derive(Debug, Clone)]
/// Game state
pub struct State {
    /// The current board state
    board: CardCollection,
    /// The current valid moves
    valid_moves: CardCollection,
    /// Current player
    cur_player: usize,
    /// Player cards
    player_cards: Vec<CardCollection>,
    #[cfg(not(feature = "nostats"))]
    misses: usize,
}

impl State {
    /// Creates a new game state
    pub fn new(player_count: u8, mut deck: Deck) -> Self {
        // Create state
        let mut state = Self {
            board: CardCollection::new(),
            valid_moves: CardCollection::new_single(SEVEN_HEARTS.clone()),
            player_cards: (0..player_count)
                .map(|_| CardCollection::new())
                .collect::<Vec<_>>(),
            cur_player: 0,
            #[cfg(not(feature = "nostats"))]
            misses: 0,
        };

        // Deal the cards
        let mut first_player = None;

        while let Some(card) = deck.pop() {
            if card == SEVEN_HEARTS {
                first_player = Some(state.cur_player);
            }

            state.player_cards[state.cur_player].add(card);

            state.next_player();
        }

        // Create state
        state.cur_player = first_player.expect("Player with 7♥ not found");

        state
    }

    #[inline]
    #[cfg(feature = "trace")]
    /// Returns the current board
    pub fn board(&self) -> &CardCollection {
        &self.board
    }

    #[inline]
    /// Returns the number of players
    pub fn player_count(&self) -> usize {
        self.player_cards.len()
    }

    #[inline]
    /// Returns the current player
    pub fn cur_player(&self) -> usize {
        self.cur_player
    }

    /// Returns player cards
    #[inline]
    pub fn all_player_cards(&self) -> &Vec<CardCollection> {
        &self.player_cards
    }

    /// Returns current player's cards
    #[inline]
    pub fn cur_player_cards(&self) -> &CardCollection {
        &self.player_cards[self.cur_player]
    }

    /// Move to the next player
    #[inline]
    pub fn next_player(&mut self) {
        // Move to next player
        let mut new_player = self.cur_player + 1;

        if new_player == self.player_cards.len() {
            new_player = 0;
        }

        self.cur_player = new_player;
    }

    /// Play a card
    #[inline]
    pub fn play_card(&mut self, card: Card) {
        if card == SEVEN_HEARTS {
            // Other sevens can now be played
            self.valid_moves.add(Card::new(&'♣', "7").unwrap());
            self.valid_moves.add(Card::new(&'♦', "7").unwrap());
            self.valid_moves.add(Card::new(&'♠', "7").unwrap());
        }

        // NB! This adds invalid moves when an ace or king is played but the bits set aren't used
        self.valid_moves.add(card.one_lower());
        self.valid_moves.add(card.one_higher());

        // Remove the card from the player's hand
        self.player_cards[self.cur_player].remove(card.clone());

        // Add the card to the board
        self.board.add(card);
    }

    #[inline]
    /// Returns the number of cards played
    pub fn cards_played(&self) -> usize {
        self.board.len()
    }

    #[inline]
    /// Returns playable cards
    pub fn playable_cards(&self) -> (CardCollection, CardCollection, CardCollection) {
        // Get player's cards
        let cards = &self.player_cards[self.cur_player()];

        #[cfg(feature = "trace")]
        println!("Player {} cards: {}", self.cur_player() + 1, cards);

        // Calculate playable cards
        let playable_cards = CardCollection::new_from_raw(cards.raw() & self.valid_moves.raw());
        let mut no_consequence_cards = CardCollection::new();
        let mut sequence_cards = CardCollection::new();

        playable_cards.card_iterator().for_each(|c| {
            let rank_elem = c.rank_elem();
            let suit_elem = c.suit_elem();

            match rank_elem.cmp(&6) {
                Ordering::Less => {
                    // Less than 7 - if we have the card one lower as well, or it's an Ace, play this one
                    if rank_elem == 0 || cards.contains(c.one_lower()) {
                        no_consequence_cards.add(c);
                    } else if cards.contains_one_to_six_except(suit_elem, &c) {
                        sequence_cards.add(c);
                    }
                }
                Ordering::Greater => {
                    // More than 7 - if we have the card one higher as well, or it's a King, play this one
                    if rank_elem == 12 || cards.contains(c.one_higher()) {
                        no_consequence_cards.add(c);
                    } else if cards.contains_eight_to_king_except(suit_elem, &c) {
                        sequence_cards.add(c);
                    }
                }
                Ordering::Equal => {
                    // A 7. Play if we have the higher and lower card
                    if cards.contains(c.one_lower()) && cards.contains(c.one_higher()) {
                        no_consequence_cards.add(c);
                    } else if cards.contains_one_to_six_except(suit_elem, &c)
                        || cards.contains_eight_to_king_except(suit_elem, &c)
                    {
                        sequence_cards.add(c);
                    }
                }
            }
        });

        #[cfg(feature = "trace")]
        {
            println!(
                "Player {}: {} total playable: {}",
                self.cur_player() + 1,
                playable_cards.len(),
                playable_cards
            );

            if !no_consequence_cards.is_empty() {
                println!(
                    "          {} no consequence: {}",
                    no_consequence_cards.len(),
                    no_consequence_cards
                );
            }

            if !sequence_cards.is_empty() {
                println!(
                    "          {} in sequence: {}",
                    sequence_cards.len(),
                    sequence_cards
                );
            }
        }

        (playable_cards, no_consequence_cards, sequence_cards)
    }

    #[cfg(not(feature = "nostats"))]
    #[inline]
    pub fn add_miss(&mut self) {
        self.misses += 1;
    }

    #[cfg(not(feature = "nostats"))]
    #[inline]
    pub fn get_misses(&self) -> usize {
        self.misses
    }
}
