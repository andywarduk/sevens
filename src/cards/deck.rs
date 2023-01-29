use std::collections::{vec_deque::Iter, VecDeque};

use rand::Rng;

use super::{Card, CARD_HASH, RANKS, SUITS};

#[derive(Debug)]
pub struct Deck(VecDeque<Card>);

impl Deck {
    pub fn new() -> Self {
        Self((0..SUITS.len()).fold(VecDeque::new(), |cards, s| {
            (0..RANKS.len()).fold(cards, |mut cards, r| {
                cards.push_back(Card::new_from_elems(s, r));
                cards
            })
        }))
    }

    pub fn new_from_hash(hash: &str) -> Option<Self> {
        let mut touched = [false; 52];

        if let Some(cards) = hash
            .chars()
            .map(|c| match CARD_HASH.iter().position(|h| *h as char == c) {
                None => None,
                Some(p) => {
                    touched[p] = true;
                    Card::new_from_hash(c as u8)
                }
            })
            .collect()
        {
            if touched.into_iter().any(|t| !t) {
                // Not all cards specified
                println!("HERE1");
                None
            } else {
                Some(Self(cards))
            }
        } else {
            // Invalid character
            println!("HERE2");
            None
        }
    }

    pub fn shuffle(&mut self) {
        let mut shuffled = VecDeque::new();

        let mut rng = rand::thread_rng();

        while !self.0.is_empty() {
            let elem = rng.gen_range(0..self.0.len());
            let card = self.0.remove(elem).unwrap();
            shuffled.push_back(card);
        }

        self.0 = shuffled;
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop_front()
    }

    pub fn hash_string(&self) -> String {
        self.0.iter().map(|c| c.hash_val()).collect()
    }

    pub fn iter(&self) -> Iter<Card> {
        self.0.iter()
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for (i, c) in self.0.iter().enumerate() {
            if i > 0 {
                string += " ";
            }

            string += &format!("{c}");
        }

        f.write_str(&string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Build deck
        let mut deck = Deck::new();

        // Build expected deck
        let mut expected = Vec::new();

        for i in 0..4 {
            for j in 0..13 {
                expected.push(Card::new_from_raw((1 << j) << (i * 16)));
            }
        }

        // Check equal
        assert_eq!(deck.0, expected);

        // Shuffle deck
        deck.shuffle();

        // Make sure we've got all of the cards
        for c in deck.0 {
            let index = expected.iter().position(|e| *e == c).unwrap();
            expected.swap_remove(index);
        }

        assert!(expected.is_empty());
    }
}
