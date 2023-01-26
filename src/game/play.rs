use std::cmp::Ordering;

use futures::future::{BoxFuture, FutureExt};
use tokio::task::JoinSet;

use crate::cards::{Card, CardCollection};

use super::{state::State, Results};

pub fn play(mut state: State) -> BoxFuture<'static, Results> {
    async move {
        let mut results = Results::new(state.players.len() as u8);
        let mut join_set = JoinSet::new();

        #[cfg(feature = "trace")]
        println!("-- Depth {}", state.depth);

        'outer: loop {
            'inner: {
                #[cfg(feature = "trace")]
                println!("Board: {}", state.board);

                // Get player's cards
                let cards = &state.players[state.cur_player()];

                #[cfg(feature = "trace")]
                println!("Player {} cards: {}", state.cur_player(), cards);

                // Calculate playable cards
                let mut playable_cards =
                    CardCollection::new_from_raw(cards.raw() & state.plays.raw());
                let mut playable_len = playable_cards.len();

                #[cfg(feature = "trace")]
                println!(
                    "Player {} has {} playable cards: {}",
                    state.cur_player(),
                    playable_len,
                    playable_cards
                );

                if playable_len > 1 {
                    // More than one card to play - Look for a card we can play with no consequences
                    playable_cards
                        .card_iterator()
                        .find(|c| {
                            let rank_elem = c.rank_elem();

                            match rank_elem.cmp(&6) {
                                Ordering::Less => {
                                    // Less than 7 - if we have the card one lower as well, or it's an Ace, play this one
                                    rank_elem == 0 || cards.contains(c.lower())
                                }
                                Ordering::Greater => {
                                    // More than 7 - if we have the card one higher as well, or it's a King, play this one
                                    rank_elem == 12 || cards.contains(c.higher())
                                }
                                Ordering::Equal => {
                                    // A 7. Play if we have the higher and lower card
                                    cards.contains(c.lower()) && cards.contains(c.higher())
                                }
                            }
                        })
                        .and_then(|no_consequence_card| {
                            playable_cards.set(no_consequence_card);
                            playable_len = 1;

                            #[cfg(feature = "trace")]
                            println!(
                                "Player {} playing no consequence card: {}",
                                state.cur_player(),
                                playable_cards
                            );

                            None::<Card>
                        });
                }

                // Make a move if possible
                let card = match playable_len.cmp(&1) {
                    Ordering::Less => {
                        // No cards to play
                        break 'inner;
                    }
                    Ordering::Equal => {
                        // One card to play
                        playable_cards.card_iterator().next().unwrap()
                    }
                    _ => {
                        // Multiple choices
                        let mut card_iter = playable_cards.card_iterator();

                        let first_card = card_iter.next().unwrap();

                        if cards.len() > 8 {
                            // Thread
                            for c in card_iter {
                                let mut next_state = state.clone();

                                #[cfg(feature = "trace")]
                                println!(
                                    "Player {} playing {} with backtrack",
                                    state.cur_player(),
                                    c
                                );

                                join_set.spawn(async move {
                                    // Play the card
                                    next_state.play_card(c);

                                    // Move to next player
                                    next_state.next_player();

                                    // Play on
                                    play(next_state).await
                                });
                            }
                        } else {
                            // Single thread
                            for c in card_iter {
                                let mut next_state = state.clone();

                                #[cfg(feature = "trace")]
                                println!(
                                    "Player {} playing {} with backtrack",
                                    state.cur_player(),
                                    c
                                );

                                // Play the card
                                next_state.play_card(c);

                                // Move to next player
                                next_state.next_player();

                                // Play on
                                let result = play(next_state).await;

                                results.add(result);
                            }
                        }

                        first_card
                    }
                };

                #[cfg(feature = "trace")]
                println!("Player {} playing {}", state.cur_player(), card);

                // Play the card
                state.play_card(card);

                if state.players[state.cur_player()].is_empty() {
                    // Player has won
                    results.win_for(state.cur_player());

                    #[cfg(feature = "trace")]
                    {
                        println!("Win for player {}", state.cur_player());

                        if (results.games() % 100_000) == 0 {
                            println!("{results:?}");
                        }
                    }

                    break 'outer;
                }
            }

            // Move to next player
            state.next_player();
        }

        #[cfg(feature = "trace")]
        println!("-----------");

        // Join threads
        while let Some(res) = join_set.join_next().await {
            results.add(res.unwrap());
        }

        results
    }
    .boxed()
}
