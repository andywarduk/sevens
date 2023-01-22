use std::collections::VecDeque;

use rand::Rng;

use super::{Card, RANKS, SUITS};

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

    pub fn shuffle(&mut self) {
        let mut shuffled = VecDeque::new();

        let mut rng = rand::thread_rng();

        while self.0.len() > 0 {
            let elem = rng.gen_range(0..self.0.len());
            let card = self.0.remove(elem).unwrap();
            shuffled.push_back(card);
        }

        self.0 = shuffled;
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop_front()
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for (i, c) in self.0.iter().enumerate() {
            if i > 0 {
                string += " ";
            }

            string += &format!("{}", c);
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

        assert!(expected.len() == 0);
    }
}
