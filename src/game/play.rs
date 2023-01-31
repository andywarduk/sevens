use std::cmp::Ordering;

use futures::future::{BoxFuture, FutureExt};
use tokio::task::JoinSet;

use crate::cards::CardCollection;

use super::{state::State, Results};

pub fn play(mut state: State) -> BoxFuture<'static, Results> {
    async move {
        let mut results = Results::new(state.players.len() as u8);
        let mut join_set = JoinSet::new();

        #[cfg(feature = "trace")]
        println!("-- Start --");

        'outer: loop {
            'inner: {
                #[cfg(feature = "trace")]
                println!("Board: {}", state.board);

                // Get player's cards
                let cards = &state.players[state.cur_player()];

                #[cfg(feature = "trace")]
                println!("Player {} cards: {}", state.cur_player() + 1, cards);

                // Calculate playable cards
                let playable_cards = CardCollection::new_from_raw(cards.raw() & state.plays.raw());
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
                        state.cur_player() + 1,
                        playable_cards.len(),
                        playable_cards
                    );

                    if no_consequence_cards.len() > 0 {
                        println!(
                            "          {} no consequence: {}",
                            no_consequence_cards.len(),
                            no_consequence_cards
                        );
                    }

                    if sequence_cards.len() > 0 {
                        println!(
                            "          {} in sequence: {}",
                            sequence_cards.len(),
                            sequence_cards
                        );
                    }
                }

                let card_set = if !no_consequence_cards.is_empty() {
                    // Just choose the first no consequence card
                    no_consequence_cards.set_first();
                    &no_consequence_cards
                } else if !sequence_cards.is_empty() {
                    &sequence_cards
                } else {
                    &playable_cards
                };

                // Make a move if possible
                let card = match card_set.len().cmp(&1) {
                    Ordering::Less => {
                        // No cards to play
                        break 'inner;
                    }
                    Ordering::Equal => {
                        // One card to play
                        card_set.card_iterator().next().unwrap()
                    }
                    _ => {
                        // Multiple choices
                        let mut card_iter = card_set.card_iterator();

                        let first_card = card_iter.next().unwrap();

                        if state.board.len() < 18 && !cfg!(feature = "trace") {
                            // Thread
                            for c in card_iter {
                                let mut next_state = state.clone();

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
                                    state.cur_player() + 1,
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
                println!("Player {} playing {}", state.cur_player() + 1, card);

                // Play the card
                state.play_card(card);

                if state.players[state.cur_player()].is_empty() {
                    // Player has won
                    results.win_for(state.cur_player());

                    #[cfg(feature = "trace")]
                    {
                        println!("Win for player {}", state.cur_player() + 1);

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
        println!("-- End --");

        // Join threads
        while let Some(res) = join_set.join_next().await {
            results.add(res.unwrap());
        }

        results
    }
    .boxed()
}
