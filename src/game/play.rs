use futures::future::{BoxFuture, FutureExt};
use tokio::task::JoinSet;

use crate::cards::CardCollection;

use super::{state::State, Results};

pub fn play(mut state: State) -> BoxFuture<'static, Results> {
    async move {
        let mut results = Results::new(state.players.len() as u8);

        #[cfg(feature = "trace")]
        println!("-- Depth {}", state.depth);

        loop {
            #[cfg(feature = "trace")]
            println!("Board: {}", state.board);

            let cards = &state.players[state.cur_player()];

            #[cfg(feature = "trace")]
            println!("Player {} cards: {}", state.cur_player(), cards);

            let mut playable_cards = CardCollection::new_from_raw(cards.raw() & state.plays.raw());
            let mut playable_len = playable_cards.len();

            #[cfg(feature = "trace")]
            println!(
                "Player {} has {} playable cards: {}",
                state.cur_player(),
                playable_len,
                playable_cards
            );

            if playable_len > 1 {
                // Look for a card we can play with no consequences
                if let Some(no_consequence_card) = playable_cards.iter().find(|c| {
                    let rank_elem = c.rank_elem();

                    if rank_elem == 0 || rank_elem == 12 {
                        // Ace or King can be played with no consequence
                        true
                    } else if rank_elem < 6 {
                        // Less than 7 - if we have the card one lower as well, play this one
                        cards.contains(c.lower())
                    } else if rank_elem > 6 {
                        // More than 7 - if we have the card one higher as well, play this one
                        cards.contains(c.higher())
                    } else {
                        // A 7. Play if we have the higher and lower card
                        cards.contains(c.lower()) && cards.contains(c.higher())
                    }
                }) {
                    playable_cards.set(no_consequence_card);
                    playable_len = 1;

                    #[cfg(feature = "trace")]
                    println!(
                        "Player {} playing no consequence card: {}",
                        state.cur_player(),
                        playable_cards
                    );
                }
            }

            // Make a move if possible
            match playable_len {
                0 => {
                    // No cards to play
                }
                1 => {
                    // One card to play
                    let card = playable_cards.iter().next().unwrap();

                    #[cfg(feature = "trace")]
                    println!("Player {} playing {}", state.cur_player(), card);

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

                        break;
                    }
                }
                _ => {
                    // Multiple choices
                    if state.depth < 8 {
                        let mut join_set = JoinSet::new();

                        for c in playable_cards.iter() {
                            let mut next_state = state.clone();

                            #[cfg(feature = "trace")]
                            println!("Player {} playing {} with backtrack", state.cur_player(), c);

                            join_set.spawn(async move {
                                next_state.play_card(c);

                                // Move to next player
                                next_state.next_player();

                                next_state.depth += 1;

                                play(next_state).await
                            });
                        }

                        while let Some(res) = join_set.join_next().await {
                            let result = res.unwrap();
                            results.add(result);
                        }
                    } else {
                        for c in playable_cards.iter() {
                            let mut next_state = state.clone();

                            #[cfg(feature = "trace")]
                            println!("Player {} playing {} with backtrack", state.cur_player(), c);

                            next_state.play_card(c);

                            // Move to next player
                            next_state.next_player();

                            next_state.depth += 1;

                            let result = play(next_state).await;

                            results.add(result);
                        }
                    }

                    break;
                }
            }

            // Move to next player
            state.next_player();
        }

        #[cfg(feature = "trace")]
        println!("-----------");

        results
    }
    .boxed()
}
