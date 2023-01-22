use crate::cards::CardCollection;

use super::{state::State, Results};

pub fn play(debug: bool, mut state: State, results: &mut Results) {
    if debug {
        println!("-- Depth {}", state.depth);
    }

    loop {
        if debug {
            println!("Board: {}", state.board);
        }

        let cards = &state.players[state.cur_player()].cards;

        if debug {
            println!("Player {} cards: {}", state.cur_player(), cards);
        }

        let mut playable_cards = cards.iter().fold(CardCollection::new(), |mut cards, c| {
            if state.card_plays(c.clone()) {
                cards.add(c);
            }

            cards
        });

        if debug {
            println!(
                "Player {} all playable cards: {}",
                state.cur_player(),
                playable_cards
            );
        }

        if playable_cards.len() > 0 {
            // Look for a card we can play with no consequences
            if let Some(no_consequence_card) = playable_cards.iter().find(|c| {
                let rank_elem = c.rank_elem();

                if rank_elem == 0 || rank_elem == 12 {
                    // Ace or King can be played with no consequence
                    true
                } else if rank_elem < 6 {
                    // Less than 7 - if we have the card one lower as well, play this one
                    if cards.contains(c.lower()) {
                        true
                    } else {
                        false
                    }
                } else if rank_elem > 6 {
                    // More than 7 - if we have the card one higher as well, play this one
                    if cards.contains(c.higher()) {
                        true
                    } else {
                        false
                    }
                } else {
                    // A 7. Play if we have the higher and lower card
                    if cards.contains(c.lower()) && cards.contains(c.higher()) {
                        true
                    } else {
                        false
                    }
                }
            }) {
                playable_cards = CardCollection::new_single(no_consequence_card);

                if debug {
                    println!(
                        "Player {} playing no consequence card: {}",
                        state.cur_player(),
                        playable_cards
                    );
                }
            }
        }

        // Make a move if possible
        match playable_cards.len() {
            0 => {
                // No cards to play
            }
            1 => {
                // One card to play
                let card = playable_cards.iter().next().unwrap();

                if debug {
                    println!("Player {} playing {}", state.cur_player(), card);
                }

                state.play_card(card);

                if state.players[state.cur_player()].cards.is_empty() {
                    // Player has won
                    results.win_for(state.cur_player());

                    if debug {
                        println!("Win for player {}", state.cur_player());

                        if (results.games() % 100_000) == 0 {
                            println!("{:?}", results);
                        }
                    }

                    break;
                }
            }
            _ => {
                // Multiple choices
                for c in playable_cards.iter() {
                    let mut next_state = state.clone();

                    if debug {
                        println!("Player {} playing {} with backtrack", state.cur_player(), c);
                    }

                    next_state.play_card(c);

                    // Move to next player
                    next_state.next_player();

                    next_state.depth += 1;

                    play(debug, next_state, results);
                }

                break;
            }
        }

        // Move to next player
        state.next_player();
    }

    if debug {
        println!("-----------");
    }
}
