use clap::ValueEnum;

use crate::cards::CardCollection;

#[cfg(not(feature = "nostats"))]
use super::Results;

#[derive(ValueEnum, Clone, Copy)]
pub enum Strategy {
    /// A single no consequence card is the preferred play, then in-sequence cards
    NoConsequence,
    /// A single no consequence card and in-sequence cards are the preferred play
    Preferred,
    /// All playable cards are considered equal
    Dumb,
}

impl Strategy {
    #[inline]
    pub fn choose_cards(
        &self,
        no_consequence_cards: CardCollection,
        sequence_cards: CardCollection,
        playable_cards: CardCollection,
        #[cfg(not(feature = "nostats"))] player: usize,
        #[cfg(not(feature = "nostats"))] results: &mut Results,
    ) -> CardCollection {
        #[cfg(not(feature = "nostats"))]
        let mut stat_card_set = None;

        #[cfg(not(feature = "nostats"))]
        let mut pref_rank = 0;

        let use_card_set = match self {
            Strategy::NoConsequence => {
                if !no_consequence_cards.is_empty() {
                    // Just choose the first no consequence card
                    let use_card_set = CardCollection::new_from_raw(no_consequence_cards.first());

                    #[cfg(not(feature = "nostats"))]
                    {
                        stat_card_set = Some(no_consequence_cards);
                    }

                    use_card_set
                } else if !sequence_cards.is_empty() {
                    // Sequence cards

                    #[cfg(not(feature = "nostats"))]
                    {
                        pref_rank = 1;
                    }

                    sequence_cards
                } else {
                    // Any playable

                    #[cfg(not(feature = "nostats"))]
                    {
                        pref_rank = 2;
                    }

                    playable_cards
                }
            }
            Strategy::Preferred => {
                if !no_consequence_cards.is_empty() {
                    // Just choose the first no consequence card and add on the sequence cards
                    let use_card_set = CardCollection::new_from_raw(
                        no_consequence_cards.first() | sequence_cards.raw(),
                    );

                    #[cfg(not(feature = "nostats"))]
                    {
                        stat_card_set = Some(CardCollection::new_from_raw(
                            no_consequence_cards.raw() | sequence_cards.raw(),
                        ));
                    }

                    use_card_set
                } else if !sequence_cards.is_empty() {
                    // Sequence cards

                    #[cfg(not(feature = "nostats"))]
                    {
                        pref_rank = 1;
                    }

                    sequence_cards
                } else {
                    // Any playable

                    #[cfg(not(feature = "nostats"))]
                    {
                        pref_rank = 2;
                    }

                    playable_cards
                }
            }
            Strategy::Dumb => playable_cards, // Any playable
        };

        // Update play stats
        #[cfg(not(feature = "nostats"))]
        results.update_stats_for(
            player,
            &stat_card_set.unwrap_or(use_card_set.clone()),
            pref_rank,
        );

        use_card_set
    }

    #[cfg(not(feature = "nostats"))]
    pub fn max_pref_rank(&self) -> u8 {
        match self {
            Strategy::NoConsequence => 2,
            Strategy::Preferred => 2,
            Strategy::Dumb => 0,
        }
    }

    #[cfg(not(feature = "nostats"))]
    pub fn pref_rank_desc(&self, pref_rank: u8) -> &str {
        match self {
            Strategy::NoConsequence => match pref_rank {
                0 => "No Cons",
                1 => "Sequence",
                2 => "Playable",
                _ => "Unknown",
            },
            Strategy::Preferred => match pref_rank {
                0 => "No Cons+Seq",
                1 => "Sequence",
                2 => "Playable",
                _ => "Unknown",
            },
            Strategy::Dumb => match pref_rank {
                0 => "Playable",
                _ => "Unknown",
            },
        }
    }
}
