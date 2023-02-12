use std::cmp::Ordering;

use futures::future::{BoxFuture, FutureExt};
use tokio::task::JoinSet;

use super::{state::State, Results, Strategy};

pub fn play(mut state: State, strategy: Strategy) -> BoxFuture<'static, Results> {
    async move {
        let mut results = Results::new(state.player_count() as u8);
        let mut join_set = JoinSet::new();

        #[cfg(feature = "trace")]
        println!("-- Start --");

        'outer: loop {
            'inner: {
                #[cfg(feature = "trace")]
                println!("Board: {}", state.board);

                // Calculate playable cards
                let (playable_cards, no_consequence_cards, sequence_cards) = state.playable_cards();

                let card_set = strategy.choose_cards(
                    no_consequence_cards,
                    sequence_cards,
                    playable_cards,
                    #[cfg(not(feature = "nostats"))]
                    state.cur_player(),
                    #[cfg(not(feature = "nostats"))]
                    &mut results,
                );

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

                        if state.cards_played() < 18 && !cfg!(feature = "trace") {
                            // Thread
                            for c in card_iter {
                                let mut next_state = state.clone();

                                join_set.spawn(async move {
                                    // Play the card
                                    next_state.play_card(c);

                                    // Move to next player
                                    next_state.next_player();

                                    // Play on
                                    play(next_state, strategy).await
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
                                let result = play(next_state, strategy).await;

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

                if state.player_cards()[state.cur_player()].is_empty() {
                    // Player has won
                    results.win_for(state.cur_player());

                    #[cfg(feature = "trace")]
                    println!("Win for player {}", state.cur_player() + 1);

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
